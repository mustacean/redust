use redusty::communication::IRespond;
use redusty::service::*;

fn main() {
    let serv = Service::open(
        "127.0.0.1",
        "master",
        &["service_joined", "service_left"],
        &["online_services", "test"],
        &[],
    )
    .unwrap();

    println!("{}", serv.to_string());

    let rcv = serv.receiver();
    println!("listening on endpoints...");
    rcv.receive_endpoints(|endp, sender, payl| {
        println!("received on '{}' --->  {}", endp.to_string(), payl);

        let rez = endp.respond(
            &rcv,
            &sender,
            serde_json::Value::String(format!(
                "welcome to the '{}' endpoint, dear sender; '{}'",
                endp.to_string(),
                sender.get_token(),
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
