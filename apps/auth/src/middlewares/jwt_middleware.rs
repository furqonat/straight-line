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
    type Transform = JwtMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddleware {
            service,
            roles: self.roles.clone(),
            jwt: JwtImpl::new(EnvImpl::default()),
            logger: Log::default(),
        }))
    }
}

pub struct JwtMiddleware<S> {
    pub service: S,
    pub roles: Vec<String>,
    pub jwt: JwtImpl<EnvImpl>,
    pub logger: Log,
}

impl<S, B> Service<ServiceRequest> for JwtMiddleware<S>
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
        self.logger
            .info("JwtMiddleware::call", "JwtMiddleware: executing middleware");

        if !self.roles.is_empty() {
            self.logger
                .info("JwtMiddleware::call", "JwtMiddleware: checking roles");

            // Try to get the Authorization header
            let token_header = req.headers().get("Authorization");

            if token_header.is_none() {
                self.logger.error(
                    "JwtMiddleware::call",
                    "JwtMiddleware: missing Authorization header",
                );
                return Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                    "Unauthorized: Missing Authorization header",
                ))));
            }

            let token_str = token_header.unwrap().to_str();
            // split Bearer from token
            let token_str = token_str.map(|s| s.split("Bearer ").collect::<Vec<_>>()[1]);
            if token_str.is_err() {
                self.logger.error(
                    "JwtMiddleware::call",
                    "JwtMiddleware: invalid Authorization header",
                );
                return Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                    "Unauthorized: Invalid Authorization header",
                ))));
            }

            let token_str = token_str.unwrap();
            let jwt_token = token_str.to_string();
            let token = self.jwt.extract(token_str);
            if token.is_none() {
                self.logger
                    .error("JwtMiddleware::call", "JwtMiddleware: invalid token");
                return Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                    "Unauthorized: Invalid token",
                ))));
            }

            let token = token.unwrap();

            if !roles_is_valid(&self.roles, &token.additional_claims.kind) {
                self.logger
                    .error("JwtMiddleware::call", "JwtMiddleware: invalid role");
                return Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                    "Unauthorized: Invalid role",
                ))));
            }

            self.logger
                .info("JwtMiddleware::call", "JwtMiddleware: valid token");
            let user_id = token.additional_claims.user_id;
            req.extensions_mut().insert(jwt_token);
            req.extensions_mut().insert(user_id);
        }
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
