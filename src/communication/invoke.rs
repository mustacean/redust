use crate::communication::Sender;
use crate::service::Event;
use serde_json::Value;

pub trait IInvoker {
    fn invoke(&self, sender: &Sender, payload: Value) -> Result<i32, ()>;
}

impl IInvoker for Event {
    fn invoke(&self, sender: &Sender, payload: Value) -> Result<i32, ()> {
        use crate::rd_tools::IRedisClient;
        match crate::rd_tools::publish(
            sender.get_conn(),
            &self.to_string(),
            serde_json::to_string(&payload).unwrap(),
        ) {
            Ok(x) => Ok(x),
            _ => Err(()),
        }
    }
}
