use crate::service::{Endpoint, Event};
use std::sync::Arc;

#[derive(Clone)]
pub struct Service {
    name: Arc<String>,
    host: Arc<String>,
    events: Arc<Vec<Event>>,
    endpoints: Arc<Vec<Endpoint>>,
    subscriptions: Arc<Vec<Event>>,
}

impl Service {
    pub fn open(
        parent_token: Option<String>,
        service: Service,
    ) -> Result<super::ServiceManager, &'static str> {
        Ok(super::ServiceManager::new(parent_token, service))
    }

    pub fn new(
        name: &str,
        host: &str,
        events: Vec<Event>,
        eps: Vec<Endpoint>,
        subs: Vec<Event>,
    ) -> Service {
        fn name_validity_check(name: &str) {
            if name.trim().is_empty() | name.contains("#") {
                panic!("invalid naming attempt.!!");
            }
        }
        name_validity_check(name);

        for n in &events {
            name_validity_check(n.name());
        }
        for n in &eps {
            name_validity_check(n.name());
        }
        for n in &subs {
            name_validity_check(n.name());
        }
        Service {
            name: Arc::new(name.to_owned()),
            host: Arc::new(host.to_owned()),
            events: Arc::new(events),
            subscriptions: Arc::new(subs),
            endpoints: Arc::new(eps),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }
    pub fn event_count(&self) -> usize {
        self.events().len()
    }
    pub fn subscriptions(&self) -> &Vec<Event> {
        self.subscriptions.as_ref()
    }
    pub fn subscription_count(&self) -> usize {
        self.subscriptions().len()
    }
    pub fn endpoints(&self) -> &Vec<Endpoint> {
        self.endpoints.as_ref()
    }
    pub fn endpoint_count(&self) -> usize {
        self.endpoints().len()
    }

    pub fn add_endpoint(&mut self, ep: Endpoint) {
        let yoo = Arc::get_mut(&mut self.endpoints).unwrap();
        yoo.push(ep);
    }
    // pub fn add_event(&mut self, ev: Event) {
    //     let yoo = Rc::get_mut(&mut self.events).unwrap();
    //     yoo.push(ev);
    // }
    // pub fn add_subs(&mut self, ev: Event) {
    //     let yoo = Rc::get_mut(&mut self.subscriptions).unwrap();
    //     yoo.push(ev);
    // }

    pub fn endpoint_names(&self) -> Vec<String> {
        self.endpoints()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }
    pub fn subsc_names(&self) -> Vec<String> {
        self.subscriptions()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }
    pub fn event_names(&self) -> Vec<String> {
        self.events()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }
}
