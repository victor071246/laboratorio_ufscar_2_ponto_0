use axum::{
    routing::{get, post, put, delete},
    Router
};
use crate::handlers::equipamento;
use crate::AppState;

pub fn rotas() -> Router<AppState> {
    Router::new()
        .route("/", get(equipamento::busca_com_filtro))
        .route("/campos", get(equipamento::listar_colunas_tabela))
        .route("/", post(equipamento::criar))
}