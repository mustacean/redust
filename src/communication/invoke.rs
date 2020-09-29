use crate::communication::Sender;
use crate::rd_tools::IRedisClient;
use crate::service::Event;
use crate::service::IServiceOwned;

pub trait IInvoker {
    fn invoke(&mut self, sender: &Sender, args: &str) -> Result<i32, ()>;

    // to be deleted later:
    fn invoke_with_conn(&mut self, conn: redis::Connection, args: &str) -> Result<i32, ()>;
}

impl IInvoker for Event {
    fn invoke(&mut self, sender: &Sender, args: &str) -> Result<i32, ()> {
        let cnn = sender.get_service().get_conn();

        match crate::rd_tools::publish(cnn, &self.to_string(), args) {
            Ok(x) => Ok(x),
            _ => Err(()),
        }
    }
    fn invoke_with_conn(&mut self, cnn: redis::Connection, args: &str) -> Result<i32, ()> {
        match crate::rd_tools::publish(cnn, &self.to_string(), args) {
            Ok(x) => Ok(x),
            _ => Err(()),
        }
    }
}
