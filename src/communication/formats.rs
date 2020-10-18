type Pl = serde_json::Value;
use crate::service::Event;
use redis::Msg;
use serde_json::Map;
use serde_json::{from_str, to_string};

// > invoke.rs
pub fn serialize_event_args(payload: &Pl) -> String {
    to_string(payload).expect("arguments couldn't be serialized :(")
}

// > antenna.rs
pub fn deserialize_event_args(msg: &Msg) -> (Event, Pl) {
    let ch = msg.get_channel::<String>().unwrap();
    let msg = msg.get_payload::<String>().unwrap();
    (
        Event::from_str(&ch),
        from_str(&msg).expect("arguments couldn't be resolved :("),
    )
}

// > post.rs
pub fn serialize_request(token: &str, payload: Pl) -> String {
    let mut mp = Map::new();

    mp.insert("token".to_owned(), Pl::String(token.to_owned()));
    mp.insert("payload".to_owned(), payload);

    to_string(&Pl::Object(mp)).expect("request couldn't be serialized :(")
}

// > post.rs
pub fn serialize_response(response: (String, String)) -> Pl {
    from_str(&response.0).expect("couldn't serialize response :(")
}

// > receiver.rs
pub fn detokenize_request(msg: &str) -> (String, Pl) {
    let val: Pl = from_str(&msg).expect("request couldn't be resolved :(");
    (val["token"].to_string(), val["payload"].clone())
}

// > respond.rs
pub fn tokenize_response(from_token: &str, to_token: &str, response_payload: Pl) -> String {
    let mut mp = Map::new();
    mp.insert("from".to_owned(), Pl::String(from_token.to_owned()));
    mp.insert("to".to_owned(), Pl::String(to_token.to_owned()));
    mp.insert("payload".to_owned(), response_payload);

    to_string(&Pl::Object(mp)).expect("response couldn't be tokenized :(")
}
