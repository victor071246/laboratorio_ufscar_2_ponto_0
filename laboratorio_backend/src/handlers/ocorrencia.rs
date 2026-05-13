use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::dto::ocorrencia::{CriarOcorrenciaDto, FiltroOcorrenciaDto};
use crate::models::equipamento::EstadoEquipamento;
use crate::models::filtro::FiltroDto;
use crate::models::ocorrencia::Ocorrencia;
use crate::response::{ApiResponse, DinamicResponse};
use crate::AppState;

pub async fn listar(State(state): State<AppState>) -> ApiResponse<Vec<Ocorrencia>> {
    let ocorrencias = sqlx::query_as::<_, Ocorrencia>(
        r#"SELECT id, uuid, equipamento_id, registrado_por,
            tipo, descricao, estado_anterior,
            removida_por_prazo, resolvida_em, criado_em
            FROM ocorrencia ORDER BY criado_em DESC"#,
    )
    .fetch_all(&state.db)
    .await;

    match ocorrencias {
        Ok(lista) => ApiResponse(StatusCode::OK, DinamicResponse::success("Ocorrencias listadas", lista)),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao buscar ocorrencias {e}")),
        ),
    }
}

pub async fn listar_campos_ocorrencia(State(state): State<AppState>) -> ApiResponse<Vec<String>> {
    let colunas = sqlx::query_scalar::<_, String>(
        "SELECT column_name FROM information_schema.columns WHERE table_name = 'ocorrencia'",
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
) -> ApiResponse<Vec<Ocorrencia>> {
    let campos_permitidos = [
        "id",
        "uuid",
        "equipamento_id",
        "registrado_por",
        "tipo",
        "descricao",
        "estado_anterior",
        "removida_por_prazo",
        "resolvida_em",
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
        r#"SELECT id, uuid, equipamento_id, registrado_por,
        tipo, descricao, estado_anterior, removida_por_prazo, resolvida_em, criado_em
        FROM ocorrencia WHERE {}::text {} $1 ORDER BY criado_em DESC"#,
        filtro.campo, comparador
    );

    let ocorrencias = sqlx::query_as::<_, Ocorrencia>(&sql)
        .bind(valor)
        .fetch_all(&state.db)
        .await;

    match ocorrencias {
        Ok(lista) => ApiResponse(StatusCode::OK, DinamicResponse::success("Ocorrencias encontradas", lista)),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao buscar ocorrencias: {}", e)),
        ),
    }
}

pub async fn buscar_por_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> ApiResponse<Ocorrencia> {
    let ocorrencia = sqlx::query_as::<_, Ocorrencia>(
        r#"SELECT id, uuid, equipamento_id, registrado_por,
            tipo, descricao, estado_anterior,
            removida_por_prazo, resolvida_em, criado_em
            FROM ocorrencia WHERE uuid = $1"#,
    )
    .bind(uuid)
    .fetch_optional(&state.db)
    .await;

    match ocorrencia {
        Ok(Some(o)) => ApiResponse(StatusCode::OK, DinamicResponse::success("Ocorrencia encontrada", o)),
        Ok(None) => ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Ocorrencia nao encontrada")),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao buscar ocorrencia {e}")),
        ),
    }
}

pub async fn buscar(
    State(state): State<AppState>,
    Query(filtro): Query<FiltroOcorrenciaDto>,
) -> ApiResponse<Vec<Ocorrencia>> {
    let ocorrencias = sqlx::query_as::<_, Ocorrencia>(
        r#"SELECT id, uuid, equipamento_id, registrado_por,
            tipo, descricao, estado_anterior,
            removida_por_prazo, resolvida_em, criado_em
            FROM ocorrencia
            WHERE ($1::int IS NULL OR equipamento_id = $1)
            AND ($2::tipo_ocorrencia IS NULL OR tipo = $2)
            AND ($3::bool IS NULL OR ($3 = true AND resolvida_em IS NOT NULL) OR ($3 = false AND resolvida_em IS NULL))
            ORDER BY criado_em DESC"#,
    )
    .bind(filtro.equipamento_id)
    .bind(filtro.tipo)
    .bind(filtro.resolvida)
    .fetch_all(&state.db)
    .await;

    match ocorrencias {
        Ok(lista) => ApiResponse(StatusCode::OK, DinamicResponse::success("Ocorrencias encontradas", lista)),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao buscar ocorrencias {e}")),
        ),
    }
}

pub async fn criar(
    State(state): State<AppState>,
    Json(payload): Json<CriarOcorrenciaDto>,
) -> ApiResponse<Ocorrencia> {
    let equipamento = sqlx::query_as::<_, (i32, EstadoEquipamento)>(
        "SELECT id, estado FROM equipamento WHERE uuid = $1",
    )
    .bind(payload.equipamento_uuid)
    .fetch_optional(&state.db)
    .await;

    let (equipamento_id, estado_anterior) = match equipamento {
        Ok(Some(e)) => e,
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

    let registrado_por: Option<i32> = None;

    let ocorrencia = sqlx::query_as::<_, Ocorrencia>(
        r#"INSERT INTO ocorrencia (equipamento_id, registrado_por, tipo, descricao, estado_anterior)
            VALUES ($1, $2, $3, $4, $5::estado_equipamento)
            RETURNING id, uuid, equipamento_id, registrado_por,
            tipo, descricao, estado_anterior,
            removida_por_prazo, resolvida_em, criado_em"#,
    )
    .bind(equipamento_id)
    .bind(registrado_por)
    .bind(payload.tipo)
    .bind(payload.descricao)
    .bind(estado_anterior)
    .fetch_one(&state.db)
    .await;

    match ocorrencia {
        Ok(o) => ApiResponse(StatusCode::CREATED, DinamicResponse::success("Ocorrencia criada", o)),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao criar ocorrencia {e}")),
        ),
    }
}

pub async fn resolver(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> ApiResponse<Ocorrencia> {
    let ocorrencia = sqlx::query_as::<_, Ocorrencia>(
        r#"UPDATE ocorrencia SET resolvida_em = now()
            WHERE uuid = $1 AND resolvida_em IS NULL
            RETURNING id, uuid, equipamento_id, registrado_por,
            tipo, descricao, estado_anterior,
            removida_por_prazo, resolvida_em, criado_em"#,
    )
    .bind(uuid)
    .fetch_optional(&state.db)
    .await;

    match ocorrencia {
        Ok(Some(o)) => ApiResponse(StatusCode::OK, DinamicResponse::success("Ocorrencia resolvida", o)),
        Ok(None) => ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Ocorrencia nao encontrada")),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Ocorrencia nao encontrada {e}")),
        ),
    }
}
