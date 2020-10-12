use crate::communication::Receiver;
use crate::communication::Sender;
use crate::service::Endpoint;
use serde_json::Value;

pub trait IRespond {
    fn respond(&self, recv: &Receiver, sender: &Sender, response_payload: Value)
        -> Result<i32, ()>;
    fn respond_token(
        &self,
        recv: &Receiver,
        token: &str,
        response_payload: Value,
    ) -> Result<i32, ()>;
}

impl IRespond for Endpoint {
    fn respond_token(
        &self,
        recv: &Receiver,
        token: &str,
        response_payload: Value,
    ) -> Result<i32, ()> {
        use crate::rd_tools::IRedisClient;

        let msg = super::formats::serialize_response(recv.sender().token(), token, response_payload);
        crate::rd_tools::rpush_str(recv.sender().get_conn(), token, &msg)
    }
    fn respond(
        &self,
        r: &Receiver,
        s: &Sender,
        v: serde_json::Value,
    ) -> std::result::Result<i32, ()> {
        self.respond_token(r, s.token(), v)
    }
}
