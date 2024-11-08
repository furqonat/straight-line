use bcrypt::{hash, verify, DEFAULT_COST};
use mockall::automock;

#[automock]
pub trait Hasher {
    fn hash(&self, password: &str) -> String;
    fn verify(&self, password: &str, hash: &str) -> bool;
}

#[derive(Default)]
pub struct Bcrypt;

impl Hasher for Bcrypt {
    fn hash(&self, password: &str) -> String {
        return hash(password, DEFAULT_COST).unwrap();
    }

    fn verify(&self, password: &str, hash: &str) -> bool {
        return verify(password, hash).unwrap();
    }
}
