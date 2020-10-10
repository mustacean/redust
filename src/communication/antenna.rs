use crate::communication::Sender;
use crate::service::Event;
use std::rc::Rc;

#[derive(Clone)]
pub struct Antenna {
    sender: Rc<Sender>,
}
impl Antenna {
    pub fn create(sender: Rc<Sender>) -> Antenna {
        Antenna { sender }
    }
    pub fn sender(&self) -> &Sender {
        self.sender.as_ref()
    }
    pub fn receive_events(&self, action: impl Fn(&Event, &serde_json::Value)) {
        use crate::rd_tools::IRedisClient;
        crate::rd_tools::receive(
            self.sender().get_conn(),
            self.sender().service().subsc_names(),
            |x| {
                let result = x.unwrap();
                let ch = result.get_channel::<String>().unwrap();

                let msg = result.get_payload::<String>().unwrap();

                (action)(&Event::from_str(&ch), &serde_json::from_str(&msg).unwrap());
            },
        )
    }
}
