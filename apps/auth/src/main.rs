use actix_web::{web, App, HttpServer};
use controllers::auth_controller::auth_controller;
use database::pgx::Postgresql;
use security::{env::EnvImpl, hasher::Bcrypt, jwt::JwtImpl};
use services::auth_service::AuthServiceImpl;
use tokio_postgres::NoTls;

pub mod controllers;
pub mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres port=5432 dbname=postgres",
        NoTls,
    )
    .await
    .expect("Unable to connect to database");

    // Spawn the database connection handler in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    // Initialize other components
    let jwt = JwtImpl::new(EnvImpl::default());
    let database = Postgresql::new(client);
    let bcrypt = Bcrypt::default();
    let auth_service = AuthServiceImpl::new(database, bcrypt, jwt);

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
