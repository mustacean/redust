use crate::rd_tools::IRedisClient;
use crate::service::{Endpoint, Event, IServiceOwned, Service};

pub struct Receiver<'t> {
    client: redis::Client,
    service: &'t Service<'t>,
    subscriptions: Option<Vec<Event>>,
    endpoints: Option<Vec<Endpoint>>,
}
impl<'t> IServiceOwned<'t> for Receiver<'t> {
    fn get_service(&self) -> &'t Service {
        self.service
    }
}

impl<'t> IRedisClient for Receiver<'t> {
    fn get_client(&self) -> &redis::Client {
        &self.client
    }
    fn get_conn(&self) -> redis::Connection {
        self.get_client().get_connection().unwrap()
    }
}

impl<'t> Receiver<'t> {
    pub fn get_subscriptions(&self) -> Option<&Vec<Event>> {
        self.subscriptions.as_ref()
    }
    pub fn get_endpoints(&self) -> Option<&Vec<Endpoint>> {
        self.endpoints.as_ref()
    }
}
