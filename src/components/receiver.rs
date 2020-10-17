use super::Sender;
use crate::service::Endpoint;

pub struct Receiver {
    sender: Sender,
    filter_and_action: (
        Box<dyn Fn(&Endpoint) -> bool>,
        Box<dyn Fn(&Endpoint, &Receiver, &str) -> Option<serde_json::Value>>,
    ),
}

impl Receiver {
    pub fn create(
        sender: Sender,
        filter_and_action: (
            Box<dyn Fn(&Endpoint) -> bool>,
            Box<dyn Fn(&Endpoint, &Receiver, &str) -> Option<serde_json::Value>>,
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
        func: impl Fn(&Endpoint, &Sender, &serde_json::Value) -> Option<serde_json::Value>,
    ) {
        crate::rd_tools::blpop_str_multiple(
            self.sender().get_conn(),
            &self.sender().service().endpoint_names(),
            0,
            |request_body, endp| {
                let ep_received = Endpoint::from_str(&endp);

                let (token, payload) =
                    crate::communication::formats::detokenize_request(&request_body);

                let returned = if !(self.filter_and_action.0)(&ep_received) {
                    (func)(
                        &ep_received,
                        &Sender::clone_from_token(&self.sender(), &token),
                        &payload,
                    )
                } else {
                    (self.filter_and_action.1)(&ep_received, self, &token)
                };
                if let Err(()) = crate::communication::respond(self, &token, returned.unwrap()) {
                    panic!("something went wrong while we receive");
                }
            },
        );
    }
}
