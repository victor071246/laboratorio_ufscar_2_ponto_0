use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::{self, Uuid};

use crate::dto::agendamento::{AtualizarAgendamentoDto, CriarAgendamentoDto};
use crate::models::agendamento::{Agendamento, StatusAgendamento};
use crate::response::{ApiResponse, DinamicResponse};
use crate::AppState;

pub async fn listar(
    State(state): State<AppState>,
) -> ApiResponse<Vec<Agendamento>> {
    let agendamentos = sqlx::query_as!(
        Agendamento,
        r#"SELECT id, uuid, equipamento_id, usuario_id,
            status as "status: StatusAgendamento",
            data_inicio, data_fim, notificar_email, notificar_whatsapp,
            observacao, criado_em
            FROM agendamento ORDER BY data_inicio DESC"#
    )
    .fetch_all(&state.db)
    .await;

    match agendamentos {
        Ok(lista) =>ApiResponse(StatusCode::OK, DinamicResponse::success("Agendamentos listados", lista)),
        Err(_) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error("Erro ao buscar agendamentos"))
    }

}

pub async fn buscar_por_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>
) -> ApiResponse<Agendamento> {
    let agendamento = sqlx::query_as!(
        Agendamento,
        r#"SELECT id, uuid, equipamento_id, usuario_id,
            status as "status: StatusAgendamento",
            data_inicio, data_fim, notificar_email, notificar_whatsapp,
            observacao, criado_em
            FROM agendamento WHERE uuid = $1"#,
            uuid
    ).fetch_optional(&state.db)
    .await;

    match agendamento {
        Ok(Some(a)) => ApiResponse(StatusCode::OK, DinamicResponse::success("Agendamento encontrado", a)),
        Ok(None) => ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Agendamento não encontrado")),
        Err(_) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error("Erro ao buscar agendamento"))
    }
}

pub async fn criar(
    State(state): State<AppState>,
    Json(payload): Json<CriarAgendamentoDto>,
) -> ApiResponse<Agendamento> {
    let equipamento = sqlx::query_scalar!(
        "SELECT id FROM equipamento WHERE uuid = $1",
        payload.equipamento_uuid
    )
    .fetch_optional(&state.db)
    .await;

    let equipamento_id = match equipamento {
        Ok(Some(id)) => id,
        Ok(None) => return ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Equipamento não encontrado")),
        Err(_) => return ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error("Erro ao buscar equipamento")),
    };

    let conflito = sqlx::query_scalar!(
        r#"SELECT EXISTS (
            SELECT 1 FROM agendamento
            WHERE equipamento_id = $1
            AND status NOT IN ('cancelado', 'concluido')
            AND (data_inicio, data_fim) OVERLAPS ($2, $3)
        ) as "existe!""#,
        equipamento_id,
        payload.data_inicio,
        payload.data_fim
    ).fetch_one(&state.db)
    .await;

    match conflito {
        Ok(true) => return ApiResponse(StatusCode::CONFLICT, DinamicResponse::error("Horário indisponível")),
        Err(_) => return ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error("Erro ao validar horário")),
        _ => {}
    }

    let usuario_id = 1;

    let agendamento = sqlx::query_as!(
        Agendamento,
        r#"INSERT INTO agendamento (equipamento_id, usuario_id, data_inicio, data_fim,
            notificar_email, notificar_whatsapp, observacao)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, uuid, equipamento_id, usuario_id,
            status as "status: StatusAgendamento",
            data_inicio, data_fim, notificar_email, notificar_whatsapp,
            observacao, criado_em"#,
        equipamento_id,
        usuario_id,
        payload.data_inicio,
        payload.data_fim,
        payload.notificar_email.unwrap_or(false),
        payload.notificar_whatsapp.unwrap_or(false),
        payload.observacao
    )
    .fetch_one(&state.db)
    .await;

    match agendamento {
        Ok(a) => ApiResponse(StatusCode::CREATED, DinamicResponse::success("Agendamento criado com sucesso", a)),
        Err(_) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error("Erro ao criar o agendamento"))
    }

}

pub async fn atualizar(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<AtualizarAgendamentoDto>,
) -> ApiResponse<Agendamento> {
    let agendamento = sqlx::query_as!(
        Agendamento,
        r#"UPDATE agendamento SET
        status = COALESCE($2, status),
        data_inicio = COALESCE($3, data_inicio),
        data_fim = COALESCE($4, data_fim),
        notificar_email = COALESCE($5, notificar_email),
        notificar_whatsapp = COALESCE($6, notificar_whatsapp),
        observacao = COALESCE($7, observacao)
        WHERE uuid = $1
        RETURNING id, uuid, equipamento_id, usuario_id,
        status as "status: StatusAgendamento",
        data_inicio, data_fim, notificar_email, notificar_whatsapp,
        observacao, criado_em"#,
    uuid,
    payload.status as Option<StatusAgendamento>,
    payload.data_inicio,
    payload.data_fim,
    payload.notificar_email,
    payload.notificar_whatsapp,
    payload.observacao
    )
    .fetch_optional(&state.db)
    .await;

    match agendamento {
        Ok(Some(a)) => ApiResponse(StatusCode::OK, DinamicResponse::success("Agendamento criado com sucesso", a)),
        Ok(None) => ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Nenhum agendamento encontrado")),
        Err(_) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error("Erro ao atualizar o agendamento"))
    }
}