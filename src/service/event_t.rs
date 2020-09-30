#[derive(Debug)]
pub struct Event {
    owner: String,
    name: String,
}

pub fn new_event(owner: &str, name: &str) -> Event {
    Event {
        owner: owner.to_owned(),
        name: name.to_owned(),
    }
}

#[test]
fn test_invoker() {
    use crate::communication::IInvoker;
    let cn = redis::Client::open("redis://127.0.0.1/").unwrap();

    assert_ne!(
        new_event("mca", "rised")
            .invoke_with_conn(&cn, "whats' upp broooooo!")
            .unwrap(),
        0
    )
}

impl Clone for Event {
    fn clone(&self) -> Event {
        Event {
            name: self.name.clone(),
            owner: self.owner.clone(),
        }
    }
}
impl Event {
    pub fn get_owner(&self) -> &str {
        &self.owner
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.owner, self.name)
    }
}
