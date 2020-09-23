use crate::redust::rd_tools::{redis_commands::*, redis_exec::*, IRedisClient};
use crate::redust::service::{Event, Service};
use redis::{Client, Connection};
use std::rc::Rc;

pub struct ServiceMetaProvider {
    client: redis::Client,
}
impl IRedisClient for ServiceMetaProvider {
    fn get_client(&self) -> &Client {
        &self.client
    }
    fn get_conn(&self) -> Connection {
        self.get_client().get_connection().unwrap()
    }
}

impl ServiceMetaProvider {
    pub fn provide(host: &str) -> Rc<Box<ServiceMetaProvider>> {
        Rc::new(Box::new(ServiceMetaProvider {
            client: Client::open(format!("redis://{}", host)).unwrap(),
        }))
    }

    pub fn get_services(self: Rc<Box<ServiceMetaProvider>>) -> Result<Vec<Service>, ()> {
        let service_names = quest::<Vec<String>>(cmd_fetch_service_names(), &mut self.get_conn())?;
        use std::iter::*;

        Ok(Vec::from_iter(
            service_names
                .iter()
                .map(|e| self.clone().get_service(e).unwrap()),
        ))
    }

    pub fn get_service(self: Rc<Box<ServiceMetaProvider>>, ser: &str) -> Result<Service, ()> {
        let host = quest::<String>(cmd_fetch_service_host(ser), &mut self.get_conn())?;
        Ok(Service::new(
            ser,
            host.as_str(),
            self.clone(),
            self.clone().get_events(ser),
        ))
    }

    pub fn remove_service(self: Rc<Box<ServiceMetaProvider>>, ser: &str) -> Result<(), ()> {
        exec(cmd_del_service_host(ser), &mut self.get_conn())?;
        exec(cmd_rem_from_service_list(ser), &mut self.get_conn())?;
        self.clone().remove_events(ser)
    }

    pub fn add_service(
        self: Rc<Box<ServiceMetaProvider>>,
        ser: &str,
        host: &str,
        events: Vec<String>,
    ) -> Result<(), ()> {
        pred(cmd_set_service_host(ser, host), &mut self.get_conn())?;
        pred(cmd_add_to_service_list(ser), &mut self.get_conn())?;
        self.clone().add_events(ser, events)
    }

    pub fn get_events(self: Rc<Box<ServiceMetaProvider>>, ser: &str) -> Option<Vec<Event>> {
        if let Ok(events) = quest::<Vec<String>>(cmd_fetch_events(ser), &mut self.get_conn()) {
            use std::iter::*;
            Some(Vec::from_iter(events.iter().map(|e| Event::new(ser, e))))
        } else {
            None
        }
    }

    pub fn add_events(
        self: Rc<Box<ServiceMetaProvider>>,
        ser: &str,
        events: Vec<String>,
    ) -> Result<(), ()> {
        for e in &events {
            exec(cmd_add_events(ser, e), &mut self.get_conn())?;
        }
        Ok(())
    }

    pub fn remove_events(self: Rc<Box<ServiceMetaProvider>>, ser: &str) -> Result<(), ()> {
        exec(cmd_del_events(ser), &mut self.get_conn())?;
        Ok(())
    }
}
