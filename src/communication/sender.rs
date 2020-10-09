use crate::rd_tools::IRedisClient;
use crate::service::Event;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sender {
    client: Rc<redis::Client>,
    events: Rc<Vec<Event>>,
    token: Rc<String>,
}

impl IRedisClient for Sender {
    fn get_client_rc(&self) -> std::rc::Rc<redis::Client> {
        self.client.clone()
    }
    fn get_client(&self) -> &redis::Client {
        &self.client
    }
    fn get_conn(&self) -> redis::Connection {
        self.get_client().get_connection().unwrap()
    }
}

// impl Clone for Sender {
//     fn clone(&self) -> Sender {
//         Sender {
//             client: self.client.clone(),
//             events: self.events.clone(),
//             token: self.token.clone(),
//         }
//     }
// }

impl Sender {
    pub fn new(host: &str, events: Vec<Event>) -> Sender {
        Sender {
            client: if let Ok(x) = redis::Client::open(String::from("redis://") + host) {
                Rc::new(x)
            } else {
                panic!("ERROR; server unreachable!")
            },
            events: Rc::new(events),
            token: Rc::new(format!("{}", uuid::Uuid::new_v4())),
        }
    }

    pub fn set_token(&self) {
        todo!()
    }
    pub fn clone_from_token(&self, token: String) -> Sender {
        let mut sn = self.clone();
        sn.token = Rc::new(token);
        return sn;
    }
    pub fn get_token(&self) -> &str {
        &self.token
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
}
