use axum::{routing::post, routing::get, Router};
use crate::handlers;
use crate::AppState;

pub fn rotas() -> Router<AppState> {
    Router::new()
        .route("/login", post(handlers::auth::login))
        .route("/registrar", post(handlers::auth::registrar))
        .route("/usuario", get(handlers::auth::checarUsuarioLogado))
        .route("/logout", post(handlers::auth::logout))
}