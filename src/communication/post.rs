use crate::communication::Sender;
use crate::service::Endpoint;

pub trait IPost {
    fn post(&self, sender: &Sender, args: &str) -> Result<String, ()>;

    // to be deleted later:
    fn post_with_cli(&self, sender: &redis::Client, args: &str) -> Result<String, ()>;
}

impl IPost for Endpoint {
    fn post(&self, sender: &Sender, args: &str) -> Result<String, ()> {
        use crate::rd_tools::IRedisClient;
        self.post_with_cli(&sender.get_client(), args)
    }

    fn post_with_cli(&self, cli: &redis::Client, args: &str) -> Result<String, ()> {
        let x = crate::rd_tools::rpush_str(
            cli.get_connection().unwrap(),
            "test".to_owned(), //self.to_string(),
            args.to_owned(),
        );

        let y = crate::rd_tools::blpop_str(cli.get_connection().unwrap(), "test".to_owned(), 2);
        x.join().unwrap().unwrap();

        y.join().unwrap()
    }
}
