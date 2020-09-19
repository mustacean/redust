use crate::redust::event::Event;
use crate::redust::iredisclient::IRedisClient;
use crate::redust::service::Service;
use std::rc::Rc;

pub struct ServiceMetaProvider {
    client: redis::Client,
}
impl IRedisClient for ServiceMetaProvider {
    fn get_client(&self) -> &redis::Client {
        &self.client
    }
    fn get_conn(&self) -> redis::Connection {
        self.get_client().get_connection().unwrap()
    }
}

impl ServiceMetaProvider {
    pub fn provide(host: &str) -> Rc<Box<ServiceMetaProvider>> {
        Rc::new(Box::new(ServiceMetaProvider {
            client: redis::Client::open(format!("redis://{}", host)).unwrap(),
        }))
    }
    pub fn get_service(self: Rc<Box<ServiceMetaProvider>>, ser: &str) -> Result<Service, ()> {
        match redis::cmd("get")
            .arg(format!("service.{}", ser).as_str())
            .query::<String>(&mut self.get_conn())
        {
            Ok(host) => Ok(Service::new(
                ser,
                host.as_str(),
                self.clone(),
                self.clone().get_events(ser),
            )),
            _ => Err(()),
        }
    }
    pub fn remove_service(self: Rc<Box<ServiceMetaProvider>>, ser: &str) -> Result<(), ()> {
        if redis::cmd("del")
            .arg(format!("service.{}", ser).as_str())
            .query::<bool>(&mut self.get_conn())
            .unwrap()
            && redis::cmd("srem")
                .arg("service:list")
                .arg(ser)
                .query::<bool>(&mut self.get_conn())
                .unwrap()
        {
            self.clone().remove_events(ser)
        } else {
            Err(())
        }
    }

    pub fn add_service(
        self: Rc<Box<ServiceMetaProvider>>,
        ser: &str,
        host: &str,
        events: Vec<String>,
    ) -> Result<Service, ()> {
        if redis::cmd("set")
            .arg(format!("service.{}", ser).as_str())
            .arg(host)
            .query::<bool>(&mut self.get_conn())
            .unwrap()
            && redis::cmd("sadd")
                .arg("service:list")
                .arg(ser)
                .query::<bool>(&mut self.get_conn())
                .unwrap()
        {
            if let Ok(()) = self.clone().add_events(ser, events) {
                self.get_service(ser)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    fn get_events(self: Rc<Box<ServiceMetaProvider>>, ser: &str) -> Option<Vec<Event>> {
        match redis::cmd("smembers")
            .arg(format!("service.{}:events", ser).as_str())
            .query::<Vec<String>>(&mut self.get_conn())
        {
            Ok(events) => {
                let mut result = Vec::<Event>::new();

                for e in &events {
                    result.push(Event::new(ser, e));
                }

                if result.len() > 0 {
                    Some(result)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn add_events(
        self: Rc<Box<ServiceMetaProvider>>,
        ser: &str,
        events: Vec<String>,
    ) -> Result<(), ()> {
        for e in &events {
            if let Ok(_) = redis::cmd("sadd")
                .arg(format!("service.{}:events", ser).as_str())
                .arg(e)
                .query::<bool>(&mut self.get_conn())
            {
                continue;
            } else {
                return Err(());
            }
        }

        Ok(())
    }

    fn remove_events(self: Rc<Box<ServiceMetaProvider>>, ser: &str) -> Result<(), ()> {
        if let Ok(_) = redis::cmd("del")
            .arg(format!("service.{}:events", ser).as_str())
            .query::<bool>(&mut self.get_conn())
        {
            Ok(())
        } else {
            return Err(());
        }
    }
}
