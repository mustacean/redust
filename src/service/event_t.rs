#[derive(Debug, Clone)]
pub struct Event {
    owner: String,
    name: String,
}
impl Event {
    fn new(owner: &str, name: &str) -> Event {
        Event {
            owner: owner.to_owned(),
            name: name.to_owned(),
        }
    }

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
            Event::new(sas[0], sas[1])
        } else {
            panic!("wrong format!");
        }
    }
}
