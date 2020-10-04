use crate::communication::Sender;
use crate::rd_tools::IRedisClient;
use crate::service::{Endpoint, Event};
use std::rc::Rc;

#[derive(Clone)]
pub struct Receiver {
    client: Rc<Box<redis::Client>>,
    endpoints: Rc<Box<Vec<Endpoint>>>,
    subscriptions: Rc<Box<Vec<Event>>>,
    sender: Sender,
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
    pub fn new(sender: Sender, endpoints: Vec<Endpoint>, subscriptions: Vec<Event>) -> Receiver {
        let recv = Receiver {
            client: sender.get_client_rc(),
            subscriptions: Rc::new(Box::new(subscriptions)),
            endpoints: Rc::new(Box::new(endpoints)),
            sender,
        };
        recv
    }

    pub fn subscriptions(&self) -> &Vec<Event> {
        self.subscriptions.as_ref()
    }
    pub fn endpoints(&self) -> &Vec<Endpoint> {
        self.endpoints.as_ref()
    }
    pub fn sender(&self) -> &Sender {
        &self.sender
    }

    pub fn receive_events(&self, action: impl Fn(Event, serde_json::Value)) {
        use std::iter::*;

        let subsc_names = self
            .subscriptions()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        crate::rd_tools::receive(self.get_conn(), subsc_names, |x| {
            let result = x.unwrap();
            let ch = result.get_channel::<String>().unwrap();

            let msg = result.get_payload::<String>().unwrap();

            (action)(Event::from_string(&ch), serde_json::from_str(&msg).unwrap());
        })
    }

    pub fn receive_endpoints(
        &self,
        action: impl Fn(&Endpoint, &Sender, &serde_json::Value) -> serde_json::Value,
    ) {
        let ep_names = self
            .endpoints()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        crate::rd_tools::blpop_str_multiple(self.get_conn(), &ep_names, 0, |request_body, endp| {
            let ep_received = Endpoint::from_string(&endp);
            use crate::communication::IRespond;

            let val: serde_json::Value = serde_json::from_str(&request_body).unwrap();

            let token = val["token"].to_string();

            ep_received.respond(
                self,
                &token,
                (action)(
                    &ep_received,
                    &self.sender().clone().from_token(&token),
                    &val["payload"],
                ),
            );
        });
    }
}
