use redust::communication::IRespond;
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
        vec![],
    );

    let s_manager = Service::open(None, service).unwrap();

    println!("{}", s_manager.service().to_string());

    let rcv = s_manager.receiver();
    println!("listening on endpoints...");
    rcv.receive_endpoints(|endp, sender, payl| {
        println!("received on '{}' --->  {}", endp.to_string(), payl);

        let rez = endp.respond(
            &rcv,
            sender,
            serde_json::Value::String(format!(
                "welcome to the '{}' endpoint, dear sender; '{}'",
                endp.to_string(),
                sender.token(),
            )),
        );
        if let Ok(_) = rez {
            println!("succesfully responded!");
        } else {
            println!("error, not responded!");
        }
        rez
    });
}
