use crate::components::Sender;
use crate::service::Event;
use serde_json::Value;

pub fn invoke(ev: &Event, sender: &Sender, payload: Value) -> Result<i32, ()> {
    match crate::rd_tools::publish(
        sender.get_conn(),
        &ev.to_string(),
        super::formats::serialize_event_args(&payload),
    ) {
        Ok(x) => Ok(x),
        _ => Err(()),
    }
}

pub async fn invoke_async(
    conn: redis::Connection,
    payload: &str,
) -> Result<i32, redis::RedisError> {
    crate::rd_tools::publish_async(conn, "test", payload).await
}

#[test]
fn test_async_invoking() {
    let redo_con = redis::Client::open("redis://127.0.0.1/").unwrap();
    let me = async {
        for _ in 0..5 {
            println!(
                "sus lan : {}",
                invoke_async(redo_con.get_connection().unwrap(), "evqweqweq")
                    .await
                    .unwrap()
            )
        }
    };
    futures::executor::block_on(me);
}
