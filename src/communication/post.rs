use crate::communication::Sender;
use crate::service::Endpoint;

pub trait IPost {
    fn post<T>(&mut self, sender: &Sender, args: &str) -> Result<T, ()>;

    // to be deleted later:
    fn post_with_cli(&mut self, sender: &redis::Client, args: &str) -> Result<String, ()>;
}

impl IPost for Endpoint {
    fn post<T>(&mut self, _sender: &Sender, _args: &str) -> Result<T, ()> {
        todo!();
        // use crate::rd_tools::IRedisClient;
        // use crate::service::IServiceOwned;
        //let con = sender.get_service().get_conn();
    }

    fn post_with_cli(&mut self, cli: &redis::Client, args: &str) -> Result<String, ()> {
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
