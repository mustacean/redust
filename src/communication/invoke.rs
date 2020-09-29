use crate::communication::Sender;
use crate::service::Event;

pub trait IInvoker {
    fn invoke(&mut self, sender: &Sender, args: &str);
}

pub trait IRdInvoker {
    fn invoke(&mut self, _sender: &Sender, _args: &str) {
        // let cnn = sender.get_service().get_conn();
        // rd_tools::publish(cnn, self.to_string(), args);
        todo!()
    }
}

impl IRdInvoker for Event {}
