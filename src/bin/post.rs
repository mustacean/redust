use redusty::service::Service;

fn main() {
    let user_service = Service::open("127.0.0.1", "user_service", &[], &[], &[]).unwrap();

    use redusty::communication::IPost;

    let sd = user_service.sender();

    let ep_1 = user_service.master_endpoint("online_services");

    let ep_2 = user_service.master_endpoint("test");

    for i in 1..5 {
        if i % 2 == 0 {
            println!(
                "response_{} : {:?}",
                i,
                ep_1.post(sd, serde_json::Value::String(format!("msg-{}", i)))
                    .unwrap()
                    .unwrap()
            );
            continue;
        }
        println!(
            "response_{} : {:?}",
            i,
            ep_2.post(
                &sd.clone(),
                serde_json::Value::String(format!("message-{}", i))
            )
            .unwrap()
            .unwrap()
        );
    }
}

/*
response_1 : Object({"from": String("cccb3200-f115-4d57-a557-f82e9b61bbfb"), "payload": String("welcome to the \'master/test\' endpoint, dear sender; \'\"dcadbf39-2732-4520-85b6-71b286f06b2b\"\'"), "to": String("\"dcadbf39-2732-4520-85b6-71b286f06b2b\"")})
response_2 : Object({"from": String("cccb3200-f115-4d57-a557-f82e9b61bbfb"), "payload": String("welcome to the \'master/online_services\' endpoint, dear sender; \'\"a6eb7ea1-f4e8-4015-b523-ecb05c9bb214\"\'"), "to": String("\"a6eb7ea1-f4e8-4015-b523-ecb05c9bb214\"")})
response_3 : Object({"from": String("cccb3200-f115-4d57-a557-f82e9b61bbfb"), "payload": String("welcome to the \'master/test\' endpoint, dear sender; \'\"c11f53d4-5340-43c6-bc23-5c8eecff493c\"\'"), "to": String("\"c11f53d4-5340-43c6-bc23-5c8eecff493c\"")})
response_4 : Object({"from": String("cccb3200-f115-4d57-a557-f82e9b61bbfb"), "payload": String("welcome to the \'master/online_services\' endpoint, dear sender; \'\"a6eb7ea1-f4e8-4015-b523-ecb05c9bb214\"\'"), "to": String("\"a6eb7ea1-f4e8-4015-b523-ecb05c9bb214\"")})

*/
