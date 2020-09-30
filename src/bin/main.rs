fn main() {
    let user_service = open_service(
        "127.0.0.1",
        "user_service",
        &["user.added", "user.removed"],
        &["master/get_online_service_list", "master/"],
        &["master.service_onlined", "master.service_left"],
    );

    match user_service {
        Ok(serv) => {
            println!("{}", serv.to_string());
        }
        _ => {
            println!("sorry, something went wrong.");
        }
    };
}

use redusty::service::*;
