use crate::service::Service;

impl Service {
    pub fn service_to_json(&self) -> serde_json::Value {
        let sv_nm = self.name().to_owned();
        let sv_host = self.host().to_owned();
        let sv_token = self.token().to_owned();
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

    pub fn to_string(&self) -> String {
        let val = self.service_to_json();
        serde_json::to_string(&val).unwrap()
    }

    pub fn json_to_service(j_val: serde_json::Value) -> Service {
        use super::{Endpoint, Event};
        let obj = j_val.as_object().unwrap();
        let name = obj.get("name").unwrap().as_str().unwrap();
        let host = obj.get("host").unwrap().as_str().unwrap();

        let events: Vec<Event> = obj
            .get("events")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|x| Event::from_str(&serde_json::to_string(x).unwrap()))
            .collect();
        let endpoints: Vec<Endpoint> = obj
            .get("endpoints")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|x| Endpoint::from_str(&serde_json::to_string(x).unwrap()))
            .collect();
        let subscriptions: Vec<Event> = obj
            .get("subscriptions")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|x| Event::from_str(&serde_json::to_string(x).unwrap()))
            .collect();

        Service::new(name, host, events, endpoints, subscriptions)
    }

    pub fn token_to_service(sender: &crate::communication::Sender, token: &str) -> Service {
        use super::Endpoint;
        use crate::communication::IPost;
        let ep = Endpoint::from_str(&format!("{}/#", token));

        let r = ep.post(sender, serde_json::Value::String("ping".to_owned()));
        Service::json_to_service(r.unwrap().unwrap())
    }
}
