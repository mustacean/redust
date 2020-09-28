pub struct Endpoint {
    owner: String,
    name: String,
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
    pub fn new(owner: &str, name: &str) -> Endpoint {
        Endpoint {
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
        format!("{}/{}", self.owner, self.name)
    }
}
