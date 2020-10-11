use crate::communication::Sender;
use crate::service::Endpoint;
use crate::service::Service;
use std::rc::Rc;

pub struct Receiver {
    sender: Rc<Sender>,
}

impl Receiver {
    pub fn create(sender: Rc<Sender>) -> Receiver {
        Receiver { sender }
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
                use crate::communication::IRespond;

                let val: serde_json::Value = serde_json::from_str(&request_body).unwrap();

                let token = val["token"].to_string();

                let _temp_res = if ep_received.name() == "#" {
                    ep_received.respond_token(
                        self,
                        &token,
                        Service::to_json(self.sender().service()),
                    )
                } else {
                    (func)(
                        &ep_received,
                        &Sender::clone_from_token(&self.sender(), &token),
                        &val["payload"],
                    )
                };
            },
        );
    }
}
