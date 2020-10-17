use redust::communication::{invoke, post};
use redust::service::Endpoint;
use redust::service::Event;
use redust::service::Service;

fn main() {
    let user_service = Service::new(
        "mca",
        "127.0.0.1",
        vec![Event::from_str("mca.born")],
        vec![],
        vec![],
    );
    println!("{}", user_service.to_string());
    let sv_manage = Service::open(
        Some("master".to_owned()), /*parent_token*/
        user_service,
    )
    .unwrap();

    let sd = sv_manage.sender();
    let ep_ = Endpoint::from_str(&std::env::args().last().unwrap());

    invoke(
        &Event::from_str("mca.born"),
        &sd,
        serde_json::Value::String("hello".to_owned()),
    )
    .unwrap();

    println!(
        "response : {:?}",
        post(&ep_, &sd, serde_json::Value::String(format!("ping")))
    );
}
