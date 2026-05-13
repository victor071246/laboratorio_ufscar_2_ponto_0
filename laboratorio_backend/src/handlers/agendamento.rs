use axum::{
    Json, extract::{Path, Query, State}, http::StatusCode
};
use uuid::{self, Uuid};

use crate::dto::{agendamento::{AtualizarAgendamentoDto, CriarAgendamentoDto},filtro::FiltroDto};
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
        Err(e) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro ao buscar agendamentos {}", e)))
    }

}

pub async fn listar_campos_agendamento(
    State(state): State<AppState>
) -> ApiResponse<Vec<String>> {
    let colunas = sqlx::query!(
        "SELECT column_name FROM information_schema.columns WHERE table_name = 'agendamento'"
    )
    .fetch_all(&state.db)
    .await
    .map(|rows| rows.into_iter().map(|r| r.column_name.unwrap_or_default()).collect::<Vec<String>>());

    match colunas {
        Ok(lista) => ApiResponse(StatusCode::OK, DinamicResponse::success("Colunas listadas", lista)),
        Err(e) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro ao listar colunas: {}", e)))
    }
}

pub async fn busca_com_filtro(
    State(state): State<AppState>,
    Query(filtro): Query<FiltroDto>
) -> ApiResponse<Vec<Agendamento>> {
    if filtro.campo.is_empty() || filtro.valor.is_empty() {
        let agendamentos = sqlx::query_as!(
            Agendamento,
            r#"SELECT id, uuid, equipamento_id, usuario_id,
            status as "status: StatusAgendamento",
            data_inicio, data_fim, notificar_email, notificar_whatsapp,
            observacao, criado_em
            FROM agendamento ORDER BY data_inicio DESC"#
        ).fetch_all(&state.db).await;

        match agendamentos {
            Ok(lista) => ApiResponse(StatusCode::OK,
            DinamicResponse::success("Agendamentos listados", lista)),
            Err(e) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro: {e}")))
        };
    }

    let campos_numericos = ["id", "equipamento_id", "usuario_id"];

    let (operador_sql, valor_sq): (&str, String) = match filtro.operador.as_str() {
        ">" => (">", filtro.valor.clone()),
        ">=" => (">=", filtro.valor.clone()),
        "<=" => ("<=", filtro.valor.clone()),
        "<" => ("<", filtro.valor.clone()),
        _ => ("ILIKE", format!("%{}%", filtro.valor))
    };

    let tipo_cast = if campos_numericos.contains(&filtro.campo.as_str()) {
        "numeric"
    } else {
        "text"
    };

    let sql = format!(
    r#"SELECT id, uuid, equipamento_id, usuario_id,
    status as "status: StatusAgendamento",
    data_inicio, data_fim, notificar_email, notificar_whatsapp,
    observacao, criado_em
    FROM agendamento WHERE {}::text {} $1::text::{} ORDER BY data_inicio DESC"#,
    filtro.campo, operador_sql, tipo_cast
    );

    let agendamentos = sqlx::query_as::<_, Agendamento>(&sql)
        .bind(valor_sq)
        .fetch_all(&state.db)
        .await;

    match agendamentos {
        Ok(lista) => ApiResponse(StatusCode::OK, DinamicResponse::success("Agendamentos encontrados", lista)),
        Err(e) => {
            eprintln!("Erro busca_com_filtro agendamento: {e}");
            ApiResponse(StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao buscar agendamentos: {e}")))
        }
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
        Err(e) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro ao buscar agendamento {e}")))
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
        Err(e) => return ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro ao buscar equipamento {e}"))),
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
        Err(e) => return ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro ao validar horário {e}"))),
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
        Err(e) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro ao criar agendamento {e}")))
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
        Err(e) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro ao atualizar o agendamento {e}")))
    }
}


pub async fn cancelar(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>
) -> ApiResponse<Agendamento> {
    let agendamento = sqlx::query_as!(
        Agendamento,
        r#"UPDATE agendamento SET status ='cancelado'
            WHERE uuid = $1 AND status NOT IN ('concluido', 'cancelado')
            RETURNING id, uuid, equipamento_id, usuario_id,
            status as "status: StatusAgendamento",
            data_inicio, data_fim, notificar_email, notificar_whatsapp,
            observacao, criado_em"#,
        uuid
    ).fetch_optional(&state.db)
    .await;

    match agendamento {
        Ok(Some(a)) => ApiResponse(StatusCode::OK, DinamicResponse::success("Agendamento cancelado", a)),
        Ok(None) => ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Agendamento não encontrado")),
        Err(e) => {
            eprintln!("Erro ao criar agendamento: {:?}", e);
            ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error("Erro ao atualizar o agendamento"))
        }
    }
}
