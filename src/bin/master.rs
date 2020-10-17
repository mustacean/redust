use redust::service::Endpoint;
use redust::service::Event;
use redust::service::Service;

fn main() {
    let service = Service::new(
        "master",
        "127.0.0.1",
        vec![
            Event::from_str("master.service_joined"),
            Event::from_str("master.service_left"),
        ],
        vec![
            Endpoint::from_str("master/online_services"),
            Endpoint::from_str("master/test"),
        ],
        vec![Event::from_str("mca.born")],
    );

    let s_manager = Service::open(None, service).unwrap();

    println!("{}", s_manager.service().to_string());
    let ant = s_manager.antenna().unwrap();

    ant.receive_events_async(|x, y| {
        println!(
            "received event! : {}  ---> arg : {}",
            x.to_string(),
            y.to_string()
        );
    });

    let rcv = s_manager.receiver().unwrap();
    let fut = rcv.receive_endpoints_async(|endp, sender, payl| {
        println!("received on '{}' --->  {}", endp.to_string(), payl);

        Some(serde_json::Value::String(format!(
            "welcome to the '{}' endpoint, dear sender; '{}'",
            endp.to_string(),
            sender.token(),
        )))
    });

    println!("listening on endpoints...");

    futures::executor::block_on(fut);
}
