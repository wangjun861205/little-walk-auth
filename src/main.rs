mod handlers;

use std::io;

use actix_web::{
    middleware::Logger,
    web::{put, Data},
    App, HttpServer,
};
use auth_service::{
    core::service::Service, hashers::sha::ShaHasher, repositories::mongo::MongodbRepository,
    token_managers::jwt::JWTTokenManager,
};
use hmac::{Hmac, Mac};
use mongodb::Client;
use nb_from_env::{FromEnv, FromEnvDerive};
use sha2::Sha384;

#[derive(Debug, FromEnvDerive)]
pub struct Config {
    server_address: String,
    db_uri: String,
    secret: String,
    #[env_default("info")]
    log_level: String,
    #[env_default("%t %s %r %D")]
    log_format: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::from_env();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(config.log_level));
    let db = Client::with_uri_str(&config.db_uri)
        .await
        .expect("failed to connect to mongodb")
        .database("little-walk-auth");

    let service = Data::new(Service::<
        MongodbRepository,
        ShaHasher,
        JWTTokenManager<Hmac<Sha384>>,
    >::new(
        MongodbRepository::new(db),
        ShaHasher,
        JWTTokenManager::new(
            Hmac::new_from_slice(config.secret.as_bytes())
                .expect("failed to create jwt signing key"),
        ),
    ));

    HttpServer::new(move || {
        let logger = Logger::new(&config.log_format);
        App::new().wrap(logger).app_data(service.clone()).route(
            "/login_by_password",
            put().to(handlers::login_by_password::<
                MongodbRepository,
                ShaHasher,
                JWTTokenManager<Hmac<Sha384>>,
            >),
        )
    })
    .bind(config.server_address)?
    .run()
    .await
}
