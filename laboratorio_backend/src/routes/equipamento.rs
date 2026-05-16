use axum::{
    routing::{delete, get, post, put},
    Router,
};
use crate::handlers::equipamento;
use crate::AppState;

pub fn rotas() -> Router<AppState> {
    Router::new()
        .route("/", get(equipamento::busca_com_filtro))
        .route("/", post(equipamento::criar))
        .route("/campos", get(equipamento::listar_colunas_tabela))
        .route("/buscar", get(equipamento::buscar))
        .route("/:{uuid}", get(equipamento::buscar_por_uuid))
        .route("/:{uuid}", put(equipamento::atualizar))
        .route("/:{uuid}", delete(equipamento::deletar))
}