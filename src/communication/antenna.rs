use crate::communication::Sender;
use crate::rd_tools::IRedisClient;
use crate::service::Event;
use std::rc::Rc;

#[derive(Clone)]
pub struct Antenna {
    client: Rc<redis::Client>,
    subscriptions: Rc<Vec<Event>>,
}

impl IRedisClient for Antenna {
    fn get_client_rc(&self) -> Rc<redis::Client> {
        self.client.clone()
    }
    fn get_client(&self) -> &redis::Client {
        &self.client
    }
    fn get_conn(&self) -> redis::Connection {
        self.get_client().get_connection().unwrap()
    }
}

impl Antenna {
    pub fn new(sender: &Sender, subscriptions: Vec<Event>) -> Antenna {
        Antenna {
            client: sender.get_client_rc(),
            subscriptions: Rc::new(subscriptions),
        }
    }
    pub fn subsc_names(&self) -> Vec<String> {
        self.subscriptions()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }
    pub fn subscriptions(&self) -> &Vec<Event> {
        self.subscriptions.as_ref()
    }

    pub fn receive_events(&self, action: impl Fn(&Event, &serde_json::Value)) {
        crate::rd_tools::receive(self.get_conn(), self.subsc_names(), |x| {
            let result = x.unwrap();
            let ch = result.get_channel::<String>().unwrap();

            let msg = result.get_payload::<String>().unwrap();

            (action)(&Event::from_str(&ch), &serde_json::from_str(&msg).unwrap());
        })
    }
}
