use std::vec;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use database::{db::Database, pgx::PgRow, redis::RedisRow};
use logger::logger::Logger;
use security::{
    hasher::Hasher,
    jwt::{AdditionalClaims, Claims, Jwt},
    uuid::uuid_v4,
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
    async fn sign_out(&self, token: &str, refresh_token: &str) -> Result<Option<String>, String>;
    async fn gain_new_token(&self, old_token: &str) -> Result<Option<String>, String>;
}

pub struct AuthServiceImpl<T: Database<PgRow>, B: Hasher, E: Jwt, L: Logger, R: Database<RedisRow>>
{
    db: T,
    hasher: B,
    jwt: E,
    logger: L,
    redis: R,
}

impl<T: Database<PgRow>, B: Hasher, E: Jwt, L: Logger, R: Database<RedisRow>>
    AuthServiceImpl<T, B, E, L, R>
{
    pub fn new(db: T, hasher: B, jwt: E, logger: L, redis: R) -> Self {
        Self {
            db,
            hasher,
            jwt,
            logger,
            redis,
        }
    }
}

#[async_trait]
impl<
        T: Database<PgRow> + Send + Sync,
        B: Hasher + Send + Sync,
        E: Jwt + Send + Sync,
        L: Logger + Send + Sync,
        R: Database<RedisRow> + Send + Sync,
    > AuthService for AuthServiceImpl<T, B, E, L, R>
{
    async fn sign_in(&self, data: &SignInData) -> Result<Option<TokenData>, String> {
        self.logger
            .info("auth_service::sign_in", "sign in is initialized");
        let row = self
            .db
            .query_one(
                "SELECT id, username, password FROM users WHERE username = $1",
                &vec![&data.username],
            )
            .await;
        match row {
            Ok(row) => {
                self.logger
                    .info("auth_service::sign_in", "user found in database");
                let user_id: String = row.get(0);
                let username: String = row.get(1);
                let password: String = row.get(2);
                self.logger
                    .info("auth_service::sign_in", "trying to verify password");
                if self.hasher.verify(&data.password, &password) {
                    self.logger
                        .info("auth_service::sign_in", "password verified");
                    self.logger
                        .info("auth_service::sign_in", "creating a token");
                    let token = self.jwt.sign(&Claims {
                        sub: username.clone(),
                        iat: Utc::now().timestamp() as usize,
                        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
                        nbf: Utc::now().timestamp() as usize,
                        jti: uuid_v4(),
                        additional_claims: AdditionalClaims {
                            user_id: user_id.clone(),
                            kind: AUTH_TOKEN.to_string(),
                        },
                    });
                    self.logger
                        .info("auth_service::sign_in", "creating a refresh token");
                    let refresh_token = self.jwt.sign(&Claims {
                        sub: username.clone(),
                        iat: Utc::now().timestamp() as usize,
                        exp: (Utc::now() + Duration::weeks(4)).timestamp() as usize,
                        nbf: Utc::now().timestamp() as usize,
                        jti: uuid_v4(),
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
                    self.logger
                        .error("auth_service::sign_in", "password is not match");
                    Ok(None)
                }
            }
            Err(e) => {
                let message = format!("an error occurred: {}", e);
                self.logger.error("auth_service::sign_in", &message);
                Ok(None)
            }
        }
    }

    async fn sign_up(&self, data: &SignUpData) -> Result<Option<String>, String> {
        self.logger.info(
            "auth_service::sign_up",
            "sign up is initialized and querying database",
        );
        let row = self
            .db
            .query_one(
                "INSERT INTO users (name, username, password) VALUES ($1, $2, $3) RETURNING username",
                &vec![&data.name,  &data.username, &self.hasher.hash(&data.password)],
            )
            .await;
        match row {
            Ok(row) => {
                self.logger
                    .info("auth_service::sign_up", "user created in database");
                Ok(Some(row.get(0)))
            }
            Err(e) => {
                let message = format!("an error occurred: {}", e);
                self.logger.error("auth_service::sign_up", &message);
                Ok(None)
            }
        }
    }

    async fn sign_out(&self, token: &str, refresh_token: &str) -> Result<Option<String>, String> {
        self.logger
            .info("auth_service::sign_out", "sign out is initialized");
        let jti = self
            .jwt
            .extract(token)
            .expect("Failed to extract jti from token");
        let refresh_jti = self
            .jwt
            .extract(refresh_token)
            .expect("Failed to extract jti from token");
        self.logger
            .info("auth_service::sign_out", "inserting jti into redis");
        let _ = self
            .redis
            .execute(&jti.jti, &vec![&"true".to_string()])
            .await;
        let _ = self
            .redis
            .execute(&refresh_jti.jti, &vec![&"true".to_string()])
            .await;

        Ok(Some("Successfully signed out".to_string()))
    }

    async fn gain_new_token(&self, old_token: &str) -> Result<Option<String>, String> {
        if self.jwt.verify(&old_token) {
            self.logger
                .info("auth_service::gain_new_token", "old token is valid");
            let old_token_claims = self.jwt.extract(&old_token);
            if old_token_claims.is_none() {
                self.logger.error(
                    "auth_service::gain_new_token",
                    "failed to extract claims from token",
                );
                return Ok(None);
            }

            let old_claims = old_token_claims.unwrap();

            // check if token is in blacklist
            let result = self.redis.query_one(&old_claims.jti, &vec![]).await;
            let msg = format!("current jti is {0}", old_claims.jti);
            self.logger.info("auth_service::gain_new_token", &msg);
            match result {
                Ok(value) => {
                    if !value.row.is_empty() {
                        self.logger
                            .info("auth_service::gain_new_token", "jti is in blacklist");
                        return Ok(None);
                    }
                }
                Err(e) => {
                    let message = format!("jti not in redis {e}");
                    self.logger.info("auth_service::gain_new_token", &message);
                }
            }

            // Set expiration to 1 hour from now
            let expired = (Utc::now() + Duration::hours(1)).timestamp() as usize;

            let claims = Claims {
                sub: old_claims.sub.clone(),
                iat: Utc::now().timestamp() as usize,
                exp: expired,
                nbf: Utc::now().timestamp() as usize,
                jti: old_claims.jti.clone(),
                additional_claims: AdditionalClaims {
                    user_id: old_claims.additional_claims.user_id.clone(),
                    kind: old_claims.additional_claims.kind.clone(),
                },
            };

            // Sign a new token with updated claims
            let new_token = self.jwt.sign(&claims);
            self.logger.info(
                "auth_service::gain_new_token",
                "new token is created successfully",
            );
            Ok(Some(new_token))
        } else {
            self.logger
                .error("auth_service::gain_new_token", "old token is not valid");
            Ok(None)
        }
    }
}
