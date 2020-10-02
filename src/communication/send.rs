use crate::rd_tools::IRedisClient;
use crate::service::Event;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sender {
    client: Rc<Box<redis::Client>>,
    events: Rc<Box<Vec<Event>>>,
}

impl<'t> IRedisClient for Sender {
    fn get_client_rc(&self) -> std::rc::Rc<Box<redis::Client>> {
        self.client.clone()
    }
    fn get_client(&self) -> &redis::Client {
        &self.client
    }
    fn get_conn(&self) -> redis::Connection {
        self.get_client().get_connection().unwrap()
    }
}

impl Sender {
    pub fn new(host: &str, events: Option<Vec<Event>>) -> Sender {
        Sender {
            client: if let Ok(x) = redis::Client::open(String::from("redis://") + host) {
                Rc::new(Box::new(x))
            } else {
                panic!("ERROR; server unreachable!")
            },
            events: Rc::new(Box::new(events.unwrap())),
        }
    }
    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }
}
