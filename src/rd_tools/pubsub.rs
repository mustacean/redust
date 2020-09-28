use redis::*;
pub fn receive(
    mut conn: Connection,
    chs: &'static [&str],
    action: Box<dyn Fn(RedisResult<Msg>) + Send + Sync>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut ps = conn.as_pubsub();
        ps.subscribe(chs).unwrap();
        loop {
            (action)(ps.get_message());
        }
    })
}

pub fn publish<T: redis::ToRedisArgs>(
    mut conn: Connection,
    ch_name: &str,
    msg: T,
) -> RedisResult<i32> {
    conn.publish::<&str, T, i32>(ch_name, msg)
}

fn get_default_con() -> Connection {
    redis::Client::open("redis://127.0.0.1/")
        .unwrap()
        .get_connection()
        .unwrap()
}

#[test]
fn test_pub() {
    assert_ne!(
        publish(get_default_con(), "test", "hello, world!").unwrap(),
        0
    )
}

#[test]
fn test_recv() {
    receive(
        get_default_con(),
        &["test", "mest"],
        Box::new(|e| {
            println!("{:?}", e);
        }),
    )
    .join()
    .unwrap();
}
