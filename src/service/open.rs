use crate::service::Service;
use crate::service::ServiceManager;

impl Service {
    pub fn open(
        parent_token: Option<String>,
        service: Service,
    ) -> Result<ServiceManager, &'static str> {
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
        Ok(ServiceManager::new(parent_token, service))
    }
    fn name_validity_check(name: &str) -> Result<&str, &'static str> {
        if name.trim().is_empty() | name.contains("#") {
            Err("invalid naming attempt.!!")
        } else {
            Ok(name)
        }
    }
}
