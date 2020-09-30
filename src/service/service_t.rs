use crate::communication::{Receiver, Sender};
use crate::service::{Endpoint, Event};

pub struct Service {
    name: &'static str,
    host: &'static str,
    sender: Box<Sender>,
    receiver: Box<Receiver>,
}

impl Service {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn host(&self) -> &str {
        &self.host
    }
    pub fn sender(&self) -> &Sender {
        self.sender.as_ref()
    }
    pub fn receiver(&self) -> &Receiver {
        self.receiver.as_ref()
    }
    pub fn new_event(&self, name: &str) -> Event {
        super::event_t::new_event(self.name, name)
    }
    pub fn new_enpoint(&self, name: &str) -> Endpoint {
        super::endpoint_t::new_endpoint(self.name, name)
    }

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
}

pub fn open_service(
    host: &'static str,
    service_name: &'static str,
    events: &[&str],
    endpoints: &[&str],
    subscriptions: &[&str],
) -> Result<Service, ()> {
    let sender = Sender::new(host, None);

    let recv = Receiver::new(&sender, None, None);

    Ok(Service {
        name: service_name,
        host,
        sender: Box::new(sender),
        receiver: Box::new(recv),
    })
}

#[test]
fn test_new_service() {
    assert_eq!(
        true,
        matches!(
            open_service(
                "127.0.0.1",
                "user_service",
                &["user.added", "user.removed"],
                &["master/get_online_service_list", "master/"],
                &["master.service_onlined", "master.service_left"]
            ),
            Ok(_)
        )
    )
}
