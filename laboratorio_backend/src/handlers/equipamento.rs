use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::dto::equipamento::{AtualizarEquipamentoDto, CriarEquipamentoDto, FiltroEquipamentoDto};
use crate::models::equipamento::{Equipamento, EstadoEquipamento};
use crate::models::filtro::FiltroDto;
use crate::response::{ApiResponse, DinamicResponse};
use crate::AppState;

pub async fn criar(
    State(state): State<AppState>,
    Json(payload): Json<CriarEquipamentoDto>,
) -> ApiResponse<Equipamento> {
    let estado = payload.estado.unwrap_or(EstadoEquipamento::Disponivel);

    let equipamento = sqlx::query_as::<_, Equipamento>(
        r#"INSERT INTO equipamento (nome, descricao, estado, data_aquisicao,
        peso_kg, largura_cm, altura_cm, comprimento_cm)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, uuid, nome, descricao, estado,
        data_aquisicao, peso_kg, largura_cm, altura_cm, comprimento_cm,
        ultima_vez_disponivel, ultima_vez_em_manutencao, criado_em, criado_por"#,
    )
    .bind(payload.nome)
    .bind(payload.descricao)
    .bind(estado)
    .bind(payload.data_aquisicao)
    .bind(payload.peso_kg)
    .bind(payload.largura_cm)
    .bind(payload.altura_cm)
    .bind(payload.comprimento_cm)
    .fetch_one(&state.db)
    .await;

    match equipamento {
        Ok(e) => ApiResponse(StatusCode::CREATED, DinamicResponse::success("Equipamento criado", e)),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao criar equipamento: {}", e)),
        ),
    }
}

pub async fn atualizar(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<AtualizarEquipamentoDto>,
) -> ApiResponse<Equipamento> {
    let equipamento = sqlx::query_as::<_, Equipamento>(
        r#"UPDATE equipamento SET
            nome = COALESCE($1, nome),
            descricao = COALESCE($2, descricao),
            estado = COALESCE($3, estado),
            data_aquisicao = COALESCE($4, data_aquisicao),
            peso_kg = COALESCE($5, peso_kg),
            largura_cm = COALESCE($6, largura_cm),
            altura_cm = COALESCE($7, altura_cm),
            comprimento_cm = COALESCE($8, comprimento_cm)
            WHERE uuid = $9
            RETURNING id, uuid, nome, descricao, estado,
            data_aquisicao, peso_kg, largura_cm, altura_cm, comprimento_cm,
            ultima_vez_disponivel, ultima_vez_em_manutencao, criado_em, criado_por"#,
    )
    .bind(payload.nome)
    .bind(payload.descricao)
    .bind(payload.estado)
    .bind(payload.data_aquisicao)
    .bind(payload.peso_kg)
    .bind(payload.largura_cm)
    .bind(payload.altura_cm)
    .bind(payload.comprimento_cm)
    .bind(uuid)
    .fetch_optional(&state.db)
    .await;

    match equipamento {
        Ok(Some(e)) => ApiResponse(StatusCode::OK, DinamicResponse::success("Equipamento atualizado", e)),
        Ok(None) => ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Equipamento nao encontrado")),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao atualizar o equipamento: {}", e)),
        ),
    }
}

pub async fn listar_todos_equipamentos(
    State(state): State<AppState>,
) -> ApiResponse<Vec<Equipamento>> {
    let equipamentos = sqlx::query_as::<_, Equipamento>(
        r#"SELECT id, uuid, nome, descricao, estado,
        data_aquisicao, peso_kg, largura_cm, altura_cm, comprimento_cm,
        ultima_vez_disponivel, ultima_vez_em_manutencao, criado_em, criado_por
        FROM equipamento ORDER BY nome"#,
    )
    .fetch_all(&state.db)
    .await;

    match equipamentos {
        Ok(lista) => ApiResponse(StatusCode::OK, DinamicResponse::success("Equipamentos listados", lista)),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao buscar equipamentos: {}", e)),
        ),
    }
}

pub async fn listar_colunas_tabela(State(state): State<AppState>) -> ApiResponse<Vec<String>> {
    let colunas = sqlx::query_scalar::<_, String>(
        "SELECT column_name FROM information_schema.columns WHERE table_name = 'equipamento'",
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
) -> ApiResponse<Vec<Equipamento>> {
    let campos_permitidos = [
        "id",
        "uuid",
        "nome",
        "descricao",
        "estado",
        "data_aquisicao",
        "peso_kg",
        "largura_cm",
        "altura_cm",
        "comprimento_cm",
        "ultima_vez_disponivel",
        "ultima_vez_em_manutencao",
        "criado_em",
        "criado_por",
    ];

    if !campos_permitidos.contains(&filtro.campo.as_str()) {
        return ApiResponse(StatusCode::BAD_REQUEST, DinamicResponse::error("Campo invalido"));
    }

    let (comparador, valor) = match filtro.operador.as_str() {
        "igual" | "=" | "==" => ("=", filtro.valor),
        _ => ("ILIKE", format!("%{}%", filtro.valor)),
    };

    let sql = format!(
        r#"SELECT id, uuid, nome, descricao, estado,
        data_aquisicao, peso_kg, largura_cm, altura_cm, comprimento_cm,
        ultima_vez_disponivel, ultima_vez_em_manutencao, criado_em, criado_por
        FROM equipamento WHERE {}::text {} $1 ORDER BY nome"#,
        filtro.campo, comparador
    );

    let equipamentos = sqlx::query_as::<_, Equipamento>(&sql)
        .bind(valor)
        .fetch_all(&state.db)
        .await;

    match equipamentos {
        Ok(lista) => ApiResponse(StatusCode::OK, DinamicResponse::success("Equipamentos encontrados", lista)),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao buscar equipamentos: {}", e)),
        ),
    }
}

pub async fn buscar_por_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> ApiResponse<Equipamento> {
    let equipamento = sqlx::query_as::<_, Equipamento>(
        r#"SELECT id, uuid, nome, descricao, estado,
            data_aquisicao, peso_kg, largura_cm, altura_cm, comprimento_cm,
            ultima_vez_disponivel, ultima_vez_em_manutencao, criado_em, criado_por
            FROM equipamento WHERE uuid = $1"#,
    )
    .bind(uuid)
    .fetch_optional(&state.db)
    .await;

    match equipamento {
        Ok(Some(e)) => ApiResponse(StatusCode::OK, DinamicResponse::success("Equipamento encontrado", e)),
        Ok(None) => ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Equipamento nao encontrado")),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao buscar equipamento {e}")),
        ),
    }
}

pub async fn deletar(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> ApiResponse<()> {
    let result = sqlx::query("DELETE FROM equipamento WHERE uuid = $1")
        .bind(uuid)
        .execute(&state.db)
        .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => {
            ApiResponse(StatusCode::OK, DinamicResponse::success("Equipamento deletado", ()))
        }
        Ok(_) => ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Equipamento nao encontrado")),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao deletar equipamento: {}", e)),
        ),
    }
}

pub async fn buscar(
    State(state): State<AppState>,
    Query(filtro): Query<FiltroEquipamentoDto>,
) -> ApiResponse<Vec<Equipamento>> {
    let equipamentos = sqlx::query_as::<_, Equipamento>(
        r#"SELECT id, uuid, nome, descricao, estado,
        data_aquisicao, peso_kg, largura_cm, altura_cm, comprimento_cm,
        ultima_vez_disponivel, ultima_vez_em_manutencao, criado_em, criado_por
        FROM equipamento
        WHERE ($1::text IS NULL OR nome ILIKE '%' || $1 || '%')
        AND ($2::text IS NULL OR descricao ILIKE '%' || $2 || '%')
        AND ($3::estado_equipamento IS NULL OR estado = $3)
        ORDER BY nome"#,
    )
    .bind(filtro.nome)
    .bind(filtro.descricao)
    .bind(filtro.estado)
    .fetch_all(&state.db)
    .await;

    match equipamentos {
        Ok(lista) => ApiResponse(StatusCode::OK, DinamicResponse::success("Equipamentos encontrados", lista)),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao buscar equipamentos {e}")),
        ),
    }
}
