use crate::communication::Antenna;
use crate::communication::Receiver;
use crate::communication::Sender;
use crate::service::Service;
use std::rc::Rc;

pub struct ServiceManager {
    sender: Rc<Sender>,
    receiver: Receiver,
    antenna: Antenna,
    service: Service,
}

impl ServiceManager {
    pub fn new(service: Service) -> ServiceManager {
        let sender = Sender::create(service.clone());
        let sd = Rc::new(sender);
        let recv = Receiver::create(sd.clone());
        let antenna = Antenna::create(sd.clone());

        ServiceManager {
            service,
            sender: sd,
            receiver: recv,
            antenna,
        }
    }
    pub fn sender(&self) -> &Sender {
        &self.sender
    }
    pub fn receiver(&self) -> &Receiver {
        &self.receiver
    }
    pub fn antenna(&self) -> &Antenna {
        &self.antenna
    }
    pub fn service(&self) -> &Service {
        &self.service
    }
}
