use crate::redust::event::Event;
use crate::redust::service::Service;

impl Service {
    pub fn get_caster<'t>(&'t mut self) -> Caster<'t> {
        Caster::<'t> { service: self }
    }

    pub fn get_antenna<'t>(&'t self, subscriptions: &'t Vec<Event>) -> Antenna<'t> {
        Antenna::<'t> {
            service: self,
            subscriptions,
        }
    }
}

pub struct Caster<'t> {
    service: &'t Service,
}

pub struct Antenna<'t> {
    service: &'t Service,
    subscriptions: &'t Vec<Event>,
}

impl<'t> Caster<'t> {
    // pub fn new(service: &'t Service) -> Caster<'t> {
    //     Caster::<'t> { service }
    // }

    pub fn invoke(&'t self, e: &str, msg: &str) -> Result<i32, ()> {
        let mut exist = false;

        for i in self.service.get_events() {
            if e == i.get_name() {
                exist = true;
                break;
            }
        }
        if !exist {
            println!("this service doesn't contain the specified event.");
            return Err(());
        }
        use super::iredisclient::IRedisClient;
        use redis::*;
        Ok(self
            .service
            .get_provider()
            .get_conn()
            .publish::<&str, &str, i32>(e, msg)
            .unwrap())
    }
}

impl<'t> Antenna<'t> {
    // pub fn new(service: &'t Service, subscriptions: &'t Vec<Event>) -> Antenna<'t> {
    //     Antenna::<'t> {
    //         service,
    //         subscriptions,
    //     }
    // }
    pub fn receive(&'t self) -> Result<String, ()> {
        use super::iredisclient::IRedisClient;
        let mut conn = self.service.get_provider().get_conn();
        let mut pubsub = conn.as_pubsub();
        if let Ok(_) = pubsub.subscribe(self.subscriptions.first().unwrap().get_name()) {
            // --- async part
            let msg = pubsub.get_message().unwrap();
            Ok(format!(
                "ch : {} ;; msg : {}",
                msg.get_channel_name(),
                msg.get_payload::<String>().unwrap()
            ))
        // --- sync part
        } else {
            Err(())
        }
    }
}

#[test]
fn test_antenna() {
    let mca_service = super::ServiceMetaProvider::provide("127.0.0.1")
        .get_service("mca_service")
        .unwrap();

    let other_service = super::ServiceMetaProvider::provide("127.0.0.1")
        .get_service("master")
        .unwrap();

    //let mut my_antenna = Antenna::new(&mca_service, other_service.get_events());
    let my_antenna = mca_service.get_antenna(other_service.get_events());

    let result = my_antenna.receive();

    println!("received a message --> {}", result.unwrap());
}

#[test]
fn test_cast() {
    let mut mca_service = super::ServiceMetaProvider::provide("127.0.0.1")
        .get_service("mca_service")
        .unwrap();

    //let my_caster = Caster::new(&mca_service);
    let my_caster = mca_service.get_caster();

    let msg = "hello, world!";

    let result = my_caster.invoke("my_birth", msg);

    println!("published a message --> {}", result.unwrap());
}



// struct EventMetaProvider;

// struct EventLogs {
//     provider: std::rc::Rc<Box<EventMetaProvider>>,
// }

// impl EventLogs {
//     pub fn add(eve: &Event, exp: u32) {
//         todo!()
//     }
// }

// struct ApiEndPoint<'t> {
//     trlo: &'t EventLogs,
// }
