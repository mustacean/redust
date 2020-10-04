use crate::communication::Antenna;
use crate::communication::Sender;
use crate::rd_tools::IRedisClient;
use crate::service::{Endpoint, Event};
use std::rc::Rc;

#[derive(Clone)]
pub struct Receiver {
    client: Rc<Box<redis::Client>>,
    endpoints: Rc<Box<Vec<Endpoint>>>,
    sender: Sender,
    antenna: Antenna,
    service_name: &'static str,
    host: &'static str,
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
        sender: Sender,
        service_name: &'static str,
        host: &'static str,
        endpoints: Vec<Endpoint>,
        subscriptions: Vec<Event>,
    ) -> Receiver {
        let recv = Receiver {
            client: sender.get_client_rc(),
            antenna: Antenna::new(&sender, subscriptions),
            endpoints: Rc::new(Box::new(endpoints)),
            sender,
            service_name,
            host,
        };
        recv
    }

    pub fn endpoints(&self) -> &Vec<Endpoint> {
        self.endpoints.as_ref()
    }
    pub fn sender(&self) -> &Sender {
        &self.sender
    }
    pub fn antenna(&self) -> &Antenna {
        &self.antenna
    }
    pub fn service_name(&self) -> &'static str {
        self.service_name
    }
    pub fn host(&self) -> &'static str {
        self.host
    }

    pub fn endpoint_names(&self) -> Vec<String> {
        self.endpoints()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }

    pub fn receive_endpoints(&self, action: impl Fn(&Endpoint, &Sender, &serde_json::Value)) {
        crate::rd_tools::blpop_str_multiple(
            self.get_conn(),
            &self.endpoint_names(),
            0,
            |request_body, endp| {
                let ep_received = Endpoint::from_str(&endp);

                use crate::communication::IRespond;

                let val: serde_json::Value = serde_json::from_str(&request_body).unwrap();

                let token = val["token"].to_string();

                if ep_received.name() == "" {
                    ep_received
                        .respond(
                            self,
                            &self.sender().clone_from_token(&token),
                            super::receive::get_meta_info(self),
                        )
                        .unwrap();
                } else {
                    (action)(
                        &ep_received,
                        &self.sender().clone_from_token(&token),
                        &val["payload"],
                    )
                };
            },
        );
    }
}

fn get_meta_info(r: &Receiver) -> serde_json::Value {
    let sv_nm = r.service_name().to_owned();
    let sv_host = r.host().to_owned();
    let sv_token = r.sender().get_token().to_owned();
    use std::iter::*;
    let evts: Vec<serde_json::Value> = r
        .sender()
        .event_names()
        .iter()
        .map(|x| serde_json::Value::String(x.to_owned()))
        .collect();

    let endps: Vec<serde_json::Value> = r
        .endpoint_names()
        .iter()
        .map(|x| serde_json::Value::String(x.to_owned()))
        .collect();
    let subs: Vec<serde_json::Value> = r
        .antenna()
        .subsc_names()
        .iter()
        .map(|x| serde_json::Value::String(x.to_owned()))
        .collect();

    let mut mp = serde_json::Map::new();

    mp.insert("token".to_owned(), serde_json::Value::String(sv_token));
    mp.insert("name".to_owned(), serde_json::Value::String(sv_nm));
    mp.insert("host".to_owned(), serde_json::Value::String(sv_host));

    mp.insert("events".to_owned(), serde_json::Value::Array(evts));
    mp.insert("endpoints".to_owned(), serde_json::Value::Array(endps));
    mp.insert("subscriptions".to_owned(), serde_json::Value::Array(subs));

    serde_json::Value::Object(mp)
}
