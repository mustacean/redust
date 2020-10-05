use crate::communication::Receiver;

pub fn get_meta_info(r: &Receiver) -> serde_json::Value {
    let sv_nm = r.service_name().to_owned();
    let sv_host = r.host().to_owned();
    let sv_token = r.sender().get_token().to_owned();
    use std::iter::*;
    let evts: Vec<serde_json::Value> = r
        .sender()
        .event_names()
        .iter()
        .map(|x| serde_json::Value::String(x.to_owned()))
        .collect();

    let endps: Vec<serde_json::Value> = r
        .endpoint_names()
        .iter()
        .map(|x| serde_json::Value::String(x.to_owned()))
        .collect();
    let subs: Vec<serde_json::Value> = r
        .antenna()
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
