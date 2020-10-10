use crate::service::{Endpoint, Event};
use std::rc::Rc;

#[derive(Clone)]
pub struct Service {
    name: &'static str,
    host: &'static str,
    token: String,
    events: Rc<Vec<Event>>,
    endpoints: Rc<Vec<Endpoint>>,
    subscriptions: Rc<Vec<Event>>,
}

impl Service {
    pub fn new(
        name: &'static str,
        host: &'static str,
        events: Vec<Event>,
        eps: Vec<Endpoint>,
        subs: Vec<Event>,
    ) -> Service {
        Service {
            name,
            host,
            events: Rc::new(events),
            subscriptions: Rc::new(subs),
            endpoints: Rc::new(eps),
            token: format!("{}", uuid::Uuid::new_v4()),
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
    pub fn add_endpoint(&mut self, ep: Endpoint) {
        let yoo = Rc::get_mut(&mut self.endpoints).unwrap();
        yoo.push(ep);
    }
    pub fn add_event(&mut self, ev: Event) {
        let yoo = Rc::get_mut(&mut self.events).unwrap();
        yoo.push(ev);
    }
    pub fn add_subs(&mut self, ev: Event) {
        let yoo = Rc::get_mut(&mut self.subscriptions).unwrap();
        yoo.push(ev);
    }

    pub fn endpoint_count(&self) -> usize {
        self.endpoints().len()
    }

    pub fn token(&self) -> &str {
        &self.token
    }
    pub fn set_token(&mut self, token: &str) {
        self.token = token.to_owned();
    }
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
