use crate::rd_tools::IRedisClient;
use crate::service::Event;

pub struct Sender {
    client: std::rc::Rc<Box<redis::Client>>,
    events: Option<Vec<Event>>,
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
        fn get_redis_client(h: &str) -> Result<std::rc::Rc<Box<redis::Client>>, String> {
            if let Ok(x) = redis::Client::open(String::from("redis://") + h) {
                Ok(std::rc::Rc::new(Box::new(x)))
            } else {
                panic!("ERROR; server unreachable!")
            }
        }

        if let Ok(r) = get_redis_client(host) {
            Sender {
                client: r.clone(),
                events,
            }
        } else {
            panic!("pandic!")
        }
    }
    pub fn get_events(&self) -> Option<&Vec<Event>> {
        self.events.as_ref()
    }
}
