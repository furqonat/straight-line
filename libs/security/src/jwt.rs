use jsonwebtoken::Header;
use mockall::automock;
use serde::{Deserialize, Serialize};

use crate::env::Env;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalClaims {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String, // Optional. Audience
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize, // Optional. Issued at (as UTC timestamp)
    // pub iss: String, // Optional. Issuer
    pub nbf: usize,  // Optional. Not Before (as UTC timestamp)
    pub sub: String, // Optional. Subject (whom token refers to)
    pub additional_claims: AdditionalClaims,
}

#[automock]
pub trait Jwt {
    fn sign(&self, payload: &Claims) -> String;
    fn verify(&self, token: &str) -> bool;
    fn extract(&self, token: &str) -> Option<Claims>;
}

pub struct JwtImpl<T: Env> {
    env: T,
}

impl<T: Env> JwtImpl<T> {
    pub fn new(env: T) -> Self {
        Self { env }
    }
}

impl<T: Env> Jwt for JwtImpl<T> {
    fn sign(&self, payload: &Claims) -> String {
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let key = &self.env.get(&crate::env::EnvConfig::SecretKey).unwrap();
        let token = jsonwebtoken::encode(
            &header,
            &payload,
            &jsonwebtoken::EncodingKey::from_secret(key.as_bytes()),
        )
        .unwrap();

        return token.to_string();
    }

    fn verify(&self, token: &str) -> bool {
        let key = &self.env.get(&crate::env::EnvConfig::SecretKey).unwrap();

        let token = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(key.as_bytes()),
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
        );
        return token.is_ok();
    }

    fn extract(&self, token: &str) -> Option<Claims> {
        let key = &self.env.get(&crate::env::EnvConfig::SecretKey).unwrap();
        let token = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(key.as_bytes()),
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
        );
        return token.ok().map(|token| token.claims);
    }
}
