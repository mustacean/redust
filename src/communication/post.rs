use crate::components::Sender;
use crate::service::Endpoint;

pub fn post(
    endp: &Endpoint,
    sender: &Sender,
    payload: serde_json::Value,
) -> Result<Option<serde_json::Value>, ()> {
    let msg = super::formats::serialize_request(&sender.token(), payload);

    crate::rd_tools::rpush_str(sender.get_conn(), &endp.to_string(), &msg)
        .expect("couldn't posted!");
    if let Ok(result) =
        crate::rd_tools::blpop_str(sender.get_conn(), &format!("\"{}\"", sender.token()), 1)
    {
        Ok(Some(super::formats::serialize_response(result)))
    } else {
        Ok(None)
    }
}
