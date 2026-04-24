mod config;
mod db;
mod response;
mod models;
mod dto;
mod services;
mod routes;
mod handlers;
mod middlewares;

use std::{net::SocketAddr};
use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: Config,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "lab_ufscar=debug, tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env().expect("Erro ao carregar configurações");

    tracing::info!("Conectando ao banco de dados");
    let pool = db::criar_pool(&config.database_url)
        .await
        .expect("Erro ao conectar ao banco");
    tracing::info!("Banco conectado");

    let state = AppState {
        db: pool,
        config: config.clone(),
    };

    let app = routes::criar_rotas()
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .expect("Endereço inválido");

    tracing::info!("Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Erro ao fazer bind");

    let config = Config::from_env().unwrap_or_else(|e| {
    eprintln!("ERRO ao carregar config: {}", e);
    std::process::exit(1);
    });

    axum::serve(listener, app)
        .await
        .expect("Erro ao iniciar o servidor");
}