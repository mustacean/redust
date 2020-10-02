use crate::communication::Receiver;
use crate::service::Endpoint;

pub trait IRespond {
    fn respond(&self, recv: &Receiver, token: &str, res: ResponseType) -> Result<i32, ()>;
}

impl IRespond for Endpoint {
    fn respond(&self, recv: &Receiver, token: &str, res: ResponseType) -> Result<i32, ()> {
        use crate::rd_tools::IRedisClient;
        match res {
            ResponseType::ListResponse(_) => Err(()),
            ResponseType::StreamResponse(_) => Err(()),
            ResponseType::StringResponse(s) => {
                crate::rd_tools::rpush_str(recv.get_conn(), token, &s)
            }
            _ => Ok(0),
        }
    }
}

pub enum ResponseType {
    StringResponse(String),
    ListResponse(String),
    StreamResponse(String),
    None,
}
