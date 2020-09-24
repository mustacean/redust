use crate::rd_tools::IRedisClient;
use crate::service::{Event, Service};
use std::cell::Cell;

pub struct Antenna<'t> {
    service: &'t Service,
    subscriptions: &'t Vec<Event>,
    event_pipeline: (
        std::sync::mpsc::Sender<(Event, String)>,
        std::sync::mpsc::Receiver<(Event, String)>,
    ),
    is_launched: Cell<bool>,
}

impl<'t> Antenna<'t> {
    pub fn launch(&self) -> &Antenna {
        if !self.is_launched.get() {
            for ev in self.subscriptions {
                let sender = self.event_pipeline.0.clone();
                let event_arg = Event::new(ev.get_owner(), ev.get_name());
                let mut conn = self.service.get_provider().get_conn();
                std::thread::spawn(move || {
                    let mut pubsub = conn.as_pubsub();
                    if let Ok(_) = pubsub.subscribe(event_arg.to_string()) {
                        loop {
                            let msg = pubsub.get_message().unwrap();
                            sender
                                .send((event_arg.clone(), msg.get_payload().unwrap()))
                                .unwrap();
                        }
                    }
                });
            }
            self.is_launched.set(true);
        }
        return self;
    }

    pub fn receive(&self) -> std::sync::mpsc::Iter<(Event, String)> {
        if !self.is_launched.get() {
            panic!("antenna not launched!");
        }
        self.event_pipeline.1.iter()
    }
}

impl Service {
    pub fn get_antenna<'t>(&'t self, subscriptions: &'t Vec<Event>) -> Antenna<'t> {
        Antenna::<'t> {
            service: self,
            subscriptions,
            event_pipeline: std::sync::mpsc::channel(),
            is_launched: Cell::new(false),
        }
    }
}
