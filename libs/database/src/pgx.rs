use async_trait::async_trait;
use tokio_postgres::{types::ToSql, Row};

use crate::db::Database;

pub struct Postgresql {
    client: tokio_postgres::Client,
}

impl Postgresql {
    pub fn new(client: tokio_postgres::Client) -> Self {
        Postgresql { client }
    }
}

#[async_trait]
impl Database for Postgresql {
    async fn query<'a>(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, tokio_postgres::Error> {
        return self.client.query(sql, params).await;
    }
    async fn query_one<'a>(
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
