// > antenna.rs
pub fn deserialize_event_args(msg: &redis::Msg) -> (crate::service::Event, serde_json::Value) {
    let ch = msg.get_channel::<String>().unwrap();
    let msg = msg.get_payload::<String>().unwrap();
    (
        crate::service::Event::from_str(&ch),
        serde_json::from_str(&msg).expect("arguments couldn't be resolved :("),
    )
}

// > invoke.rs
pub fn serialize_event_args(payload: &serde_json::Value) -> String {
    serde_json::to_string(payload).expect("arguments couldn't be serialized :(")
}

// > post.rs
pub fn serialize_request(token: &str, payload: serde_json::Value) -> String {
    let mut mp = serde_json::Map::new();

    mp.insert(
        "token".to_owned(),
        serde_json::Value::String(token.to_owned()),
    );
    mp.insert("payload".to_owned(), payload);

    serde_json::to_string(&serde_json::Value::Object(mp))
        .expect("request couldn't be serialized :(")
}

// > receiver.rs
pub fn deserialize_request(msg: &str) -> (String, serde_json::Value) {
    let val: serde_json::Value =
        serde_json::from_str(&msg).expect("request couldn't be resolved :(");

    let token = &val["token"];
    let payload = &val["payload"];

    (token.to_string(), payload.clone())
}

// > respond.rs
pub fn serialize_response(
    from_token: &str,
    to_token: &str,
    response_payload: serde_json::Value,
) -> String {
    let mut mp = serde_json::Map::new();
    mp.insert(
        "from".to_owned(),
        serde_json::Value::String(from_token.to_owned()),
    );
    mp.insert(
        "to".to_owned(),
        serde_json::Value::String(to_token.to_owned()),
    );
    mp.insert("payload".to_owned(), response_payload);

    serde_json::to_string(&serde_json::Value::Object(mp))
        .expect("response couldn't be serialized :(")
}
