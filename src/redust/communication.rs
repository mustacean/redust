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
            event_pipeline: std::sync::mpsc::channel(),
        }
    }
}

pub struct Caster<'t> {
    service: &'t Service,
}

pub struct Antenna<'t> {
    service: &'t Service,
    subscriptions: &'t Vec<Event>,
    event_pipeline: (
        std::sync::mpsc::Sender<(Event, String)>,
        std::sync::mpsc::Receiver<(Event, String)>,
    ),
}

impl<'t> Caster<'t> {
    // pub fn new(service: &'t Service) -> Caster<'t> {
    //     Caster::<'t> { service }
    // }

    pub fn invoke(&self, e: &str, msg: &str) -> Result<i32, ()> {
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
    pub fn receive(&self) {
        use super::iredisclient::IRedisClient;

        for ev in self.subscriptions {
            let sender = self.event_pipeline.0.clone();
            let event_arg = Event::new(ev.get_owner(), ev.get_name());
            let mut conn = self.service.get_provider().get_conn();
            std::thread::spawn(move || {
                let mut pubsub = conn.as_pubsub();
                if let Ok(_) = pubsub.subscribe(event_arg.get_name()) {
                    let msg = pubsub.get_message().unwrap();
                    sender
                        .send((event_arg, msg.get_payload().unwrap()))
                        .unwrap();
                }
            });
        }
        loop {
            let (ev, arg) = self.event_pipeline.1.recv().unwrap();
            println!("Received.. \n e : {}\n arg : {}", ev.get_name(), arg);
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

    let my_antenna = mca_service.get_antenna(other_service.get_events());
    my_antenna.receive();
}

#[test]
fn test_cast() {
    let mut mca_service = super::ServiceMetaProvider::provide("127.0.0.1")
        .get_service("mca_service")
        .unwrap();

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
