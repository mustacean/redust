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
