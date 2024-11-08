use dotenv::dotenv;
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
        dotenv()
            .inspect_err(|_| println!("Failed to read .env file"))
            .ok();

        match key {
            EnvConfig::SecretKey => std::env::var("JWT_SECRET").ok(),
            EnvConfig::DatabaseUrl => std::env::var("DATABASE_URL").ok(),
        }
    }
}
