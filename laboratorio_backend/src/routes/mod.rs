pub mod auth;
pub mod equipamento;
pub mod agendamento;
pub mod ocorrencia;

use axum::Router;
use crate::AppState;

pub fn criar_rotas() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::rotas())
        .nest("/equipamentos", equipamento::rotas())
        .nest("/agendamentos", agendamento::rotas())
        .nest("/ocorrencias", ocorrencia::rotas())
}