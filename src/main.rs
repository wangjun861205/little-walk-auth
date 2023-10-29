mod handlers;

use std::io;

use actix_web::{
    middleware::Logger,
    web::{put, Data},
    App, HttpServer,
};
use auth_service::{
    core::service::Service,
    impls::{
        cachers::redis::RedisCacher, hashers::sha::ShaHasher,
        repositories::postgresql::PostgresqlRepository,
        secret_generators::random::RandomSecretGenerator,
    },
    ServerConfig,
};
use sqlx::postgres::PgPool;

pub struct VerificationCodeServiceAddress(pub String);

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    let config = ServerConfig::from_env();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(config.log_level));
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("failed to connect to postgresql");
    let redis_client = redis::Client::open(config.redis_url).expect("failed to connect to redis");
    let service = Data::new(Service::<
        PostgresqlRepository,
        RedisCacher<String>,
        ShaHasher,
        RandomSecretGenerator,
        String,
    >::new(
        PostgresqlRepository::new(pool),
        RedisCacher::new(redis_client),
        ShaHasher,
        RandomSecretGenerator,
    ));

    HttpServer::new(move || {
        let logger = Logger::new(&config.log_format);
        App::new()
            .wrap(logger)
            .app_data(service.clone())
            .app_data(Data::new(VerificationCodeServiceAddress(
                "localhost:8002".into(),
            )))
            .route(
                "/login_by_verification_code",
                put().to(handlers::login_by_verification_code::<
                    PostgresqlRepository,
                    RedisCacher<String>,
                    ShaHasher,
                    RandomSecretGenerator,
                >),
            )
    })
    .bind(config.server_address)?
    .run()
    .await
}
