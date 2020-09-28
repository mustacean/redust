use crate::communication::{Receiver, Sender};

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
    pub fn new(name: &'t str, host: &'t str) -> Service<'t> {
        Service::<'t> {
            name,
            host,
            sender: None,
            receiver: None,
        }
    }
}
