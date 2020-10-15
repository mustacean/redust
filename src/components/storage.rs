use crate::components::Sender;

pub struct Storage {
    sender: Sender,
}
impl Storage {
    pub fn create(sender: Sender) -> Storage {
        Storage { sender }
    }
    pub fn sender(&self) -> &Sender {
        &self.sender
    }

    pub fn list<'t>(&'t self, ep_string: &'t str) -> List<'t> {
        if self
            .sender()
            .service()
            .endpoint_names()
            .contains(&ep_string.to_owned())
        {
            return List::new(ep_string, &self.sender);
        }
        panic!("unknown endpoint!")
    }
}

pub struct List<'t> {
    name: &'t str,
    sender: &'t Sender,
    cursor: std::cell::Cell<u32>,
}
impl<'t> List<'t> {
    pub const PREFIX: &'static str = "ls:";

    pub fn new(name: &'t str, sender: &'t Sender) -> List<'t> {
        List {
            name,
            sender,
            cursor: std::cell::Cell::new(0),
        }
    }

    pub fn size(&self) -> usize {
        use crate::rd_tools::IRedisClient;

        if let Ok(i) = redis::cmd("llen")
            .arg(format!("{}{}", List::<'t>::PREFIX, self.name))
            .query::<usize>(&mut self.sender.get_conn())
        {
            i
        } else {
            0
        }
    }
}

impl<'t> Iterator for List<'t> {
    type Item = String;
    fn next(&mut self) -> Option<<Self as std::iter::Iterator>::Item> {
        //

        use crate::rd_tools::IRedisClient;
        let res = redis::cmd("lrange")
            .arg(format!("{}{}", List::<'t>::PREFIX, self.name))
            .arg(self.cursor.get())
            .arg(self.cursor.get())
            .query::<Vec<String>>(&mut self.sender.get_conn());
        self.cursor.set(self.cursor.get() + 1);

        if let Ok(i) = res {
            if let Some(j) = i.last() {
                Some(j.to_owned())
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[test]
fn test_list_cursor() {
    use crate::service::{Endpoint, Service};
    let service = Service::new(
        "master",
        "127.0.0.1",
        vec![],
        vec![Endpoint::from_str("master/service_tokens")],
        vec![],
    );

    let s_manager = Service::open(None, service).unwrap();

    let stor = s_manager.storage().unwrap();
    let ls = stor.list("master/service_tokens");
    let lsz = ls.size();

    let vvv = ls.collect::<Vec<String>>();

    assert_eq!(vvv.len(), lsz);
}
