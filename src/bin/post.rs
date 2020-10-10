use redust::communication::IPost;
use redust::service::Endpoint;
use redust::service::Service;

fn main() {
    let user_service = Service::new("127.0.0.1", "mca", vec![], vec![], vec![]);

    let sv_manage = Service::open(user_service).unwrap();

    let sd = sv_manage.sender();
    let ep_ = Endpoint::from_str(&std::env::args().last().unwrap());

    println!(
        "response : {:?}",
        ep_.post(&sd, serde_json::Value::String(format!("ping")))
    );
}
