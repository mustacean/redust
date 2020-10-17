use crate::service::Service;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Sender {
    client: Arc<RwLock<Arc<redis::Client>>>,
    service: Arc<RwLock<Arc<Service>>>,
    token: Arc<RwLock<Arc<String>>>,
}

impl Sender {
    pub fn get_client(&self) -> Arc<redis::Client> {
        self.client.read().unwrap().clone()
    }

    pub fn get_conn(&self) -> redis::Connection {
        self.get_client().get_connection().unwrap()
    }
}

impl Sender {
    pub fn create(service: Arc<Service>, tk: Option<&str>) -> Sender {
        Sender {
            token: if let Some(e) = tk {
                Arc::new(RwLock::new(Arc::new(e.to_owned())))
            } else {
                Arc::new(RwLock::new(Arc::new(service.name().to_owned())))
            },
            service: Arc::new(RwLock::new(service)),
            client: if let Ok(x) = redis::Client::open("redis://127.0.0.1/") {
                Arc::new(RwLock::new(Arc::new(x)))
            } else {
                panic!("ERROR; server unreachable!")
            },
        }
    }

    pub fn service(&self) -> Arc<Service> {
        self.service.read().unwrap().clone()
    }
    pub fn token(&self) -> Arc<String> {
        self.token.read().unwrap().clone()
    }

    pub fn clone_from_token(&self, tk: &str) -> Sender {
        let sd = Sender::create(self.service(), Some(tk));
        sd
    }
}
