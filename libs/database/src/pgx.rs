use async_trait::async_trait;
use security::env::{Env, EnvImpl};
use tokio_postgres::{types::ToSql, NoTls};

use crate::db::{self, Database};

pub struct Postgresql {
    client: tokio_postgres::Client,
}

pub struct PgRow {
    row: Vec<String>,
}

impl PgRow {
    pub fn get(&self, index: usize) -> String {
        self.row[index].clone()
    }

    pub fn new() -> Self {
        PgRow { row: Vec::new() }
    }
}

impl db::Row for PgRow {}

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
impl Database<PgRow> for Postgresql {
    async fn query(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<PgRow>, tokio_postgres::Error> {
        let rows = self.client.query(sql, params).await;
        match rows {
            Ok(rows) => {
                let mut pg_rows = Vec::new();
                for (index, row) in rows.iter().enumerate() {
                    pg_rows.push(PgRow {
                        row: row.get(index),
                    });
                }
                Ok(pg_rows)
            }
            Err(e) => Err(e),
        }
    }
    async fn query_one(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<PgRow, tokio_postgres::Error> {
        let row = self.client.query_one(sql, params).await;
        match row {
            Ok(row) => {
                let mut rows = Vec::new();
                for index in 0..row.len() {
                    rows.push(row.get(index));
                }
                Ok(PgRow { row: rows })
            }
            Err(e) => Err(e),
        }
    }

    async fn execute(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, tokio_postgres::Error> {
        return self.client.execute(sql, params).await;
    }
}
