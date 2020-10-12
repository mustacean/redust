use crate::communication::Sender;
use crate::service::Endpoint;

pub struct Receiver {
    sender: Sender,
    filter_and_action: (
        Box<dyn Fn(&Endpoint) -> bool>,
        Box<dyn Fn(&Endpoint, &Receiver, &str) -> Result<i32, ()>>,
    ),
}

impl Receiver {
    pub fn create(
        sender: Sender,
        filter_and_action: (
            Box<dyn Fn(&Endpoint) -> bool>,
            Box<dyn Fn(&Endpoint, &Receiver, &str) -> Result<i32, ()>>,
        ),
    ) -> Receiver {
        Receiver {
            sender,
            filter_and_action,
        }
    }
    pub fn sender(&self) -> &Sender {
        &self.sender
    }
}

impl Receiver {
    pub fn receive_endpoints(
        &self,
        func: impl Fn(&Endpoint, &Sender, &serde_json::Value) -> Result<i32, ()>,
    ) {
        use crate::rd_tools::IRedisClient;
        crate::rd_tools::blpop_str_multiple(
            self.sender().get_conn(),
            &self.sender().service().endpoint_names(),
            0,
            |request_body, endp| {
                let ep_received = Endpoint::from_str(&endp);

                let val: serde_json::Value = serde_json::from_str(&request_body).unwrap();

                let token = val["token"].to_string();

                if let Err(()) = if !(self.filter_and_action.0)(&ep_received) {
                    (func)(
                        &ep_received,
                        &Sender::clone_from_token(&self.sender(), &token),
                        &val["payload"],
                    )
                } else {
                    (self.filter_and_action.1)(&ep_received, self, &token)
                } {
                    panic!("we couldn't act on the request :/")
                }
            },
        );
    }
}
