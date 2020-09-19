pub struct Event {
    owner: String,
    name: String,
}

impl Event {
    pub fn new(owner: &str, name: &str) -> Event {
        Event {
            owner: owner.to_owned(),
            name: name.to_owned(),
        }
    }

    pub fn get_owner(&self) -> &str {
        &self.owner
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
