use actix_web::{web, App, HttpServer};
use controllers::auth_controller::auth_controller;
use database::pgx::Postgresql;
use logger::log::Log;
use security::{env::EnvImpl, hasher::Bcrypt, jwt::JwtImpl};
use services::auth_service::AuthServiceImpl;

mod controllers;
mod middlewares;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let jwt = JwtImpl::new(EnvImpl::default());
    let database = Postgresql::new(EnvImpl::default()).await;
    let bcrypt = Bcrypt::default();
    let logger = Log::default();
    let auth_service = AuthServiceImpl::new(database, bcrypt, jwt, logger);

    // Share the auth service instance with all handlers using web::Data
    let auth_service_data = web::Data::new(auth_service);

    //serve on 127.0.0.1:8080
    HttpServer::new(move || {
        App::new()
            .app_data(auth_service_data.clone()) // Share the auth service with handlers
            .configure(auth_controller) // Configure routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
