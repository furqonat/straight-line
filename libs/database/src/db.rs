use async_trait::async_trait;
use mockall::automock;

#[automock]
pub trait Row {}

#[async_trait]
#[automock]
pub trait Database<T>
where
    T: Row,
{
    async fn query(&self, sql: &str, params: &Vec<&String>) -> Result<Vec<T>, String>;

    async fn query_one(&self, sql: &str, params: &Vec<&String>) -> Result<T, String>;

    async fn execute(&self, sql: &str, params: &Vec<&String>) -> Result<u64, String>;
}
