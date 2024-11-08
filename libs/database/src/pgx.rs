use async_trait::async_trait;
use security::env::{Env, EnvImpl};
use tokio_postgres::{types::ToSql, NoTls, Row};

use crate::db::Database;

pub struct Postgresql {
    client: tokio_postgres::Client,
}

impl Postgresql {
    pub async fn new(env: EnvImpl) -> Self {
        let db_url = env.get(&security::env::EnvConfig::DatabaseUrl);
        if db_url.is_none() {
            panic!("DATABASE_URL is not set");
        }
        let (client, connection) = tokio_postgres::connect(&db_url.unwrap(), NoTls)
            .await
            .expect("Unable to connect to database");

        // Spawn the database connection handler in the background
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Database connection error: {}", e);
            }
        });
        Postgresql { client }
    }
}

#[async_trait]
impl Database for Postgresql {
    async fn query(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, tokio_postgres::Error> {
        return self.client.query(sql, params).await;
    }
    async fn query_one(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, tokio_postgres::Error> {
        return self.client.query_one(sql, params).await;
    }

    async fn execute(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, tokio_postgres::Error> {
        return self.client.execute(sql, params).await;
    }
}
