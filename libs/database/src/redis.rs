use mockall::automock;
use redis::{Client, Commands};
use security::env::{Env, EnvConfig, EnvImpl};

#[automock]
pub trait Redis {
    fn set(&self, key: &str, value: &str) -> Result<(), String>;
    fn get(&self, key: &str) -> Result<Option<String>, String>;
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

    fn get(&self, key: &str) -> Result<Option<String>, String> {
        let mut connection = self
            .client
            .get_connection()
            .expect("Failed to get connection from client");
        let value = connection.get(key);
        if value.is_err() {
            return Err("Failed to get value".to_string());
        }
        Ok(Some(value.unwrap()))
    }
}
