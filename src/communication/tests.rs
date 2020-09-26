use crate::service::ServiceMetaProvider;

#[test]
fn test_antenna() {
    let spr = ServiceMetaProvider::provide("127.0.0.1");

    let mca_service = spr.clone().get_service("mca_service").unwrap();
    let other_service = spr.clone().get_service("master").unwrap();

    let my_antenna = mca_service.get_antenna(other_service.get_events());

    for (ev, arg) in my_antenna.launch().receive() {
        println!("Received.. \n e : {}\n arg : {}", ev.get_name(), arg);
    }
}

#[test]
fn test_cast() {
    let spr = ServiceMetaProvider::provide("127.0.0.1");
    let mca_service = spr.clone().get_service("mca_service").unwrap();

    let my_caster = mca_service.get_caster();

    let me_event = &mca_service.get_events()[0];
    let msg = "hello, world!";

    let result = my_caster.invoke(me_event, msg).unwrap();
    let res = my_caster.invoke(me_event, msg).unwrap();

    assert_ne!(res + result, 0);
}

#[test]
fn test_send_envoy() {
    let spr = ServiceMetaProvider::provide("127.0.0.1");
    let mca_service = spr.clone().get_service("mca_service").unwrap();
    let caster = mca_service.get_caster();
    caster
        .prepare_packets(&mca_service.get_events()[1])
        .send(
            "hello, world!, today it's the day that 
            I be blowin' up like a bubble."
                .as_bytes(),
        )
        .unwrap();
}

#[test]

fn test_receive_envoy() {
    let spr = ServiceMetaProvider::provide("127.0.0.1");
    let mca_service = spr.clone().get_service("mca_service").unwrap();
    let ante = mca_service.get_antenna(mca_service.get_events());
    ante.launch();
    for (_, id) in ante.receive() {
        ante.receive_packets(id, 3, std::io::stdout());
    }
}
