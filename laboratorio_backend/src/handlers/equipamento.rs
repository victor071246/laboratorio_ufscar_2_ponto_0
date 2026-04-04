use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{dto::equipamento::{AtualizarEquipamentoDto, CriarEquipamentoDto}, response::DinamicResponse};
use crate::models::equipamento::{Equipamento, EstadoEquipamento};
use crate::response::ApiResponse;
use crate::AppState;

pub async fn listar_todos_equipamentos (
    State(state): State<AppState>,
) -> ApiResponse<Vec<Equipamento>> {
    let equipamentos = sqlx::query_as!(
        Equipamento,
        r#"SELECT id, uuid, nome, descricao, estado as "estado: EstadoEquipamento",
        data_aquisicao, peso_kg, largura_cm, altura_cm, profundidade_cm,
        ultima_vez_disponivel, ultima_vez_em_manutencao, criado_em, criado_por
        FROM equipamento ORDER BY nome"#
    ).fetch_all(&state.db)
    .await;

    match equipamentos {
        Ok(lista) => ApiResponse(
            StatusCode::OK,
            DinamicResponse::success("Equipamentos listados", lista),
        ),
        Err(_) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error("Erro ao buscar equipamentos"),
        )
    }
}

pub async fn buscar_por_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> ApiResponse<Equipamento> {
    let equipamento = sqlx::query_as!(
        Equipamento,
        r#"SELECT id, uuid, nome, descricao, estado as "estado: EstadoEquipamento",
            data_aquisicao, peso_kg, largura_cm, altura_cm, profundidade_cm,
            ultima_vez_disponivel, ultima_vez_em_manutencao, criado_em, criado_por
            FROM equipamento WHERE uuid = $1"#,
            uuid
    )
    .fetch_optional(&state.db)
    .await;

    match equipamento {
        Ok(Some(e)) => ApiResponse(
            StatusCode::OK,
            DinamicResponse::success("Equipamento encontrado", e),
        ),
        Ok(None) => ApiResponse(
            StatusCode::NOT_FOUND,
            DinamicResponse::error("Equipamento não encontrado"),
        )
        ,
        Err(_) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error("Erro ao buscar equipamento")
        )
    }
}

pub async fn deletar (
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> ApiResponse<()> {
    let result = sqlx::query!("DELETE FROM equipamento WHERE uuid = $1", uuid)
    .execute(&state.db)
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => ApiResponse (
            StatusCode::OK,
            DinamicResponse::success("Equipamento deletado", ()),
        ),
        Ok(_) => ApiResponse(
            StatusCode::NOT_FOUND,
            DinamicResponse::error("Equipamento não encontrado")
        ),
        Err(_) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error("Erro ao deletar equipamento"),
        )
    }
}