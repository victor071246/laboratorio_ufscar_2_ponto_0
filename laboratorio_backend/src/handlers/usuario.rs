use axum::extract::{Query, State, Path};
use axum::http::StatusCode;
use uuid::Uuid;

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
        ),
    }
}

pub async fn listar_campos_usuario(
    State(state): State<AppState>,
) -> ApiResponse<Vec<String>> {
    let colunas = sqlx::query_scalar::<_, String>(
        "SELECT column_name FROM information_schema.columns WHERE table_name = 'usuario'"
    )
    .fetch_all(&state.db)
    .await;

    match colunas {
        Ok(lista) => ApiResponse(
            StatusCode::OK,
            DinamicResponse::success("Colunas listadas", lista),
        ),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao listar colunas: {}", e)),
        ),
    }
}

pub async fn busca_com_filtro(
    State(state): State<AppState>,
    Query(filtro): Query<FiltroDto>,
) -> ApiResponse<Vec<Usuario>> {
    let campos_permitidos = [
        "id", "uuid", "nome", "email", "telefone",
        "senha_hash", "papel", "ativo", "criado_em", "criado_por",
    ];

    if !campos_permitidos.contains(&filtro.campo.as_str()) {
        return ApiResponse(
            StatusCode::BAD_REQUEST,
            DinamicResponse::error("Campo inválido"),
        );
    }

    let (comparador, valor) = match filtro.operador.as_str() {
        "igual" | "=" | "==" => ("=", filtro.valor),
        _ => ("ILIKE", format!("%{}%", filtro.valor)),
    };

    let sql = format!(
        r#"SELECT id, uuid, nome, email, telefone, senha_hash,
            papel as "papel: PapelUsuario", ativo, criado_em, criado_por
            FROM usuario WHERE {}::text {} $1 ORDER BY nome"#,
        filtro.campo, comparador
    );

    let usuarios = sqlx::query_as::<_, Usuario>(&sql)
        .bind(valor)
        .fetch_all(&state.db)
        .await;

    match usuarios {
        Ok(lista) => ApiResponse(
            StatusCode::OK,
            DinamicResponse::success("Usuários encontrados", lista),
        ),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao buscar usuários: {}", e)),
        ),
    }
}

pub async fn buscar_por_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> ApiResponse<Usuario> {
    let usuario = sqlx::query_as!(
        Usuario,
        r#"SELECT id, uuid, nome, email, telefone, senha_hash,
            papel as "papel: PapelUsuario", ativo, criado_em, criado_por
            FROM usuario WHERE uuid = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await;

    match usuario {
        Ok(Some(u)) => ApiResponse(StatusCode::OK, DinamicResponse::success("Usuário encontrado", u)),
        Ok(None) => ApiResponse(StatusCode::NOT_FOUND, DinamicResponse::error("Usuário não encontrado")),
        Err(e) => ApiResponse(StatusCode::INTERNAL_SERVER_ERROR, DinamicResponse::error(format!("Erro ao buscar usuário: {}", e)))
    }
}