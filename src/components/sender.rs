use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Sender {
    client: Arc<RwLock<Arc<redis::Client>>>,
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
    pub fn create(service_token: &str) -> Sender {
        Sender {
            token: Arc::new(RwLock::new(Arc::new(service_token.to_owned()))),
            client: if let Ok(x) = redis::Client::open("redis://127.0.0.1/") {
                Arc::new(RwLock::new(Arc::new(x)))
            } else {
                panic!("ERROR; server unreachable!")
            },
        }
    }

    // pub fn service(&self) -> Arc<Service> {
    //     self.service.read().unwrap().clone()
    // }
    pub fn token(&self) -> Arc<String> {
        self.token.read().unwrap().clone()
    }
}
