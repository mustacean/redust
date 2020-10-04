use crate::rd_tools::IRedisClient;
use crate::service::Event;
use std::rc::Rc;

pub struct Sender {
    client: Rc<Box<redis::Client>>,
    events: Rc<Box<Vec<Event>>>,
    token: String,
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

impl Clone for Sender {
    fn clone(&self) -> Sender {
        Sender {
            client: self.client.clone(),
            events: self.events.clone(),
            token: format!("{}", uuid::Uuid::new_v4()),
        }
    }
}

impl Sender {
    pub fn new(host: &str, events: Vec<Event>) -> Sender {
        Sender {
            client: if let Ok(x) = redis::Client::open(String::from("redis://") + host) {
                Rc::new(Box::new(x))
            } else {
                panic!("ERROR; server unreachable!")
            },
            events: Rc::new(Box::new(events)),
            token: format!("{}", uuid::Uuid::new_v4()),
        }
    }

    pub fn clone_from_token(&self, token: &String) -> Sender {
        let mut sn = self.clone();
        sn.token = token.to_owned();
        return sn;
    }

    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }
    pub fn event_names(&self) -> Vec<String> {
        self.events()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }
}
