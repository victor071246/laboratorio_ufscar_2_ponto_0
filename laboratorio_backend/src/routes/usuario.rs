use axum::{
    routing::get,
    Router,
};
use crate::handlers::usuario;
use crate::AppState;

pub fn rotas() -> Router<AppState> {
    Router::new()
        .route("/", get(usuario::listar_todos_usuarios))
        .route("/campos", get(usuario::listar_campos_usuario))
        .route("/buscar", get(usuario::busca_com_filtro))
}
