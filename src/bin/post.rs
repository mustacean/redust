use redusty::service::Service;

fn main() {
    let user_service = Service::open("127.0.0.1", "user_service", &[], &[], &[]).unwrap();

    use redusty::communication::IPost;

    let sd = user_service.sender();

    let ep_1 = user_service.master_endpoint("online_services");

    let ep_2 = user_service.master_endpoint("test");

    println!("response_1 : {:?}", ep_1.post_str(sd, "msg-1"));

    println!("response_2 : {:?}", ep_2.post_str(sd, "message-2"));
}
