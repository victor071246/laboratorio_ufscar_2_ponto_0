use axum::{
    routing::{get, post, put, delete},
    Router
};
use crate::handlers::equipamento;
use crate::AppState;

pub fn rotas() -> Router<AppState> {
    Router::new()
        .route("/", get(equipamento::listar_todos_equipamentos))
        .route("/", post(equipamento::criar))
}