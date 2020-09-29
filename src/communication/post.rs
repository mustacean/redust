use crate::communication::Sender;
use crate::service::Endpoint;

pub trait IPost {
    fn post<T>(&mut self, sender: &Sender, args: &str) -> Result<T, ()>;
}

impl IPost for Endpoint {
    fn post<T>(&mut self, _sender: &Sender, _args: &str) -> Result<T, ()> {
        // let con = sender.get_service().get_conn();
        // // complex ...
        todo!()
    }
}
