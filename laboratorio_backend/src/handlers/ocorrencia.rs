use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::dto::ocorrencia::{CriarOcorrenciaDto, FiltroOcorrenciaDto};
use crate::models::ocorrencia::{Ocorrencia, TipoOcorrencia};
use crate::models::equipamento::EstadoEquipamento;
use crate::response::{ApiResponse, DinamicResponse};
use crate::AppState;

pub async fn listar(
    State(state): State<AppState>,
) -> ApiResponse<Vec<Ocorrencia>> {
    let ocorrencias = sqlx::query_as!(
        Ocorrencia,
        r#"SELECT id, uuid, equipamento_id, registrado_por,
            tipo as "tipo: TipoOcorrencia",
            descricao, estado_anterior as "estado_anterior: EstadoEquipamento",
            removida_por_prazo, resolvida_em, criado_em
            FROM ocorrencia ORDER BY criado_em DESC"#
    )
    .fetch_all(&state.db)
    .await;

    match ocorrencias {
        Ok(lista) => ApiResponse(StatusCode::OK, DinamicResponse::success("Ocorrências listadas", lista)),
        Err(e) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro ao buscar ocorrências {e}")))
    }
}

pub async fn buscar_por_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> ApiResponse<Ocorrencia> {
    let ocorrencia = sqlx::query_as!(
        Ocorrencia,
        r#"SELECT id, uuid, equipamento_id, registrado_por,
            tipo as "tipo: TipoOcorrencia",
            descricao, estado_anterior as "estado_anterior: EstadoEquipamento",
            removida_por_prazo, resolvida_em, criado_em
            FROM ocorrencia WHERE uuid = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await;

    match ocorrencia {
        Ok(Some(o)) => ApiResponse(StatusCode::OK, DinamicResponse::success("Ocorrência encontrada", o)),
        Ok(None) => ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Ocorrência não encontrada")),
        Err(e) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro ao buscar ocorrência {e}")))
    }
}

pub async fn buscar(
    State(state): State<AppState>,
    Query(filtro):  Query<FiltroOcorrenciaDto>,
) -> ApiResponse<Vec<Ocorrencia>> {
    let ocorrencias = sqlx::query_as!(
        Ocorrencia,
        r#"SELECT id, uuid, equipamento_id, registrado_por,
            tipo as "tipo: TipoOcorrencia",
            descricao, estado_anterior as "estado_anterior: EstadoEquipamento",
            removida_por_prazo, resolvida_em, criado_em
            FROM ocorrencia
            WHERE ($1::int IS NULL OR equipamento_id = $1)
            AND ($2::tipo_ocorrencia IS NULL OR tipo = $2)
            AND ($3::bool IS NULL OR ($3 = true AND resolvida_em IS NOT NULL) OR ($3 = false AND resolvida_em IS NULL))ORDER BY criado_em DESC"#,
        filtro.equipamento_id,
        filtro.tipo as Option<TipoOcorrencia>,
        filtro.resolvida
    )
    .fetch_all(&state.db)
    .await;

    match ocorrencias {
        Ok(lista) => ApiResponse(StatusCode::OK, DinamicResponse::success("Ocorrências encontradas, ", lista)),
        Err(e) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro ao buscar ocorrências {e}")))
    }
}

pub async fn criar(
    State(state) : State<AppState>,
    Json(payload): Json<CriarOcorrenciaDto>,
) -> ApiResponse<Ocorrencia> {
    let equipamento = sqlx::query!(
        "SELECT id, estado as \"estado: EstadoEquipamento\" FROM equipamento WHERE uuid = $1",
        payload.equipamento_uuid
    )
    .fetch_optional(&state.db)
    .await;

    let (equipamento_id, estado_anterior) = match equipamento {
        Ok(Some(e)) => (e.id, e.estado),
        Ok(None) => return ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Equipamento não encontrado")),
        Err(e) => return ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro ao buscar equipamento {e}"))),
    };

    // TODO: registrado_por vem do JWT
    let registrado_por: Option<i32> = None;

    let ocorrencia = sqlx::query_as!(
        Ocorrencia,
        r#"INSERT INTO ocorrencia (equipamento_id, registrado_por, tipo, descricao, estado_anterior)
            VALUES ($1, $2, $3, $4, $5::estado_equipamento)
            RETURNING id, uuid, equipamento_id, registrado_por,
            tipo as "tipo: TipoOcorrencia",
            descricao, estado_anterior as "estado_anterior: EstadoEquipamento",
            removida_por_prazo, resolvida_em, criado_em"#,
        equipamento_id,
        registrado_por,
        payload.tipo as TipoOcorrencia,
        payload.descricao,
        estado_anterior as _
    ).fetch_one(&state.db)
    .await;

    match ocorrencia{
        Ok(o) => ApiResponse(StatusCode::CREATED, DinamicResponse::success("Ocorrência criada", o)),
        Err(e) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro ao criar ocorrência {e}")))
    }

}

pub async fn resolver(
    State(state) : State<AppState>,
    Path(uuid): Path<Uuid>,
) -> ApiResponse<Ocorrencia> {
    let ocorrencia = sqlx::query_as!(
        Ocorrencia,
        r#"UPDATE ocorrencia SET resolvida_em = now()
            WHERE uuid = $1 AND resolvida_em IS NULL
            RETURNING id, uuid, equipamento_id, registrado_por,
            tipo as "tipo: TipoOcorrencia",
            descricao, estado_anterior as "estado_anterior: EstadoEquipamento",
            removida_por_prazo, resolvida_em, criado_em"#,
            uuid
    )
    .fetch_optional(&state.db)
    .await;

    match ocorrencia {
        Ok(Some(o)) => ApiResponse(StatusCode::OK, DinamicResponse::success("Ocorrência resolvida", o)),
        Ok(None) => ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Ocorrência não encontrada")),
        Err(e) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Ocorrência não encontrada {e}")))

    }
}