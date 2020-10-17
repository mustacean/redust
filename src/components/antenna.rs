use crate::components::Sender;
use crate::service::Event;

#[derive(Clone)]
pub struct Antenna {
    sender: Sender,
    subscription_strings: Vec<String>,
}
impl Antenna {
    pub fn create(sender: Sender, subscription_strings: Vec<String>) -> Antenna {
        Antenna {
            sender,
            subscription_strings,
        }
    }
    pub fn sender(&self) -> &Sender {
        &self.sender
    }
    pub fn receive_events(&self, action: impl Fn(&Event, &serde_json::Value)) {
        let when_recv = |x: redis::RedisResult<redis::Msg>| {
            let r = crate::communication::formats::deserialize_event_args(&x.unwrap());
            (action)(&r.0, &r.1);
        };
        crate::rd_tools::receive(
            self.sender().get_conn(),
            self.subscription_strings.clone(),
            when_recv,
        );
    }

    pub fn receive_events_async(
        &self,
        action: impl Send + Sync + 'static + Fn(&Event, &serde_json::Value),
    ) -> std::thread::JoinHandle<()> {
        let cn = self.sender().get_conn();
        let sns = self.subscription_strings.clone();
        std::thread::spawn(move || {
            crate::rd_tools::receive(cn, sns, |x: redis::RedisResult<redis::Msg>| {
                let r = crate::communication::formats::deserialize_event_args(&x.unwrap());
                (action)(&r.0, &r.1);
            });
        })
    }
}
