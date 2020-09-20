use crate::redust::event_handler::EventHandler;

fn _publish(cli: &mut redis::Connection, handle: &EventHandler, msg: &str) -> Result<i32, ()> {
    use redis::*;

    Ok(cli
        .publish::<&str, &str, i32>(handle.get_event().get_name(), msg)
        .unwrap())
}

fn _receive(ps: &mut redis::PubSub, handle: &EventHandler) -> String {
    use redis::*;
    PubSub::subscribe(ps, handle.get_event().get_name()).unwrap();

    let msg = ps.get_message().unwrap();

    format!(
        "ch : {} ;; msg : {}",
        msg.get_channel_name(),
        msg.get_payload::<String>().unwrap()
    )
}

#[test]
fn test_publish() {
    use crate::redust::iredisclient::IRedisClient;
    use crate::redust::Event;
    let ev = Event::new("master", "left");
    let handle = EventHandler::new(&ev, &|_| todo!());
    let sp = super::s_meta_provider::ServiceMetaProvider::provide("127.0.0.1");
    let _num_received = _publish(&mut sp.get_conn(), &handle, "hello, world!").unwrap();
}

#[test]
fn test_receive() {
    use crate::redust::iredisclient::IRedisClient;
    use crate::redust::Event;
    let ev = Event::new("master", "joined");
    let handle = EventHandler::new(&ev, &|_| todo!());
    let sp = super::s_meta_provider::ServiceMetaProvider::provide("127.0.0.1");
    let response = _receive(&mut sp.get_conn().as_pubsub(), &handle);
    println!("{}", response);
}
