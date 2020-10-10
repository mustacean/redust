use crate::service::Service;

impl Service {
    pub fn service_to_json(&self) -> serde_json::Value {
        let sv_nm = self.name().to_owned();
        let sv_host = self.host().to_owned();
        let sv_token = self.token().to_owned();
        use std::iter::*;
        let evts: Vec<serde_json::Value> = self
            .event_names()
            .iter()
            .map(|x| serde_json::Value::String(x.to_owned()))
            .collect();

        let endps: Vec<serde_json::Value> = self
            .endpoint_names()
            .iter()
            .map(|x| serde_json::Value::String(x.to_owned()))
            .collect();
        let subs: Vec<serde_json::Value> = self
            .subsc_names()
            .iter()
            .map(|x| serde_json::Value::String(x.to_owned()))
            .collect();

        let mut mp = serde_json::Map::new();

        mp.insert("token".to_owned(), serde_json::Value::String(sv_token));
        mp.insert("name".to_owned(), serde_json::Value::String(sv_nm));
        mp.insert("host".to_owned(), serde_json::Value::String(sv_host));

        mp.insert("events".to_owned(), serde_json::Value::Array(evts));
        mp.insert("endpoints".to_owned(), serde_json::Value::Array(endps));
        mp.insert("subscriptions".to_owned(), serde_json::Value::Array(subs));

        serde_json::Value::Object(mp)
    }

    // pub fn json_to_service(_j_val: serde_json::Value) -> Service {
    //     todo!()
    // }

    pub fn token_to_service(&self, token: &str) -> Service {
        let mut sevo = self.clone();
        sevo.set_token(token);
        sevo
    }
}
