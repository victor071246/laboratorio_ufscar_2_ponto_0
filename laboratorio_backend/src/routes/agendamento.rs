use axum::{
    routing::{get, post, put},
    Router,
};
use crate::handlers::agendamento;
use crate::AppState;

pub fn rotas() -> Router<AppState> {
    Router::new()
        .route("/", get(agendamento::listar))
        .route("/", post(agendamento::criar))
        .route("/listar_todos_equipamentos", get(agendamento::listar))
        .route("/{uuid}", get(agendamento::buscar_por_uuid))
        .route("/buscar/", get(agendamento::buscar_com_filtro))
        .route("/{uuid}", put(agendamento::atualizar))
        .route("/{uuid}/cancelar", post(agendamento::cancelar))
}