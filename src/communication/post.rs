use crate::communication::Sender;
use crate::service::Endpoint;

pub trait IPost {
    fn post_str(&self, sender: &Sender, args: &str) -> Result<Option<String>, ()>;
}

impl IPost for Endpoint {
    fn post_str(&self, sender: &Sender, args: &str) -> Result<Option<String>, ()> {
        use crate::rd_tools::IRedisClient;

        if let Ok(_) =
            crate::rd_tools::rpush_str(sender.get_conn(), self.to_string(), args.to_owned())
        {
            if let Ok(result) = crate::rd_tools::blpop_str(sender.get_conn(), args.to_owned(), 1) {
                Ok(Some(result.0))
            } else {
                Ok(None)
            }
        } else {
            Err(())
        }
    }
}
