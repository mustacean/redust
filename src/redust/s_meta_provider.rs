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

// service.{service_name}:events -> [event_names]

// event.{event_name} -> {service_name}

// event.{event_name}:subs -> [service_names]

impl ServiceMetaProvider {
    pub fn provide() -> Rc<Box<ServiceMetaProvider>> {
        Rc::new(Box::new(ServiceMetaProvider {
            client: redis::Client::open("redis://127.0.0.1".to_owned()).unwrap(),
        }))
    }
    pub fn get_service(self: Rc<Box<ServiceMetaProvider>>, ser: &str) -> Result<Service, ()> {
        match redis::cmd("get")
            .arg(format!("service.{}", ser).as_str())
            .query::<String>(&mut self.get_conn())
        {
            Ok(x) => Ok(Service::new(x, self)),
            _ => Err(()),
        }
    }
    pub fn _remove_service(self: Rc<Box<ServiceMetaProvider>>, _ser: String) {
        todo!()
    }

    pub fn _add_service(self: Rc<Box<ServiceMetaProvider>>, _ser: Service) -> Result<Service, ()> {
        todo!()
    }
}
