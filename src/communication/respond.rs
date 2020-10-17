use crate::components::Receiver;
use serde_json::Value;

pub fn respond(recv: &Receiver, token: &str, response_payload: Value) -> Result<i32, ()> {
    let msg = super::formats::tokenize_response(&recv.sender().token(), token, response_payload);
    crate::rd_tools::rpush_str(recv.sender().get_conn(), token, &msg)
}
