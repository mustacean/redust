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
            let rcv = serv.receiver();
            println!("listening on endpoints...");
            rcv.receive_endpoints(|ep, arg| {
                use redusty::communication::ResponseType;
                println!("received on '{}' --->  {}", ep.to_string(), arg);

                ResponseType::StringResponse(format!(
                    "welcome to the '{}' endpoint.",
                    ep.to_string(),
                ))
            });
        }
        _ => {
            println!("sorry, something went wrong.");
        }
    };
}
