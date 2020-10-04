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
            subscriptions: Rc::new(Box::new(subscriptions)),
            endpoints: Rc::new(Box::new(endpoints)),
            sender,
            service_name,
            host,
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
    pub fn service_name(&self) -> &'static str {
        self.service_name
    }
    pub fn host(&self) -> &'static str {
        self.host
    }

    pub fn subsc_names(&self) -> Vec<String> {
        self.subscriptions()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }

    pub fn receive_events(&self, action: impl Fn(&Event, &serde_json::Value)) {
        crate::rd_tools::receive(self.get_conn(), self.subsc_names(), |x| {
            let result = x.unwrap();
            let ch = result.get_channel::<String>().unwrap();

            let msg = result.get_payload::<String>().unwrap();

            (action)(&Event::from_str(&ch), &serde_json::from_str(&msg).unwrap());
        })
    }

    pub fn endpoint_names(&self) -> Vec<String> {
        self.endpoints()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }

    pub fn receive_endpoints(
        &self,
        action: impl Fn(&Endpoint, &Sender, &serde_json::Value) -> serde_json::Value,
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

                let response_body = if ep_received.name() == "" {
                    self.get_meta_info()
                } else {
                    (action)(
                        &ep_received,
                        &self.sender().clone_from_token(&token),
                        &val["payload"],
                    )
                };

                ep_received.respond(self, &token, response_body);
            },
        );
    }

    fn get_meta_info(&self) -> serde_json::Value {
        let sv_nm = self.service_name().to_owned();
        let sv_host = self.host().to_owned();
        let sv_token = self.sender().get_token().to_owned();
        let evts = self.sender().event_names();
        let endps = self.endpoint_names();
        let subs = self.subsc_names();

        let _meta_info_to_serialize = Smeta::new(sv_nm, sv_host, sv_token, evts, endps, subs);

        //
        todo!()
    }
}

// can't find the goddamn bloody fuckin' macro :S
#[allow(dead_code)]
struct Smeta {
    x: String,
    y: String,
    z: String,
    w: Vec<String>,
    u: Vec<String>,
    t: Vec<String>,
}

impl Smeta {
    fn new(
        x: String,
        y: String,
        z: String,
        w: Vec<String>,
        u: Vec<String>,
        t: Vec<String>,
    ) -> Smeta {
        Smeta { x, y, z, w, u, t }
    }
}
