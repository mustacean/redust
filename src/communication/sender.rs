use crate::rd_tools::IRedisClient;
use crate::service::Service;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sender {
    client: Rc<redis::Client>,
    service: Rc<Service>,
}

impl IRedisClient for Sender {
    fn get_client_rc(&self) -> std::rc::Rc<redis::Client> {
        self.client.clone()
    }
    fn get_client(&self) -> &redis::Client {
        &self.client
    }
    fn get_conn(&self) -> redis::Connection {
        self.get_client().get_connection().unwrap()
    }
}

impl Sender {
    pub fn create(service: Service) -> Sender {
        Sender {
            service: Rc::new(service),
            client: if let Ok(x) = redis::Client::open("redis://127.0.0.1/") {
                Rc::new(x)
            } else {
                panic!("ERROR; server unreachable!")
            },
        }
    }
    pub fn service(&self) -> &Service {
        &self.service
    }
}
