use mockall::automock;

pub enum EnvConfig {
    SecretKey,
    DatabaseUrl,
}

#[automock]
pub trait Env {
    fn get(&self, key: &EnvConfig) -> Option<String>;
}

#[derive(Default)]
pub struct EnvImpl;

impl Env for EnvImpl {
    fn get(&self, key: &EnvConfig) -> Option<String> {
        dotenv::dotenv().ok();

        match key {
            EnvConfig::SecretKey => std::env::var("SECRET_KEY").ok(),
            EnvConfig::DatabaseUrl => std::env::var("DATABASE_URL").ok(),
        }
    }
}
