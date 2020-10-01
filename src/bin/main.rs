use redusty::service::*;

fn main() {
    let user_service = Service::open(
        "127.0.0.1",
        "user_service",
        &["added", "removed"],
        &[
            ("user_service", "online_services"),
            ("user_service", "test"),
        ],
        &[
            ("master", "service_joined"),
            ("master", "service_left"),
            ("master", "service_killed"),
        ],
    );

    match user_service {
        Ok(serv) => {
            //let (s, r) = std::sync::mpsc::sync_channel::<u32>(4);

            //let evx = s.clone();
            print_out(&serv);

            let rcv = serv.receiver();
            rcv.receive_events(Box::new(move |event, arg| {
                //evx.send(1).unwrap();
                println!("[{:?}] invoked -- > {}", event, arg);
            }));
            //let enx = s.clone();
            rcv.receive_endpoints(Box::new(move |ep, arg| {
                //enx.send(2).unwrap();
                println!("['{:?}'] called -- > {}", ep, arg);
            }));

            // std::thread::spawn(move || loop {
            //     let mut x = String::new();

            //     std::io::stdin().read_line(&mut x).unwrap();
            //     print!("{}", x);
            // });
            loop {
                //println!("opcode : {}", r.recv().unwrap());
                //print_out(&serv);
            }
        }
        _ => {
            println!("sorry, something went wrong.");
        }
    };
}

fn print_out(serv: &Service) {
    //use redusty::communication::{IInvoker, IPost};
    print!("----");
    println!("Events owned: ");
    for e in serv.events().unwrap() {
        print!("{},  ", e.to_string());

        // println!(
        //     " --> online subscribers : {}",
        //     e.invoke(serv.sender(), "test").unwrap()
        // )
    }
    println!("\nEndpoints to accept: ");
    for e in serv.endpoints().unwrap() {
        print!("{},  ", e.to_string());

        //let arg = "hello, world!";
        // println!(
        //     "---> The message we sent to the function : '{}' ; The result : '{}'",
        //     arg,
        //     e.post(serv.sender(), arg).unwrap()
        // )
    }
    println!("\nEvents subscribed: ");
    for e in serv.subscriptions().unwrap() {
        print!("{},  ", e.to_string());
        // when commented out the code block below that invokes subsriptions which
        // is already a stupid idea :-) it goes nut infinitely X)

        // println!(
        //     " --> online subscribers : {}",
        //     e.invoke(serv.sender(), "test").unwrap()
        // )
    }
    print!("----\n");
}
