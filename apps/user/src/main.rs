use actix_web::{web, App, HttpServer};
use controllers::user_controller::user_controller;
use database::pgx::Postgresql;
use logger::log::Log;
use security::env::EnvImpl;
use services::user_service::UserServiceImpl;

mod controllers;
mod middlewares;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = EnvImpl::default();
    let db = Postgresql::new(env).await;
    let logger = Log::default();
    let service = UserServiceImpl::new(db, logger);
    let web_service = web::Data::new(service);
    HttpServer::new(move || {
        App::new()
            .app_data(web_service.clone())
            .configure(user_controller)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
