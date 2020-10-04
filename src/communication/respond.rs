use crate::communication::Receiver;
use crate::service::Endpoint;
use serde_json::Value;

pub trait IRespond {
    fn respond(&self, recv: &Receiver, token: &str, response_payload: Value) -> Result<i32, ()>;
}

impl IRespond for Endpoint {
    fn respond(&self, recv: &Receiver, token: &str, response_payload: Value) -> Result<i32, ()> {
        use crate::rd_tools::IRedisClient;

        let mut mp = serde_json::Map::new();

        mp.insert(
            "from".to_owned(),
            serde_json::Value::String(recv.sender().get_token().to_owned()),
        );
        mp.insert("to".to_owned(), serde_json::Value::String(token.to_owned()));
        mp.insert("payload".to_owned(), response_payload);

        let ss = serde_json::to_string(&serde_json::Value::Object(mp)).unwrap();

        crate::rd_tools::rpush_str(recv.get_conn(), token, &ss)
    }
}
