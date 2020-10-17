use super::Sender;
use crate::communication::formats::detokenize_request;
use crate::communication::respond;
use crate::service::Endpoint;

pub struct Receiver {
    sender: Sender,
    filter_and_action: (
        Box<dyn Send + Sync + Fn(&Endpoint) -> bool>,
        Box<dyn Send + Sync + Fn(&Endpoint, &Receiver, &str) -> Option<serde_json::Value>>,
    ),
    endpoint_strings: Vec<String>,
}

impl Receiver {
    pub fn create(
        sender: Sender,
        filter_and_action: (
            Box<dyn Send + Sync + Fn(&Endpoint) -> bool>,
            Box<dyn Send + Sync + Fn(&Endpoint, &Receiver, &str) -> Option<serde_json::Value>>,
        ),
        endpoint_strings: Vec<String>,
    ) -> Receiver {
        Receiver {
            sender,
            filter_and_action,
            endpoint_strings,
        }
    }
    pub fn sender(&self) -> &Sender {
        &self.sender
    }
}

impl Receiver {
    pub async fn receive_endpoints_async(
        &self,
        func: impl Fn(&Endpoint, &Sender, &serde_json::Value) -> Option<serde_json::Value>,
    ) {
        crate::rd_tools::blpop_str_multiple_async(
            self.sender().get_conn(),
            &self.endpoint_strings,
            0,
            |request_body: String, endp: String| {
                let ep_received = Endpoint::from_str(&endp);
                let (token, payload) = detokenize_request(&request_body);
                let returned = if !(self.filter_and_action.0)(&ep_received) {
                    (func)(&ep_received, &Sender::create(&token), &payload)
                } else {
                    (self.filter_and_action.1)(&ep_received, self, &token)
                };
                let returned = if let Some(x) = returned {
                    x
                } else {
                    serde_json::Value::Null
                };
                if let Err(()) = respond(self, &token, returned) {
                    panic!("something went wrong while responding :(");
                }
            },
        )
        .await;
    }
}
