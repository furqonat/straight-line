use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use database::pgx::Postgresql;
use logger::log::Log;

use crate::{
    middlewares::jwt_middleware::Middleware,
    services::user_service::{QueryUser, UpdateUser, UserService, UserServiceImpl},
};

pub fn user_controller(config: &mut web::ServiceConfig) {
    let jwt_middleware = Middleware {
        roles: vec!["auth_token".to_string()],
    };
    config.service(
        web::scope("/user")
            .wrap(jwt_middleware)
            .route("/profile", web::get().to(get_user_handler))
            .route("/{user_id}", web::get().to(get_user_by_id_handler))
            .route("/", web::get().to(get_users_handler))
            .route("/", web::put().to(update_user_with_id_handler)),
    );
}

async fn get_user_handler(
    service: web::Data<UserServiceImpl<Postgresql, Log>>,
    req: HttpRequest,
) -> HttpResponse {
    if let Some(user_id) = req.extensions().get::<String>() {
        match service.get_user_by_id(user_id).await {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => HttpResponse::InternalServerError().json(err),
        }
    } else {
        HttpResponse::NotFound().json("User id not found in request")
    }
}

async fn get_user_by_id_handler(
    service: web::Data<UserServiceImpl<Postgresql, Log>>,
    path: web::Path<String>,
) -> HttpResponse {
    let user_id = path.into_inner();
    match service.get_user_by_id(&user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

async fn get_users_handler(
    service: web::Data<UserServiceImpl<Postgresql, Log>>,
    query: web::Query<QueryUser>,
) -> HttpResponse {
    match service.get_users(&query).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

async fn update_user_with_id_handler(
    service: web::Data<UserServiceImpl<Postgresql, Log>>,
    path: web::Path<String>,
    body: web::Json<UpdateUser>,
) -> HttpResponse {
    let user_id = path.into_inner();
    match service.update_user(&user_id, &body).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}
