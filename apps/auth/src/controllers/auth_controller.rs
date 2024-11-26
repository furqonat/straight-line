use actix_web::{
    cookie::{time, Cookie},
    web, HttpMessage, HttpRequest, HttpResponse,
};
use database::{pgx::Postgresql, redis::RedisImpl};
use logger::log::Log;
use security::{env::EnvImpl, hasher::Bcrypt, jwt::JwtImpl};
use serde::{Deserialize, Serialize};

use crate::{
    middlewares::{self},
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
    let refresh_middleware = middlewares::refresh_middleware::Middleware {
        roles: vec![utils::constants::REFRESH_TOKEN.to_string()],
    };
    let jwt_middleware = middlewares::jwt_middleware::Middleware {
        roles: vec![utils::constants::AUTH_TOKEN.to_string()],
    };
    config.service(
        web::scope("/auth")
            .route("/signup", web::post().to(sign_up_handler))
            .route("/signin", web::post().to(sign_in_handler))
            .route("/signout", web::get().to(sign_out_handler))
            .service(
                web::scope("/refresh-token")
                    .wrap(refresh_middleware)
                    .route("", web::get().to(refresh_token_handler)),
            )
            .service(
                web::scope("/token")
                    .wrap(jwt_middleware)
                    .route("", web::get().to(get_token_handler)),
            ),
    );
}

async fn sign_up_handler(
    data: web::Json<crate::services::auth_service::SignUpData>,
    ctrl: web::Data<AuthServiceImpl<Postgresql, Bcrypt, JwtImpl<EnvImpl>, Log, RedisImpl>>,
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
    ctrl: web::Data<AuthServiceImpl<Postgresql, Bcrypt, JwtImpl<EnvImpl>, Log, RedisImpl>>,
) -> HttpResponse {
    // TODO: add secure cookie and strict same site
    match ctrl.sign_in(&data).await {
        Ok(Some(token)) => {
            let cookie = Cookie::build("token", token.token.clone())
                .path("/")
                .max_age(time::Duration::hours(4))
                .http_only(true)
                .finish();
            let refresh_cookie = Cookie::build("refresh_token", token.refresh_token.clone())
                .path("/")
                .http_only(true)
                .max_age(time::Duration::weeks(4))
                .finish();
            HttpResponse::Ok()
                .cookie(cookie)
                .cookie(refresh_cookie)
                .json(ResponseOk {
                    data: Some(crate::services::auth_service::TokenData {
                        token: token.token,
                        refresh_token: token.refresh_token,
                    }),
                    message: "Successfully signed in".to_string(),
                })
        }
        Ok(None) => HttpResponse::BadRequest().json(ResponseError {
            message: "Failed to sign in, invalid credentials".to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ResponseError {
            message: format!("Error: {}", err),
        }),
    }
}

async fn sign_out_handler(
    ctrl: web::Data<AuthServiceImpl<Postgresql, Bcrypt, JwtImpl<EnvImpl>, Log, RedisImpl>>,
    req: HttpRequest,
) -> HttpResponse {
    let token = req.cookie("token");
    let refresh_token = req.cookie("refresh_token");
    if token.is_none() {
        return HttpResponse::BadRequest().json(ResponseError {
            message: "Token not found in request".to_string(),
        });
    }
    if refresh_token.is_none() {
        return HttpResponse::BadRequest().json(ResponseError {
            message: "Refresh token not found in request".to_string(),
        });
    }
    let token = token.unwrap().value().to_string();
    let refresh_token = refresh_token.unwrap().value().to_string();

    match ctrl.sign_out(&token, &refresh_token).await {
        Ok(Some(_)) => HttpResponse::Ok().json(ResponseOk {
            data: Some(()),
            message: "Successfully signed out".to_string(),
        }),
        Ok(None) => HttpResponse::BadRequest().json(ResponseError {
            message: "Failed to sign out".to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ResponseError {
            message: format!("Error: {}", err),
        }),
    };

    HttpResponse::Ok().json(ResponseOk {
        data: Some(()),
        message: "Successfully signed out".to_string(),
    })
}

async fn refresh_token_handler(
    ctrl: web::Data<AuthServiceImpl<Postgresql, Bcrypt, JwtImpl<EnvImpl>, Log, RedisImpl>>,
    req: HttpRequest,
) -> HttpResponse {
    if let Some(refresh_token) = req.extensions().get::<String>() {
        match ctrl.gain_new_token(refresh_token).await {
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

async fn get_token_handler(
    _ctrl: web::Data<AuthServiceImpl<Postgresql, Bcrypt, JwtImpl<EnvImpl>, Log, RedisImpl>>,
    req: HttpRequest,
) -> HttpResponse {
    if let Some(jwt_token) = req.extensions().get::<String>() {
        HttpResponse::Ok().json(ResponseOk {
            data: Some(jwt_token.clone()),
            message: "Successfully got token".to_string(),
        })
    } else {
        HttpResponse::BadRequest().json(ResponseError {
            message: "Token not found in request".to_string(),
        })
    }
}
