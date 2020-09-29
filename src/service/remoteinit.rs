use crate::communication::Receiver;
use crate::communication::Sender;
use crate::service::Endpoint;
use crate::service::Service;

pub trait RemoteInit
where
    Self: JsonSerializable,
{
    fn init(mut _scli: redis::Connection, _endp: Endpoint, _arg: &str) -> Result<Self, ()> {
        // request/response logic
        let response_str = {
            // rd_tools::rpush_str(&mut conn, enp.to_string(), arg );
            //  rd_tools::blpop_json(xkkxkddkne)
            ""
        };
        ///////

        Ok(Self::json_deserialize(response_str))
    }
}

pub trait JsonSerializable
where
    Self: Sized,
{
    fn json_deserialize(json: &str) -> Self;
    fn json_serialize(&self) -> &str;
}

impl<'t> RemoteInit for Service<'t> {}

impl<'t> JsonSerializable for Service<'t> {
    fn json_deserialize(_: &str) -> Self {
        todo!()
    }
    fn json_serialize(&self) -> &str {
        todo!()
    }
}

impl<'t> RemoteInit for Sender<'t> {}

impl<'t> JsonSerializable for Sender<'t> {
    fn json_deserialize(_: &str) -> Self {
        todo!()
    }
    fn json_serialize(&self) -> &str {
        todo!()
    }
}

impl<'t> RemoteInit for Receiver<'t> {}

impl<'t> JsonSerializable for Receiver<'t> {
    fn json_deserialize(_: &str) -> Self {
        todo!()
    }
    fn json_serialize(&self) -> &str {
        todo!()
    }
}
