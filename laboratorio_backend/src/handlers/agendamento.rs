use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::dto::agendamento::{AtualizarAgendamentoDto, CriarAgendamentoDto};
use crate::models::{agendamento::Agendamento, filtro::FiltroDto};
use crate::response::{ApiResponse, DinamicResponse};
use crate::AppState;

pub async fn listar(State(state): State<AppState>) -> ApiResponse<Vec<Agendamento>> {
    let agendamentos = sqlx::query_as::<_, Agendamento>(
        r#"SELECT id, uuid, equipamento_id, usuario_id, status,
            data_inicio, data_fim, notificar_email, notificar_whatsapp,
            observacao, criado_em
            FROM agendamento ORDER BY data_inicio DESC"#,
    )
    .fetch_all(&state.db)
    .await;

    match agendamentos {
        Ok(lista) => ApiResponse(StatusCode::OK, DinamicResponse::success("Agendamentos listados", lista)),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao buscar agendamentos {}", e)),
        ),
    }
}

pub async fn listar_campos_agendamento(State(state): State<AppState>) -> ApiResponse<Vec<String>> {
    let colunas = sqlx::query_scalar::<_, String>(
        "SELECT column_name FROM information_schema.columns WHERE table_name = 'agendamento'",
    )
    .fetch_all(&state.db)
    .await;

    match colunas {
        Ok(lista) => ApiResponse(StatusCode::OK, DinamicResponse::success("Colunas listadas", lista)),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao listar colunas: {}", e)),
        ),
    }
}

pub async fn busca_com_filtro(
    State(state): State<AppState>,
    Query(filtro): Query<FiltroDto>,
) -> ApiResponse<Vec<Agendamento>> {
    let campos_permitidos = [
        "id",
        "uuid",
        "equipamento_id",
        "usuario_id",
        "status",
        "data_inicio",
        "data_fim",
        "notificar_email",
        "notificar_whatsapp",
        "observacao",
        "criado_em",
    ];

    if !campos_permitidos.contains(&filtro.campo.as_str()) {
        return ApiResponse(StatusCode::BAD_REQUEST, DinamicResponse::error("Campo invalido"));
    }

    let (comparador, valor) = match filtro.operador.as_str() {
        "igual" | "=" | "==" => ("=", filtro.valor),
        _ => ("ILIKE", format!("%{}%", filtro.valor)),
    };

    let sql = format!(
        r#"SELECT id, uuid, equipamento_id, usuario_id, status,
        data_inicio, data_fim, notificar_email, notificar_whatsapp,
        observacao, criado_em
        FROM agendamento WHERE {}::text {} $1 ORDER BY data_inicio DESC"#,
        filtro.campo, comparador
    );

    let agendamentos = sqlx::query_as::<_, Agendamento>(&sql)
        .bind(valor)
        .fetch_all(&state.db)
        .await;

    match agendamentos {
        Ok(lista) => ApiResponse(StatusCode::OK, DinamicResponse::success("Agendamentos encontrados", lista)),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao buscar agendamentos: {}", e)),
        ),
    }
}

pub async fn buscar_por_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> ApiResponse<Agendamento> {
    let agendamento = sqlx::query_as::<_, Agendamento>(
        r#"SELECT id, uuid, equipamento_id, usuario_id, status,
            data_inicio, data_fim, notificar_email, notificar_whatsapp,
            observacao, criado_em
            FROM agendamento WHERE uuid = $1"#,
    )
    .bind(uuid)
    .fetch_optional(&state.db)
    .await;

    match agendamento {
        Ok(Some(a)) => ApiResponse(StatusCode::OK, DinamicResponse::success("Agendamento encontrado", a)),
        Ok(None) => ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Agendamento nao encontrado")),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao buscar agendamento {e}")),
        ),
    }
}

pub async fn criar(
    State(state): State<AppState>,
    Json(payload): Json<CriarAgendamentoDto>,
) -> ApiResponse<Agendamento> {
    let equipamento = sqlx::query_scalar::<_, i32>("SELECT id FROM equipamento WHERE uuid = $1")
        .bind(payload.equipamento_uuid)
        .fetch_optional(&state.db)
        .await;

    let equipamento_id = match equipamento {
        Ok(Some(id)) => id,
        Ok(None) => {
            return ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Equipamento nao encontrado"));
        }
        Err(e) => {
            return ApiResponse(
                StatusCode::INTERNAL_SERVER_ERROR,
                DinamicResponse::error(format!("Erro ao buscar equipamento {e}")),
            );
        }
    };

    let conflito = sqlx::query_scalar::<_, bool>(
        r#"SELECT EXISTS (
            SELECT 1 FROM agendamento
            WHERE equipamento_id = $1
            AND status NOT IN ('cancelado', 'concluido')
            AND (data_inicio, data_fim) OVERLAPS ($2, $3)
        )"#,
    )
    .bind(equipamento_id)
    .bind(payload.data_inicio)
    .bind(payload.data_fim)
    .fetch_one(&state.db)
    .await;

    match conflito {
        Ok(true) => return ApiResponse(StatusCode::CONFLICT, DinamicResponse::error("Horario indisponivel")),
        Err(e) => {
            return ApiResponse(
                StatusCode::INTERNAL_SERVER_ERROR,
                DinamicResponse::error(format!("Erro ao validar horario {e}")),
            );
        }
        _ => {}
    }

    let usuario_id = 1;

    let agendamento = sqlx::query_as::<_, Agendamento>(
        r#"INSERT INTO agendamento (equipamento_id, usuario_id, data_inicio, data_fim,
            notificar_email, notificar_whatsapp, observacao)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, uuid, equipamento_id, usuario_id, status,
            data_inicio, data_fim, notificar_email, notificar_whatsapp,
            observacao, criado_em"#,
    )
    .bind(equipamento_id)
    .bind(usuario_id)
    .bind(payload.data_inicio)
    .bind(payload.data_fim)
    .bind(payload.notificar_email.unwrap_or(false))
    .bind(payload.notificar_whatsapp.unwrap_or(false))
    .bind(payload.observacao)
    .fetch_one(&state.db)
    .await;

    match agendamento {
        Ok(a) => ApiResponse(StatusCode::CREATED, DinamicResponse::success("Agendamento criado com sucesso", a)),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao criar agendamento {e}")),
        ),
    }
}

pub async fn atualizar(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<AtualizarAgendamentoDto>,
) -> ApiResponse<Agendamento> {
    let agendamento = sqlx::query_as::<_, Agendamento>(
        r#"UPDATE agendamento SET
        status = COALESCE($2, status),
        data_inicio = COALESCE($3, data_inicio),
        data_fim = COALESCE($4, data_fim),
        notificar_email = COALESCE($5, notificar_email),
        notificar_whatsapp = COALESCE($6, notificar_whatsapp),
        observacao = COALESCE($7, observacao)
        WHERE uuid = $1
        RETURNING id, uuid, equipamento_id, usuario_id, status,
        data_inicio, data_fim, notificar_email, notificar_whatsapp,
        observacao, criado_em"#,
    )
    .bind(uuid)
    .bind(payload.status)
    .bind(payload.data_inicio)
    .bind(payload.data_fim)
    .bind(payload.notificar_email)
    .bind(payload.notificar_whatsapp)
    .bind(payload.observacao)
    .fetch_optional(&state.db)
    .await;

    match agendamento {
        Ok(Some(a)) => ApiResponse(StatusCode::OK, DinamicResponse::success("Agendamento atualizado", a)),
        Ok(None) => ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Nenhum agendamento encontrado")),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao atualizar o agendamento {e}")),
        ),
    }
}

pub async fn cancelar(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> ApiResponse<Agendamento> {
    let agendamento = sqlx::query_as::<_, Agendamento>(
        r#"UPDATE agendamento SET status ='cancelado'
            WHERE uuid = $1 AND status NOT IN ('concluido', 'cancelado')
            RETURNING id, uuid, equipamento_id, usuario_id, status,
            data_inicio, data_fim, notificar_email, notificar_whatsapp,
            observacao, criado_em"#,
    )
    .bind(uuid)
    .fetch_optional(&state.db)
    .await;

    match agendamento {
        Ok(Some(a)) => ApiResponse(StatusCode::OK, DinamicResponse::success("Agendamento cancelado", a)),
        Ok(None) => ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Agendamento nao encontrado")),
        Err(e) => {
            eprintln!("Erro ao cancelar agendamento: {:?}", e);
            ApiResponse(
                StatusCode::INTERNAL_SERVER_ERROR,
                DinamicResponse::error("Erro ao atualizar o agendamento"),
            )
        }
    }
}
