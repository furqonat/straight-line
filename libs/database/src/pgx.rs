use crate::db::{self, Database};
use async_trait::async_trait;
use security::env::{Env, EnvImpl};
use tokio_postgres::{types::ToSql, NoTls};

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
    async fn query(&self, sql: &str, params: &Vec<&String>) -> Result<Vec<PgRow>, String> {
        let param_refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|p| p as &(dyn ToSql + Sync)).collect();
        let rows = self.client.query(sql, &param_refs).await;
        match rows {
            Ok(rows) => {
                let mut pg_rows = Vec::new();
                for row in rows.iter() {
                    let mut rw = Vec::new();
                    for va in 0..row.len() {
                        rw.push(row.get(va));
                    }
                    pg_rows.push(PgRow { row: rw });
                }
                Ok(pg_rows)
            }
            Err(e) => Err(e.to_string()),
        }
    }
    async fn query_one(&self, sql: &str, params: &Vec<&String>) -> Result<PgRow, String> {
        let param_refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|p| p as &(dyn ToSql + Sync)).collect();
        let row = self.client.query_one(sql, &param_refs).await;
        match row {
            Ok(row) => {
                let mut rows = Vec::new();
                for index in 0..row.len() {
                    rows.push(row.get(index));
                }
                Ok(PgRow { row: rows })
            }
            Err(e) => Err(e.to_string()),
        }
    }

    async fn execute(&self, sql: &str, params: &Vec<&String>) -> Result<u64, String> {
        let param_refs: Vec<&(dyn ToSql + Sync)> =
            params.iter().map(|p| p as &(dyn ToSql + Sync)).collect();

        let result = self.client.execute(sql, &param_refs).await;
        match result {
            Ok(count) => Ok(count),
            Err(e) => Err(e.to_string()),
        }
    }
}
