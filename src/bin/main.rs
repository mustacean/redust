fn main() {
    use redusty::communication::{IInvoker, IPost};
    use redusty::service::*;

    let user_service = Service::open(
        "127.0.0.1",
        "user_service",
        &["added", "removed"],
        &[("master", "get_online_service_list")],
        &[
            ("master", "service_onlined"),
            ("master", "service_left"),
            ("master", "service_updated"),
        ],
    );

    match user_service {
        Ok(serv) => {
            print!("----");
            for e in serv.events().unwrap() {
                print!("{}", e.to_string());

                println!(
                    " --> online subscribers : {}",
                    e.invoke(serv.sender(), "test").unwrap()
                )
            }
            println!("\nEndpoints to accept: ");
            for e in serv.endpoints().unwrap() {
                print!("{}", e.to_string());
                let arg = "hello, world!";
                println!(
                    "---> The message we sent to the function : '{}' ; The result : '{}'",
                    arg,
                    e.post(serv.sender(), arg).unwrap()
                )
            }
            println!("\nEvents subscribed: ");
            for e in serv.subscriptions().unwrap() {
                print!("{}", e.to_string());
                println!(
                    " --> online subscribers : {}",
                    e.invoke(serv.sender(), "test").unwrap()
                )
            }
            print!("----\n");
        }
        _ => {
            println!("sorry, something went wrong.");
        }
    };
}

/* OUTPUT:

----user_service.added --> online subscribers : 0
user_service.removed --> online subscribers : 0

Endpoints to accept:
master/get_online_service_list---> The message we sent to the function : 'hello, world!' ; The result : 'hello, world!'

Events subscribed:
master.service_onlined --> online subscribers : 0
master.service_left --> online subscribers : 0
master.service_updated --> online subscribers : 0
*/
