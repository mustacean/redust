pub struct Event {
    owner: String,
    name: String,
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
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.owner, self.name)
    }
}
