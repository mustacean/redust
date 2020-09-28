use crate::rd_tools::IRedisClient;
use crate::service::{Endpoint, Event, IServiceOwned, Service};

pub struct Sender<'t> {
    service: &'t Service<'t>,
    client: redis::Client,
    events: Option<Vec<Event>>,
    targets_known: Option<Vec<Endpoint>>,
}

impl<'t> IServiceOwned<'t> for Sender<'t> {
    fn get_service(&self) -> &'t Service {
        self.service
    }
}
impl<'t> IRedisClient for Sender<'t> {
    fn get_client(&self) -> &redis::Client {
        &self.client
    }
    fn get_conn(&self) -> redis::Connection {
        self.get_client().get_connection().unwrap()
    }
}

impl<'t> Sender<'t> {
    pub fn get_targets(&self) -> Option<&Vec<Endpoint>> {
        self.targets_known.as_ref()
    }

    pub fn get_events(&self) -> Option<&Vec<Event>> {
        self.events.as_ref()
    }
}
