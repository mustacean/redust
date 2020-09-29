use crate::communication::{Receiver, Sender};
use crate::rd_tools::IRedisClient;
use crate::service::{Endpoint, Event};

pub struct Service<'t> {
    name: &'t str,
    host: &'t str,
    sender: Option<Box<Sender<'t>>>,
    receiver: Option<Box<Receiver<'t>>>,
    client: redis::Client,
}

impl<'t> Service<'t> {
    // pub fn open(
    //     service_name: &'static str,
    //     master: &Service,
    //     ep_sender: Option<&str>,
    //     ep_receiver: Option<&str>,
    // ) -> Result<Service<'t>, ()> {

    //     Ok(serv)
    // }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_host(&self) -> &str {
        &self.host
    }
    pub fn get_sender(&self) -> &Sender {
        self.sender.as_ref().unwrap()
    }
    pub fn get_receiver(&self) -> &Receiver {
        self.receiver.as_ref().unwrap()
    }
    pub fn new_event(&self, name: &str) -> Event {
        super::event_t::new_event(self.name, name)
    }
    pub fn new_enpoint(&self, name: &str) -> Endpoint {
        super::endpoint_t::new_endpoint(self.name, name)
    }
}

impl<'t> IRedisClient for Service<'t> {
    fn get_client(&self) -> &redis::Client {
        &self.client
    }
    fn get_conn(&self) -> redis::Connection {
        self.get_client().get_connection().unwrap()
    }
}
