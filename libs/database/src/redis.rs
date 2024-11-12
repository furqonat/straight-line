use std::vec;

use async_trait::async_trait;
use redis::{Client, Commands};
use security::env::{Env, EnvConfig, EnvImpl};

use crate::db::{Database, Row};

pub struct RedisRow {
    pub row: Vec<String>,
}

impl RedisRow {
    pub fn new(row: Vec<String>) -> Self {
        Self { row }
    }
}

pub struct RedisImpl {
    client: Client,
}

impl Row for RedisRow {}

#[async_trait]
impl Database<RedisRow> for RedisImpl {
    async fn query(&self, _sql: &str, _params: &Vec<&String>) -> Result<Vec<RedisRow>, String> {
        Ok(vec![])
    }

    async fn query_one(&self, sql: &str, _params: &Vec<&String>) -> Result<RedisRow, String> {
        let mut connection = self
            .client
            .get_connection()
            .expect("Failed to get connection from client");
        let value: Result<String, redis::RedisError> = connection.get(sql);
        match value {
            Ok(v) => Ok(RedisRow::new(vec![v])),
            Err(_) => Err("no data in redis".to_string()),
        }
        // Ok(RedisRow::new(vec![value]))
    }

    async fn execute(&self, sql: &str, params: &Vec<&String>) -> Result<u64, String> {
        let mut connection = self
            .client
            .get_connection()
            .expect("Failed to get connection from client");
        let _: () = connection
            .set(sql, &params[0])
            .expect("Failed to set value");
        Ok(1)
    }
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
