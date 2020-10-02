use crate::communication::Sender;
use crate::rd_tools::IRedisClient;
use crate::service::{Endpoint, Event};
use std::rc::Rc;

#[derive(Clone)]
pub struct Receiver {
    client: Rc<Box<redis::Client>>,
    endpoints: Rc<Box<Vec<Endpoint>>>,
    subscriptions: Rc<Box<Vec<Event>>>,
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
    pub fn new(sender: &Sender, endpoints: Vec<Endpoint>, subscriptions: Vec<Event>) -> Receiver {
        let recv = Receiver {
            client: sender.get_client_rc(),
            subscriptions: Rc::new(Box::new(subscriptions)),
            endpoints: Rc::new(Box::new(endpoints)),
        };
        recv
    }

    pub fn subscriptions(&self) -> &Vec<Event> {
        self.subscriptions.as_ref()
    }
    pub fn endpoints(&self) -> &Vec<Endpoint> {
        self.endpoints.as_ref()
    }

    pub fn receive_events(&self, action: impl Fn(Event, String)) {
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

            (action)(Event::from_string(&ch), msg);
        })
    }

    pub fn receive_endpoints(
        &self,
        action: impl Fn(Endpoint, String) -> crate::communication::ResponseType,
    ) {
        let ep_names = self
            .endpoints()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        crate::rd_tools::blpop_str_multiple(
            self.get_conn(),
            &ep_names,
            0,
            |request_payload, endp| {
                let ep_received = Endpoint::from_string(&endp);
                use crate::communication::IRespond;
                if let Ok(i) = ep_received.respond(
                    self,
                    &request_payload,
                    (action)(ep_received.clone(), request_payload.clone()),
                ) {
                    if i > 0 {
                        println!("responded");
                    } else {
                        println!("no response");
                    }
                }
            },
        );
    }
}
