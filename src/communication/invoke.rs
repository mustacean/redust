use crate::communication::Sender;
use crate::service::Event;

pub trait IInvoker {
    fn invoke(&self, sender: &Sender, args: &str) -> Result<i32, ()>;
}

impl IInvoker for Event {
    fn invoke(&self, sender: &Sender, args: &str) -> Result<i32, ()> {
        use crate::rd_tools::IRedisClient;
        match crate::rd_tools::publish(sender.get_conn(), &self.to_string(), args) {
            Ok(x) => Ok(x),
            _ => Err(()),
        }
    }
}
