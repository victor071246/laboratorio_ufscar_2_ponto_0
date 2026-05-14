use axum::{
    routing::{get, post},
    Router,
};
use crate::handlers::ocorrencia;
use crate::AppState;

pub fn rotas() -> Router<AppState> {
    Router::new()
        .route("/", get(ocorrencia::listar))
        .route("/", post(ocorrencia::criar))
        .route("/buscar", get(ocorrencia::busca_com_filtro))
        .route("/buscar/filtro", get(ocorrencia::buscar))
        .route("/campos", get(ocorrencia::listar_campos_ocorrencia))
        .route("/{uuid}", get(ocorrencia::buscar_por_uuid))
        .route("/{uuid}/resolver", post(ocorrencia::resolver))
}