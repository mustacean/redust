use redusty::service::*;

fn main() {
    let master_service = Service::open(
        "127.0.0.1",
        "master",
        &[],
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

                serde_json::Value::String(format!(
                    "welcome to the '{}' endpoint, dear sender; '{}'",
                    endp.to_string(),
                    sender.get_token(),
                ))
            });
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}
