type Pl = serde_json::Value;
use crate::service::Service;
use serde_json::Map;

impl Service {
    pub fn to_json(&self) -> Pl {
        let sv_nm = self.name().to_owned();
        let sv_host = self.host().to_owned();
        let evts: Vec<Pl> = self
            .event_names()
            .iter()
            .map(|x| Pl::String(x.to_owned()))
            .collect();

        let endps: Vec<Pl> = self
            .endpoint_names()
            .iter()
            .map(|x| Pl::String(x.to_owned()))
            .collect();
        let subs: Vec<Pl> = self
            .subsc_names()
            .iter()
            .map(|x| Pl::String(x.to_owned()))
            .collect();

        let mut mp = Map::new();

        mp.insert("name".to_owned(), Pl::String(sv_nm));
        mp.insert("host".to_owned(), Pl::String(sv_host));

        mp.insert("events".to_owned(), Pl::Array(evts));
        mp.insert("endpoints".to_owned(), Pl::Array(endps));
        mp.insert("subscriptions".to_owned(), Pl::Array(subs));

        Pl::Object(mp)
    }

    pub fn to_string(&self) -> String {
        let val = self.to_json();
        serde_json::to_string(&val).unwrap()
    }

    pub fn from_json(j_val: Pl) -> Service {
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
}
