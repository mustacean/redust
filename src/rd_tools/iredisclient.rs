pub trait IRedisClient {
    fn get_client(&self) -> &redis::Client;
    fn get_conn(&self) -> redis::Connection;
}
