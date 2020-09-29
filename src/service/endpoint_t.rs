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

#[test]
fn test_post() {
    use crate::communication::IPost;
    let cli = redis::Client::open("redis://127.0.0.1/").unwrap();

    let fn_result = new_endpoint("mca", "get_rich")
        .post_with_cli(&cli, "whats' upp broooooo!")
        .unwrap();

    println!("post result : {}", fn_result);

    assert_ne!(fn_result, String::default())
    //panic!("{}",fn_result);
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
