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

impl Clone for Event {
    fn clone(&self) -> Event {
        Event {
            name: self.name.clone(),
            owner: self.owner.clone(),
        }
    }
}
impl Event {
    pub fn owner(&self) -> &str {
        &self.owner
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.owner, self.name)
    }
    pub fn from_str(st: &str) -> Event {
        let sas: Vec<_> = st.split(".").collect();

        if sas.len() == 2 {
            new_event(sas[0], sas[1])
        } else {
            panic!("wrong format!");
        }
    }
}
