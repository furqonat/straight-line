use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use logger::{log::Log, logger::Logger};
use security::{
    env::EnvImpl,
    jwt::{Jwt, JwtImpl},
};

use crate::utils;

pub struct Middleware {
    pub roles: Vec<String>,
}

impl<S, B> Transform<S, ServiceRequest> for Middleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RefreshTokenMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RefreshTokenMiddleware {
            service,
            roles: self.roles.clone(),
            jwt: JwtImpl::new(EnvImpl::default()),
            logger: Log::default(),
        }))
    }
}

pub struct RefreshTokenMiddleware<S> {
    service: S,
    roles: Vec<String>,
    jwt: JwtImpl<EnvImpl>,
    logger: Log,
}

impl<S, B> Service<ServiceRequest> for RefreshTokenMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        self.logger.info(
            "RefreshTokenMiddleware::call",
            "RefreshTokenMiddleware: executing middleware",
        );

        // Check if roles exist in middleware
        if !self.roles.is_empty() {
            self.logger.info(
                "RefreshTokenMiddleware::call",
                "RefreshTokenMiddleware: checking roles",
            );

            // Try to get the Authorization-refresh header
            let token_header = extract_token(&req);

            if token_header.is_none() {
                self.logger.error(
                    "RefreshTokenMiddleware::call",
                    "RefreshTokenMiddleware: missing Authorization-refresh header",
                );
                return Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                    "Unauthorized: Missing Authorization-refresh header",
                ))));
            }

            // split Bearer from token

            let refresh_token = token_header.unwrap();

            // Validate JWT token
            let claims = self.jwt.extract(&refresh_token);
            if claims.is_none() {
                self.logger.error(
                    "RefreshTokenMiddleware::call",
                    "RefreshTokenMiddleware: failed to extract token claims",
                );
                return Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                    "Unauthorized: Invalid or expired token",
                ))));
            }

            let claims = claims.unwrap();

            // Validate role
            if !roles_is_valid(&self.roles, &claims.additional_claims.kind) {
                self.logger.error(
                    "RefreshTokenMiddleware::call",
                    "RefreshTokenMiddleware: unauthorized role",
                );
                return Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                    "Unauthorized: Invalid role",
                ))));
            }

            // Check if the token is of the type REFRESH_TOKEN
            if claims.additional_claims.kind != utils::constants::REFRESH_TOKEN {
                self.logger.error(
                    "RefreshTokenMiddleware::call",
                    "RefreshTokenMiddleware: invalid token type",
                );
                return Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                    "Unauthorized: Token is not a refresh token",
                ))));
            }

            self.logger.info(
                "RefreshTokenMiddleware::call",
                "RefreshTokenMiddleware: token and role validated",
            );

            // Attach token to request extensions for future use
            req.extensions_mut().insert(refresh_token.to_string());
        }

        // Continue with the request
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            Ok(res)
        })
    }
}

fn roles_is_valid(roles: &Vec<String>, role: &str) -> bool {
    for r in roles {
        if r == role {
            return true;
        }
    }
    return false;
}

fn extract_token(headers: &ServiceRequest) -> Option<String> {
    let token_header = headers.headers().get("Authorization-refresh");
    let token = token_header
        .map(|s| s.to_str().unwrap().split("Bearer ").collect::<Vec<_>>()[1].to_string());
    let cookie = headers.cookie("refresh_token");
    let value = cookie.map(|s| s.value().to_string());
    if token_header.is_none() || value.is_none() {
        return None;
    }
    if token_header.is_some() {
        return Some(token.unwrap());
    }
    if value.is_some() {
        return Some(value.unwrap());
    }
    None
}
