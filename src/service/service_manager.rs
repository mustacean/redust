use crate::components::Antenna;
use crate::components::Receiver;
use crate::components::Sender;
use crate::components::Storage;
use crate::service::Service;

pub struct ServiceManager {
    sender: Sender,
    service: Service,
    parent: Option<String>,
}

impl ServiceManager {
    fn service_presets(is_parent: bool, service: &mut Service) {
        if is_parent {
            service.add_endpoint(crate::service::Endpoint::from_str(&format!(
                "{}/#",
                service.name()
            )));
        }
    }

    pub fn new(parent: Option<String>, mut service: Service) -> ServiceManager {
        ServiceManager::service_presets(parent.is_none(), &mut service);

        ServiceManager {
            service: service.clone(),
            sender: Sender::create(service, None),
            parent,
        }
    }

    pub fn sender(&self) -> &Sender {
        &self.sender
    }

    pub fn service(&self) -> &Service {
        &self.service
    }

    pub fn receiver(&self) -> Result<Receiver, &'static str> {
        if self.service().endpoint_count() == 0 {
            Err("service has no endpoints.")
        } else {
            Ok(Receiver::create(
                self.sender.clone(),
                (
                    Box::new(|ep| ep.name() == "#"),
                    Box::new(|ep, recv, token| {
                        use crate::communication::IRespond;
                        ep.respond_token(recv, token, Service::to_json(recv.sender().service()))
                    }),
                ),
            ))
        }
    }

    pub fn antenna(&self) -> Result<Antenna, &'static str> {
        if self.service().subscription_count() == 0 {
            Err("service has no subscriptions.")
        } else {
            Ok(Antenna::create(self.sender.clone()))
        }
    }

    pub fn storage(&self) -> Result<Storage, &'static str> {
        Ok(Storage::create(self.sender.clone()))
    }
}
