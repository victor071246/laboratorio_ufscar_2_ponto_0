use axum::{routing::post, Router};
use crate::handlers;
use crate::AppState;

pub fn rotas() -> Router<AppState> {
    Router::new()
        .route("/login", post(handlers::auth::login))
        .route("/registrar", post(handlers::auth::registrar))
}