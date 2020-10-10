use crate::communication::Antenna;
use crate::communication::Receiver;
use crate::communication::Sender;
use crate::service::Service;

pub struct ServiceManager {
    receiver: Receiver,
    sender: Sender,
    antenna: Antenna,
    service: Service,
}

impl ServiceManager {
    pub fn new(service: Service) -> ServiceManager {
        let sender = Sender::create(service.clone());
        let receiver = Receiver::create(service.clone(), &sender);
        let antenna = Antenna::create(service.clone(), &sender);

        ServiceManager {
            service,
            sender,
            receiver,
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
    pub fn service(&self) -> Service {
        self.service.clone()
    }
}
