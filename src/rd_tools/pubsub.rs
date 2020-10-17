use redis::*;
pub fn receive(mut conn: Connection, chs: Vec<String>, action: impl Fn(RedisResult<Msg>)) {
    let mut ps = conn.as_pubsub();
    ps.subscribe(chs).unwrap();
    loop {
        (action)(ps.get_message());
    }
}

pub fn publish<T: redis::ToRedisArgs>(
    mut conn: Connection,
    ch_name: &str,
    msg: T,
) -> RedisResult<i32> {
    conn.publish::<&str, T, i32>(ch_name, msg)
}

pub async fn receive_async(conn: Connection, chs: Vec<String>, action: impl Fn(RedisResult<Msg>)) {
    receive(conn, chs, action)
}

pub async fn publish_async<T: redis::ToRedisArgs>(
    conn: Connection,
    ch_name: &str,
    msg: T,
) -> RedisResult<i32> {
    publish(conn, ch_name, msg)
}
