#[test]
fn test_get_service() {
    let sp = super::ServiceMetaProvider::provide("127.0.0.1");
    let x = sp.get_service("mca_service").unwrap();
    assert_eq!(x.get_name(), "mca_service");
}

#[test]
fn test_get_services() {
    let sp = super::ServiceMetaProvider::provide("127.0.0.1");
    assert_ne!(sp.get_services().unwrap().len(), 0);
}

#[test]
fn test_add_service() {
    let sp = super::ServiceMetaProvider::provide("127.0.0.1");
    let _ = sp
        .add_service(
            "mca_service",
            "127.0.0.1",
            vec![&String::from("2pac_murder"), &String::from("my_birth")],
        )
        .unwrap();
    assert_eq!(true, true);
}

#[test]
fn test_remove_service() {
    let sp = super::ServiceMetaProvider::provide("127.0.0.1");
    sp.remove_service("mca_service").unwrap();
    assert_eq!(true, true);
}

#[test]
fn test_get_events() {
    let sp = super::ServiceMetaProvider::provide("127.0.0.1");
    let x = sp.get_events("mca_service").unwrap();
    assert_ne!(x.len(), 0);
}

#[test]
fn test_add_events() {
    let sp = super::ServiceMetaProvider::provide("127.0.0.1");
    let _ = sp
        .add_events(
            "mca_service",
            vec![&String::from("2pac_murder"), &String::from("my_birth")],
        )
        .unwrap();
    assert_eq!(true, true);
}

#[test]
fn test_remove_events() {
    let sp = super::ServiceMetaProvider::provide("127.0.0.1");
    let _ = sp.remove_events("mca_service").unwrap();
    assert_eq!(true, true);
}
