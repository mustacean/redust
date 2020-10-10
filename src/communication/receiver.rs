use crate::communication::Sender;
use crate::rd_tools::IRedisClient;
use crate::service::Endpoint;
use crate::service::Service;
use std::rc::Rc;

pub struct Receiver {
    service: Service,
    client: Rc<redis::Client>,
}

impl Receiver {
    pub fn create(service: Service, sender: &Sender) -> Receiver {
        Receiver {
            service,
            client: sender.get_client_rc(),
        }
    }
    pub fn service(&self) -> &Service {
        &self.service
    }
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
    pub fn receive_endpoints(
        &self,
        func: impl Fn(&Endpoint, &Service, &serde_json::Value) -> Result<i32, ()>,
    ) {
        crate::rd_tools::blpop_str_multiple(
            self.get_conn(),
            &self.service().endpoint_names(),
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
                        Service::service_to_json(self.service()),
                    )
                } else {
                    (func)(
                        &ep_received,
                        &Service::token_to_service(self.service(), &token),
                        &val["payload"],
                    )
                };
            },
        );
    }
}
