use crate::communication::Antenna;
use crate::communication::Sender;
use crate::rd_tools::IRedisClient;
use crate::service::{Endpoint, Event};
use std::rc::Rc;

pub struct Receiver {
    client: Rc<redis::Client>,
    endpoints: Rc<Vec<Endpoint>>,
    sender: Sender,
    antenna: Antenna,
    service_name: &'static str,
    host: &'static str,
}

impl IRedisClient for Receiver {
    fn get_client_rc(&self) -> Rc<redis::Client> {
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
            endpoints: Rc::new(endpoints),
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

    pub fn receive_endpoints(
        &self,
        func: impl Fn(&Endpoint, &Sender, &serde_json::Value) -> Result<i32, ()>,
    ) {
        crate::rd_tools::blpop_str_multiple(
            self.get_conn(),
            &self.endpoint_names(),
            0,
            |request_body, endp| {
                let ep_received = Endpoint::from_str(&endp);
                use crate::communication::IRespond;

                let val: serde_json::Value = serde_json::from_str(&request_body).unwrap();

                let token = val["token"].to_string();
                let cl_tokn = &self.sender().clone_from_token(token);

                let _temp_res = if ep_received.name() == "#" {
                    ep_received.respond(self, cl_tokn, super::formats::get_meta_info(self))
                } else {
                    (func)(&ep_received, cl_tokn, &val["payload"])
                };
            },
        );
    }
}
