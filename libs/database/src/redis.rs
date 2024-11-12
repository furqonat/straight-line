use redis::{Client, Commands};
use security::env::{Env, EnvConfig, EnvImpl};

pub trait Redis {
    fn set(&self, key: &str, value: &str) -> Result<(), String>;
    fn get(&self, key: &str) -> Result<String, String>;
}

pub struct RedisImpl {
    client: Client,
}

impl RedisImpl {
    pub fn new(env: EnvImpl) -> Self {
        let url = env
            .get(&EnvConfig::RedisUrl)
            .expect("Failed to get redis url from env");

        let client = Client::open(url).expect("Failed to connect to redis");
        Self { client }
    }
}

impl Redis for RedisImpl {
    fn set(&self, key: &str, value: &str) -> Result<(), String> {
        let mut connection = self
            .client
            .get_connection()
            .expect("Failed to get connection from client");
        let _: () = connection.set(key, value).expect("Failed to set value");
        Ok(())
    }

    fn get(&self, key: &str) -> Result<String, String> {
        let mut connection = self
            .client
            .get_connection()
            .expect("Failed to get connection from client");
        let value: String = connection.get(key).expect("Failed to get value");
        Ok(value)
    }
}
