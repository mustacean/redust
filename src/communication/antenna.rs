use crate::communication::Sender;
use crate::rd_tools::IRedisClient;
use crate::service::Event;
use crate::service::Service;
use std::rc::Rc;

#[derive(Clone)]
pub struct Antenna {
    client: Rc<redis::Client>,
    service: Service,
}

impl IRedisClient for Antenna {
    fn get_client_rc(&self) -> Rc<redis::Client> {
        self.client.clone()
    }
    fn get_client(&self) -> &redis::Client {
        &self.client
    }
    fn get_conn(&self) -> redis::Connection {
        self.get_client().get_connection().unwrap()
    }
}

impl Antenna {
    pub fn create(service: Service, sender: &Sender) -> Antenna {
        Antenna {
            service,
            client: sender.get_client_rc(),
        }
    }
    pub fn service(&self) -> &Service {
        &self.service
    }
}
impl Antenna {
    pub fn receive_events(&self, action: impl Fn(&Event, &serde_json::Value)) {
        crate::rd_tools::receive(self.get_conn(), self.service().subsc_names(), |x| {
            let result = x.unwrap();
            let ch = result.get_channel::<String>().unwrap();

            let msg = result.get_payload::<String>().unwrap();

            (action)(&Event::from_str(&ch), &serde_json::from_str(&msg).unwrap());
        })
    }
}
