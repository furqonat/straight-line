use async_trait::async_trait;
use tokio_postgres::{types::ToSql, Row};

#[async_trait]
pub trait Database {
    async fn query<'a>(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, tokio_postgres::Error>;

    async fn query_one<'a>(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, tokio_postgres::Error>;

    async fn execute(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, tokio_postgres::Error>;
}
