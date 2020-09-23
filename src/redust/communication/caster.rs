use crate::redust::rd_tools::IRedisClient;
use crate::redust::service::{Event, Service};

pub struct Caster<'t> {
    service: &'t Service,
}

impl<'t> Caster<'t> {
    // pub fn new(service: &'t Service) -> Caster<'t> {
    //     Caster::<'t> { service }
    // }

    pub fn invoke(&self, e: &Event, msg: &str) -> Result<i32, ()> {
        fn _get_if_contains<'t>(evs: &'t Vec<Event>, e: &'t Event) -> Option<&'t Event> {
            for i in evs {
                if e.get_name() == i.get_name() && e.get_owner() == i.get_owner() {
                    return Some(e);
                }
            }
            None
        }
        if let Some(e) = _get_if_contains(self.service.get_events(), e) {
            use redis::*;
            return Ok(self
                .service
                .get_provider()
                .get_conn()
                .publish::<String, &str, i32>(e.to_string(), msg)
                .unwrap());
        }

        println!("this service doesn't contain the specified event.");
        Err(())
    }
}
impl Service {
    pub fn get_caster<'t>(&'t self) -> Caster<'t> {
        Caster::<'t> { service: self }
    }
}
