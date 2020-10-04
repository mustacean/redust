use redusty::communication::IPost;
use redusty::service::Service;

fn main() {
    let user_service = Service::open("127.0.0.1", "user_service", &[], &[], &[]).unwrap();
    let sd = user_service.sender();
    let ep_ = user_service.master_endpoint("");

    println!(
        "response : {:?}",
        ep_.post(&sd, serde_json::Value::String(format!("ping")))
    );
}
