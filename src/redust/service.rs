use crate::redust::event::Event;
use crate::redust::s_meta_provider::ServiceMetaProvider;
use std::rc::Rc;

pub struct Service {
    name: String,
    host: String,
    _events: Vec<Event>,
    _provider: Rc<Box<ServiceMetaProvider>>,
}

impl Service {
    pub fn new(name: String, _provider: Rc<Box<ServiceMetaProvider>>) -> Service {
        Service {
            name,
            host: String::default(),
            _provider,
            _events: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn _get_provider(&self) -> Rc<Box<ServiceMetaProvider>> {
        self._provider.clone()
    }
    pub fn _get_events(&self) -> &Vec<Event> {
        &self._events
    }
}
