use crate::redust::event::Event;
use crate::redust::s_meta_provider::ServiceMetaProvider;
use std::rc::Rc;

pub struct Service {
    name: String,
    host: String,
    events: Vec<Event>,
    provider: Rc<Box<ServiceMetaProvider>>,
}

impl Service {
    pub fn new(
        name: &str,
        host: &str,
        provider: Rc<Box<ServiceMetaProvider>>,
        evs: Option<Vec<Event>>,
    ) -> Service {
        Service {
            name: name.to_owned(),
            host: host.to_owned(),
            provider,
            events: match evs {
                Some(e) => e,
                None => Vec::new(),
            },
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_provider(&self) -> Rc<Box<ServiceMetaProvider>> {
        self.provider.clone()
    }
    pub fn get_events(&self) -> &Vec<Event> {
        &self.events
    }
}
