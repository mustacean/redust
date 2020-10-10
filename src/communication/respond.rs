use crate::communication::Receiver;
use crate::service::Endpoint;
use crate::service::Service;
use serde_json::Value;

pub trait IRespond {
    fn respond(
        &self,
        recv: &Receiver,
        service_sent: &Service,
        response_payload: Value,
    ) -> Result<i32, ()>;
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

        let mut mp = serde_json::Map::new();
        mp.insert(
            "from".to_owned(),
            serde_json::Value::String(recv.sender().service().token().to_owned()),
        );
        mp.insert("to".to_owned(), serde_json::Value::String(token.to_owned()));
        mp.insert("payload".to_owned(), response_payload);

        let ss = serde_json::to_string(&serde_json::Value::Object(mp)).unwrap();

        crate::rd_tools::rpush_str(recv.sender().get_conn(), token, &ss)
    }
    fn respond(
        &self,
        r: &Receiver,
        s: &Service,
        v: serde_json::Value,
    ) -> std::result::Result<i32, ()> {
        self.respond_token(r, s.token(), v)
    }
}
