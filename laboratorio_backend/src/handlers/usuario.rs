use axum::extract::State;
use axum::http::StatusCode;

use crate::models::filtro::FiltroDto;
use crate::models::usuario::{Usuario, PapelUsuario};
use crate::response::{ApiResponse, DinamicResponse};
use crate::AppState;

pub async fn listar_todos_usuarios(
    State(state): State<AppState>,
) -> ApiResponse<Vec<Usuario>> {
    let usuarios = sqlx::query_as!(
        Usuario,
        r#"SELECT id, uuid, nome, email, telefone, senha_hash,
            papel as "papel: PapelUsuario", ativo, criado_em, criado_por
            FROM usuario ORDER BY nome"#
    )
    .fetch_all(&state.db)
    .await;

    match usuarios {
        Ok(lista) => ApiResponse(
            StatusCode::OK,
            DinamicResponse::success("Usuários listados", lista),
        ),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao buscar usuários: {}", e)),
        )
    }
}

pub async fn listar_campos_usuario(
 State(state): State<AppState>   
) -> ApiResponse<Vec<String>> {
    let colunas = sqlx::query!(
        "SELECT column_name FROM information_schema.columns WHERE table_name = 'usuario'"
    )
    .fetch_all(&state.db)
    .await
    .map(|rows| rows.into_iter().map(|r| r.column_name.unwrap_or_default()).collect::<Vec<String>>());

    match colunas {
        Ok(lista) => ApiResponse(
            StatusCode::OK,
            DinamicResponse::success("Colunas listadas", lista),
        ),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao listar colunas: {}", e))
        ),
    }
}

pub async fn busca_com_filtro(
    State(state): State<AppState>,
    Query(filtro): Query<FiltroDto>
) -> ApiResponse<Vec<Usuario>> {
    let operador = match filtro.operador_as_str() {
        "gt" | ">" => ">",
        "lt" | "<" => "<",
        "gte" | ">=" => ">=",
        "lte" | "<=" => "<=",
        _ => "=",
    };

    let sql = format!(
        r#"SELECT id, uuid, nome, email, telefone, senha_hash, papel as "papel: PapelUsuario", ativo, criado_em, criado_por FROM usuario WHERE {} {} $1 ORDER BY nome"#,
        filtro.campo, operador
    );

    let usuarios = sqlx::query_as::<_, Usuario>(&sql)
        .bind(filtro.valor)
        .fetch_all(&state.db)
        .await;

    match usuarios {
        Ok(lista) => ApiResponse(StatusCode::OK, DinamicResponse::success("Usuários encontrados", lista)),
        Err(e) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro ao buscar usuários: {}", e)))
    }
}