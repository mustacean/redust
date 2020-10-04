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
            rcv.receive_endpoints(|endp, sender, payl| {
                println!("received on '{}' --->  {}", endp.to_string(), payl);

                serde_json::Value::String(format!(
                    "welcome to the '{}' endpoint, dear sender; '{}'",
                    endp.to_string(),
                    sender.get_token(),
                ))
            });
        }
        _ => {
            println!("sorry, something went wrong.");
        }
    };
}

/*

listening on endpoints...
received on 'master/test' --->  "message-1"
received on 'master/online_services' --->  "msg-2"
received on 'master/test' --->  "message-3"
received on 'master/online_services' --->  "msg-4"
received on 'master/test' --->  "message-1"
received on 'master/online_services' --->  "msg-2"
received on 'master/test' --->  "message-3"
received on 'master/online_services' --->  "msg-4"

*/
