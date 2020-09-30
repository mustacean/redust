use crate::communication::{Receiver, Sender};
use crate::service::{Endpoint, Event};

pub struct Service {
    name: &'static str,
    host: &'static str,
    sender: Sender,
    receiver: Receiver,
}

impl Service {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn host(&self) -> &str {
        &self.host
    }
    pub fn sender(&self) -> &Sender {
        &self.sender
    }

    pub fn receiver(&self) -> &Receiver {
        &self.receiver
    }
    // pub fn new_event(&self, name: &str) -> Event {
    //     super::event_t::new_event(self.name, name)
    // }
    // pub fn new_enpoint(&self, name: &str) -> Endpoint {
    //     super::endpoint_t::new_endpoint(self.name, name)
    // }

    pub fn events(&self) -> Option<&Vec<Event>> {
        self.sender().get_events()
    }

    pub fn event_count(&self) -> usize {
        if let Some(x) = self.sender().get_events() {
            x.len()
        } else {
            0
        }
    }

    pub fn subscriptions(&self) -> Option<&Vec<Event>> {
        self.receiver().get_subscriptions()
    }

    pub fn subscription_count(&self) -> usize {
        if let Some(x) = self.receiver().get_subscriptions() {
            x.len()
        } else {
            0
        }
    }

    pub fn endpoints(&self) -> Option<&Vec<Endpoint>> {
        self.receiver().get_endpoints()
    }

    pub fn endpoint_count(&self) -> usize {
        if let Some(x) = self.receiver().get_endpoints() {
            x.len()
        } else {
            0
        }
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
        events: &[&str],
        endpoints: &[(&str, &str)],
        subscriptions: &[(&str, &str)],
    ) -> Result<Service, ()> {
        let events: Vec<Event> = events
            .iter()
            .map(|en| crate::service::event_t::new_event(service_name, en))
            .collect();
        let sender = Sender::new(host, Some(events));
        let endpoints: Vec<Endpoint> = endpoints
            .iter()
            .map(|(sn, epn)| crate::service::endpoint_t::new_endpoint(sn, epn))
            .collect();
        let subscriptions: Vec<Event> = subscriptions
            .iter()
            .map(|(sn, evn)| crate::service::event_t::new_event(sn, evn))
            .collect();
        let receiver = Receiver::new(&sender, Some(endpoints), Some(subscriptions));
        Ok(Service {
            name: service_name,
            host,
            sender,
            receiver,
        })
    }
}

#[test]
fn test_new_service() {
    assert_eq!(
        true,
        matches!(
            Service::open(
                "127.0.0.1",
                "user_service",
                &["added", "removed"],
                &[("master", "get_online_service_list")],
                &[
                    ("master", "service_onlined"),
                    ("master", "service_left"),
                    ("master", "service_updated"),
                ],
            ),
            Ok(_)
        )
    )
}
