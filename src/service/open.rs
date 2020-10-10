use crate::service::Service;
use crate::service::ServiceManager;

impl Service {
    pub fn open(mut service: Service) -> Result<ServiceManager, &'static str> {
        Service::name_validity_check(service.name())?;

        for n in service.events() {
            Service::name_validity_check(n.name())?;
        }
        for n in service.endpoints() {
            Service::name_validity_check(n.name())?;
        }
        for n in service.subscriptions() {
            Service::name_validity_check(n.name())?;
        }

        service.add_endpoint(crate::service::endpoint_t::new_endpoint(
            service.name(),
            "#",
        ));

        Ok(ServiceManager::new(service))
    }
    fn name_validity_check(name: &str) -> Result<&str, &'static str> {
        if name.trim().is_empty() | name.contains("#") {
            Err("invalid naming attempt.!!")
        } else {
            Ok(name)
        }
    }
}
