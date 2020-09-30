use crate::communication::Sender;
use crate::service::Event;

pub trait IInvoker {
    fn invoke(&mut self, sender: &Sender, args: &str) -> Result<i32, ()>;

    // to be deleted later:
    fn invoke_with_conn(&mut self, client: &redis::Client, args: &str) -> Result<i32, ()>;
}

impl IInvoker for Event {
    fn invoke(&mut self, sender: &Sender, args: &str) -> Result<i32, ()> {
        use crate::rd_tools::IRedisClient;
        self.invoke_with_conn(&sender.get_client(), args)
    }
    fn invoke_with_conn(&mut self, client: &redis::Client, args: &str) -> Result<i32, ()> {
        match crate::rd_tools::publish(client.get_connection().unwrap(), &self.to_string(), args) {
            Ok(x) => Ok(x),
            _ => Err(()),
        }
    }
}
