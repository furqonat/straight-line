use async_trait::async_trait;
use chrono::{Duration, Utc};
use database::db::Database;
use security::{
    hasher::Hasher,
    jwt::{AdditionalClaims, Claims, Jwt},
};
use serde::{Deserialize, Serialize};

use crate::utils::constants::{AUTH_TOKEN, REFRESH_TOKEN};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenData {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpData {
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GainNewTokenData {
    pub old_token: String,
}

#[async_trait]
pub trait AuthService {
    async fn sign_in(&self, data: &SignInData) -> Result<Option<TokenData>, String>;
    async fn sign_up(&self, data: &SignUpData) -> Result<Option<String>, String>;
    async fn gain_new_token(&self, old_token: &str) -> Result<Option<String>, String>;
}

pub struct AuthServiceImpl<T: Database, B: Hasher, E: Jwt> {
    db: T,
    hasher: B,
    jwt: E,
}

impl<T: Database, B: Hasher, E: Jwt> AuthServiceImpl<T, B, E> {
    pub fn new(db: T, hasher: B, jwt: E) -> Self {
        Self { db, hasher, jwt }
    }
}

#[async_trait]
impl<T: Database + Send + Sync, B: Hasher + Send + Sync, E: Jwt + Send + Sync> AuthService
    for AuthServiceImpl<T, B, E>
{
    async fn sign_in(&self, data: &SignInData) -> Result<Option<TokenData>, String> {
        let row = self
            .db
            .query_one(
                "SELECT id, username, password FROM users WHERE username = $1",
                &[&data.username],
            )
            .await;
        match row {
            Ok(row) => {
                let user_id: String = row.get(0);
                let username: String = row.get(1);
                let password: String = row.get(2);
                if self.hasher.verify(&data.password, &password) {
                    let token = self.jwt.sign(&Claims {
                        sub: username.clone(),
                        iat: Utc::now().timestamp() as usize,
                        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
                        nbf: Utc::now().timestamp() as usize,
                        additional_claims: AdditionalClaims {
                            user_id: user_id.clone(),
                            kind: AUTH_TOKEN.to_string(),
                        },
                    });
                    let refresh_token = self.jwt.sign(&Claims {
                        sub: username.clone(),
                        iat: Utc::now().timestamp() as usize,
                        exp: (Utc::now() + Duration::weeks(4)).timestamp() as usize,
                        nbf: Utc::now().timestamp() as usize,
                        additional_claims: AdditionalClaims {
                            user_id: user_id.clone(),
                            kind: REFRESH_TOKEN.to_string(),
                        },
                    });

                    let token_data = TokenData {
                        token,
                        refresh_token,
                    };
                    Ok(Some(token_data))
                } else {
                    Ok(None)
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                Ok(None)
            }
        }
    }

    async fn sign_up(&self, data: &SignUpData) -> Result<Option<String>, String> {
        let row = self
            .db
            .query_one(
                "INSERT INTO users (name, email, username, password) VALUES ($1, $2, $3, $4) RETURNING username",
                &[&data.name, &data.email, &data.username, &self.hasher.hash(&data.password)],
            )
            .await;
        match row {
            Ok(row) => Ok(Some(row.get(0))),
            Err(e) => {
                println!("Error: {}", e);
                Ok(None)
            }
        }
    }

    async fn gain_new_token(&self, old_token: &str) -> Result<Option<String>, String> {
        if self.jwt.verify(&old_token) {
            let old_token_claims = self.jwt.extract(&old_token);
            if old_token_claims.is_none() {
                return Ok(None);
            }

            let old_claims = old_token_claims.unwrap();

            // Set expiration to 1 hour from now
            let expired = (Utc::now() + Duration::hours(1)).timestamp() as usize;

            let claims = Claims {
                sub: old_claims.sub.clone(),
                iat: Utc::now().timestamp() as usize,
                exp: expired,
                nbf: Utc::now().timestamp() as usize,
                additional_claims: AdditionalClaims {
                    user_id: old_claims.additional_claims.user_id.clone(),
                    kind: old_claims.additional_claims.kind.clone(),
                },
            };

            // Sign a new token with updated claims
            let new_token = self.jwt.sign(&claims);

            Ok(Some(new_token))
        } else {
            Ok(None)
        }
    }
}
