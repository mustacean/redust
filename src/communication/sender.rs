use crate::rd_tools::IRedisClient;
use crate::service::Service;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sender {
    client: Rc<redis::Client>,
    service: Rc<Service>,
    token: Rc<String>,
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
    pub fn create(service: Service, tk: Option<&str>) -> Sender {
        Sender {
            token: if let Some(e) = tk {
                Rc::new(e.to_owned())
            } else {
                Rc::new(
                    /*format!("{}", uuid::Uuid::new_v4())*/ service.name().to_owned(),
                )
            },
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
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn clone_from_token(&self, tk: &str) -> Sender {
        let sd = Sender::create(self.service().clone(), Some(tk));
        sd
    }
}
