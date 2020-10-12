use crate::communication::Antenna;
use crate::communication::Receiver;
use crate::communication::Sender;
use crate::service::Service;

pub struct ServiceManager {
    sender: Sender,
    service: Service,
    parent: Option<String>,
}

impl ServiceManager {
    fn service_presets(service: &mut Service) {
        service.add_endpoint(crate::service::Endpoint::from_str(&format!(
            "{}/#",
            service.name()
        )));
    }

    pub fn new(parent: Option<String>, mut service: Service) -> ServiceManager {
        if parent.is_none() {
            ServiceManager::service_presets(&mut service);
        }
        let sender = Sender::create(service.clone(), None);

        ServiceManager {
            service,
            sender,
            parent,
        }
    }

    pub fn sender(&self) -> &Sender {
        &self.sender
    }
    pub fn receiver(&self) -> Receiver {
        Receiver::create(self.sender.clone())
    }
    pub fn antenna(&self) -> Antenna {
        Antenna::create(self.sender.clone())
    }
    pub fn service(&self) -> &Service {
        &self.service
    }
}
