use crate::redust::service::ServiceMetaProvider;

#[test]
fn test_antenna() {
    let spr = ServiceMetaProvider::provide("127.0.0.1");

    let mca_service = spr.clone().get_service("mca_service").unwrap();
    let other_service = spr.clone().get_service("master").unwrap();

    let mut my_antenna = mca_service.get_antenna(other_service.get_events());

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
