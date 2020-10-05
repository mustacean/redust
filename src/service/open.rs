use crate::communication::Receiver;
use crate::communication::Sender;
use crate::service::Endpoint;
use crate::service::Event;
use crate::service::Service;

impl Service {
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

        Ok(Service::new(Receiver::new(
            Sender::new(host, events),
            service_name,
            host,
            endpoints,
            subscriptions,
        )))
    }
    fn name_validity_check(name: &'static str) -> Result<&'static str, &'static str> {
        if name.trim().is_empty() {
            Err("invalid naming attempt.!!")
        } else {
            Ok(name)
        }
    }
}
