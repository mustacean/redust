use crate::communication::Sender;
use crate::rd_tools::IRedisClient;
use crate::service::{Endpoint, Event};
use std::rc::Rc;

pub struct Receiver {
    client: Rc<Box<redis::Client>>,
    endpoints: Option<Vec<Endpoint>>,
    subscriptions: Option<Vec<Event>>,
}

impl IRedisClient for Receiver {
    fn get_client_rc(&self) -> Rc<Box<redis::Client>> {
        self.client.clone()
    }
    fn get_client(&self) -> &redis::Client {
        &self.client
    }
    fn get_conn(&self) -> redis::Connection {
        self.get_client().get_connection().unwrap()
    }
}

impl Receiver {
    pub fn new(
        sender: &Sender,
        endpoints: Option<Vec<Endpoint>>,
        subscriptions: Option<Vec<Event>>,
    ) -> Receiver {
        let recv = Receiver {
            client: sender.get_client_rc(),
            subscriptions,
            endpoints,
        };
        recv
    }

    pub fn get_subscriptions(&self) -> Option<&Vec<Event>> {
        self.subscriptions.as_ref()
    }
    pub fn get_endpoints(&self) -> Option<&Vec<Endpoint>> {
        self.endpoints.as_ref()
    }

    pub fn receive_events(
        &self,
        action: Box<dyn Fn(Event, String) + Send + Sync>,
    ) -> std::thread::JoinHandle<()> {
        use std::iter::*;

        let subsc_names = self
            .get_subscriptions()
            .unwrap()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        crate::rd_tools::receive(
            self.get_conn(),
            subsc_names,
            Box::new(move |x| {
                // middleware...
                let result = x.unwrap();
                let ch = result.get_channel::<String>().unwrap();

                let msg = result.get_payload::<String>().unwrap();

                (action)(Event::from_string(&ch), msg);
            }),
        )
    }

    pub fn receive_endpoints(
        &self,
        action: Box<dyn Fn(Endpoint, String) + Send + Sync>,
    ) -> std::thread::JoinHandle<()> {
        let ep_names = self
            .get_endpoints()
            .unwrap()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        crate::rd_tools::blpop_str_multiple(
            self.get_conn(),
            ep_names,
            0,
            Box::new(move |(s, e)| {
                (action)(Endpoint::from_string(&e), s);
            }),
        )
    }
}
