#[derive(Debug)]
pub struct Endpoint {
    owner: String,
    name: String,
}

pub fn new_endpoint(owner: &str, name: &str) -> Endpoint {
    Endpoint {
        owner: owner.to_owned(),
        name: name.to_owned(),
    }
}

impl Clone for Endpoint {
    fn clone(&self) -> Endpoint {
        Endpoint {
            name: self.name.clone(),
            owner: self.owner.clone(),
        }
    }
}

impl Endpoint {
    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn to_string(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }

    pub fn from_str(st: &str) -> Endpoint {
        let sas: Vec<_> = st.split("/").collect();

        if sas.len() == 2 {
            new_endpoint(sas[0], sas[1])
        } else {
            panic!("wrong format!");
        }
    }
}
