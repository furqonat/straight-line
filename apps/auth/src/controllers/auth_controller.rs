use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use database::pgx::Postgresql;
use security::{env::EnvImpl, hasher::Bcrypt, jwt::JwtImpl};
use serde::{Deserialize, Serialize};

use crate::{
    middlewares,
    services::auth_service::{AuthService, AuthServiceImpl},
    utils,
};

#[derive(Serialize, Deserialize, Debug)]
struct ResponseOk<T> {
    pub data: Option<T>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseError {
    pub message: String,
}

pub fn auth_controller(config: &mut web::ServiceConfig) {
    let refresh_middeware = middlewares::refresh_middleware::Middleware {
        roles: vec![utils::constants::REFRESH_TOKEN.to_string()],
    };
    config.service(
        web::scope("/auth")
            .route("/signup", web::post().to(sign_up_handler))
            .route("/signin", web::post().to(sign_in_handler))
            .service(
                web::scope("/refresh-token")
                    .wrap(refresh_middeware)
                    .route("", web::get().to(refresh_token_handler)),
            ),
    );
}

async fn sign_up_handler(
    data: web::Json<crate::services::auth_service::SignUpData>,
    ctrl: web::Data<AuthServiceImpl<Postgresql, Bcrypt, JwtImpl<EnvImpl>>>,
) -> HttpResponse {
    match ctrl.sign_up(&data).await {
        Ok(Some(user_id)) => HttpResponse::Ok().json(ResponseOk {
            data: Some(user_id),
            message: "Successfully signed up".to_string(),
        }),
        Ok(None) => HttpResponse::BadRequest().json(ResponseError {
            message: "Failed to sign up, missing fields".to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ResponseError {
            message: format!("Error: {}", err),
        }),
    }
}

async fn sign_in_handler(
    data: web::Json<crate::services::auth_service::SignInData>,
    ctrl: web::Data<AuthServiceImpl<Postgresql, Bcrypt, JwtImpl<EnvImpl>>>,
) -> HttpResponse {
    match ctrl.sign_in(&data).await {
        Ok(Some(token)) => HttpResponse::Ok().json(ResponseOk {
            data: Some(crate::services::auth_service::TokenData {
                token: token.token,
                refresh_token: token.refresh_token,
            }),
            message: "Successfully signed in".to_string(),
        }),
        Ok(None) => HttpResponse::BadRequest().json(ResponseError {
            message: "Failed to sign in, invalid credentials".to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ResponseError {
            message: format!("Error: {}", err),
        }),
    }
}

async fn refresh_token_handler(
    ctrl: web::Data<AuthServiceImpl<Postgresql, Bcrypt, JwtImpl<EnvImpl>>>,
    req: HttpRequest,
) -> HttpResponse {
    if let Some(token) = req.extensions().get::<String>() {
        match ctrl.gain_new_token(token).await {
            Ok(Some(new_token)) => HttpResponse::Ok().json(ResponseOk {
                data: Some(new_token),
                message: "Successfully refreshed token".to_string(),
            }),
            Ok(None) => HttpResponse::BadRequest().json(ResponseError {
                message: "Failed to refresh token".to_string(),
            }),
            Err(err) => HttpResponse::InternalServerError().json(ResponseError {
                message: format!("Error: {}", err),
            }),
        }
    } else {
        HttpResponse::BadRequest().json(ResponseError {
            message: "Token not found in request".to_string(),
        })
    }
}
