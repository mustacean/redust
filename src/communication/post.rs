use crate::communication::Sender;
use crate::service::Endpoint;
use serde_json::Value;

pub trait IPost {
    fn post(&self, sender: &Sender, payload: Value) -> Result<Option<serde_json::Value>, ()>;
}

impl IPost for Endpoint {
    fn post(&self, sender: &Sender, payload: Value) -> Result<Option<serde_json::Value>, ()> {
        use crate::rd_tools::IRedisClient;

        let mut mp = serde_json::Map::new();

        mp.insert(
            "token".to_owned(),
            serde_json::Value::String(sender.service().token().to_owned()),
        );
        mp.insert("payload".to_owned(), payload);

        let ss = serde_json::to_string(&serde_json::Value::Object(mp)).unwrap();

        if let Ok(_) = crate::rd_tools::rpush_str(sender.get_conn(), &self.to_string(), &ss) {
            if let Ok(result) = crate::rd_tools::blpop_str(
                sender.get_conn(),
                &format!("\"{}\"", sender.service().token()),
                1,
            ) {
                Ok(Some(serde_json::from_str(&result.0).unwrap()))
            } else {
                Ok(None)
            }
        } else {
            Err(())
        }
    }
}
