use async_trait::async_trait;
use mockall::automock;
use tokio_postgres::types::ToSql;

#[automock]
pub trait Row {}

#[async_trait]
#[automock]
pub trait Database<T>
where
    T: Row,
{
    async fn query(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<T>, tokio_postgres::Error>;

    async fn query_one(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<T, tokio_postgres::Error>;

    async fn execute(
        &self,
        sql: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, tokio_postgres::Error>;
}
