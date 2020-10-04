use redusty::communication::IRespond;
use redusty::service::*;

fn main() {
    let master_service = Service::open(
        "127.0.0.1",
        "master",
        &["service_joined", "service_left"],
        &["online_services", "test"],
        &[],
    );

    match master_service {
        Ok(serv) => {
            println!("{}", serv.to_string());

            let rcv = serv.receiver();
            println!("listening on endpoints...");
            rcv.receive_endpoints(|endp, sender, payl| {
                println!("received on '{}' --->  {}", endp.to_string(), payl);

                if let Ok(_) = endp.respond(
                    &rcv,
                    &sender,
                    serde_json::Value::String(format!(
                        "welcome to the '{}' endpoint, dear sender; '{}'",
                        endp.to_string(),
                        sender.get_token(),
                    )),
                ) {
                    println!("responded!");
                } else {
                    println!("not responded!");
                }
            });
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}
