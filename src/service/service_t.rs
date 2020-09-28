use crate::communication::{Receiver, Sender};
use crate::service::{Endpoint, Event};

pub struct Service<'t> {
    name: &'t str,
    host: &'t str,
    sender: Option<Box<Sender<'t>>>,
    receiver: Option<Box<Receiver<'t>>>,
}

impl<'t> Service<'t> {
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

    pub fn new(_name: &'t str, _host: &'t str) -> Result<Service<'t>, &'t str> {
        Err("not implemented!")
    }

    pub fn new_event(&self, name: &str) -> Event {
        super::event_t::new_event(self.name, name)
    }
    pub fn new_enpoint(&self, name: &str) -> Endpoint {
        super::endpoint_t::new_endpoint(self.name, name)
    }
}
