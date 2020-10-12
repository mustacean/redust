use crate::communication::Sender;
use crate::service::Event;

#[derive(Clone)]
pub struct Antenna {
    sender: Sender,
}
impl Antenna {
    pub fn create(sender: Sender) -> Antenna {
        Antenna { sender }
    }
    pub fn sender(&self) -> &Sender {
        &self.sender
    }
    pub fn receive_events(&self, action: impl Fn(&Event, &serde_json::Value)) {
        use crate::rd_tools::IRedisClient;
        crate::rd_tools::receive(
            self.sender().get_conn(),
            self.sender().service().subsc_names(),
            |x| {
                let r = super::formats::deserialize_event_args(&x.unwrap());
                (action)(&r.0, &r.1);
            },
        )
    }
}
