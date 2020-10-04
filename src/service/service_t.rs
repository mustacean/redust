use crate::communication::{Receiver, Sender};
use crate::service::{Endpoint, Event};

pub struct Service {
    receiver: Receiver,
}

impl Service {
    pub fn name(&self) -> &str {
        &self.receiver().service_name()
    }
    pub fn host(&self) -> &str {
        &self.receiver().host()
    }
    pub fn sender(&self) -> &Sender {
        &self.receiver().sender()
    }

    pub fn receiver(&self) -> &Receiver {
        &self.receiver
    }

    pub fn new_event(&self, name: &str) -> Event {
        super::event_t::new_event(self.receiver().service_name(), name)
    }
    pub fn new_enpoint(&self, name: &str) -> Endpoint {
        super::endpoint_t::new_endpoint(self.receiver().service_name(), name)
    }

    pub fn master_event(&self, name: &str) -> Event {
        super::event_t::new_event("master", name)
    }
    pub fn master_endpoint(&self, name: &str) -> Endpoint {
        super::endpoint_t::new_endpoint("master", name)
    }

    pub fn events(&self) -> &Vec<Event> {
        self.sender().events()
    }

    pub fn event_count(&self) -> usize {
        self.sender().events().len()
    }

    pub fn subscriptions(&self) -> &Vec<Event> {
        self.receiver().antenna().subscriptions()
    }

    pub fn subscription_count(&self) -> usize {
        self.receiver().antenna().subscriptions().len()
    }

    pub fn endpoints(&self) -> &Vec<Endpoint> {
        self.receiver().endpoints()
    }

    pub fn endpoint_count(&self) -> usize {
        self.receiver().endpoints().len()
    }

    pub fn to_string(&self) -> String {
        format!(
            "Service : {}/{}\n[numEvents : {}\nnumEndpoints : {}\nnumSubscriptions : {}]",
            self.host(),
            self.name(),
            self.event_count(),
            self.endpoint_count(),
            self.subscription_count()
        )
    }

    pub fn open(
        host: &'static str,
        service_name: &'static str,
        events: &[&'static str],
        endpoints: &[&'static str],
        subscriptions: &[(&'static str, &'static str)],
    ) -> Result<Service, &'static str> {
        Service::name_validity_check(service_name)?;

        for n in events {
            Service::name_validity_check(n)?;
        }
        for n in endpoints {
            Service::name_validity_check(n)?;
        }
        for (n, m) in subscriptions {
            Service::name_validity_check(n)?;
            Service::name_validity_check(m)?;
        }

        let events: Vec<Event> = events
            .iter()
            .map(|en| crate::service::event_t::new_event(service_name, en))
            .collect();
        let mut endpoints: Vec<Endpoint> = endpoints
            .iter()
            .map(|epn| crate::service::endpoint_t::new_endpoint(service_name, epn))
            .collect();
        let subscriptions: Vec<Event> = subscriptions
            .iter()
            .map(|(sn, evn)| crate::service::event_t::new_event(sn, evn))
            .collect();

        endpoints.push(crate::service::endpoint_t::new_endpoint(service_name, ""));

        Ok(Service {
            receiver: Receiver::new(
                Sender::new(host, events),
                service_name,
                host,
                endpoints,
                subscriptions,
            ),
        })
    }

    fn name_validity_check(name: &'static str) -> Result<&'static str, &'static str> {
        if name.trim().is_empty() {
            Err("invalid naming attempt.!!")
        } else {
            Ok(name)
        }
    }
}
