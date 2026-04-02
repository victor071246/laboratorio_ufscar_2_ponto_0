use axum::{extract::State, http::StatusCode, Json};

use crate::dto::usuario::{CriarUsuarioDto, LoginDto};
use crate::models::usuario::PapelUsuario;
use crate::responses::ApiResponse;
use crate::services::jwt::{gerar_hash, gerar_jwt, verificar_senha};
use crate::AppState;

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginDto>,
) -> (StatusCode, Json<ApiResponse<LoginResponse>>) {
    let usuario = sqlx::query_as!(
        crate::models::usuario::Usuario,
        r#"SELECT id, uuid, nome, email, telefone, senha_hash, papel as "papel: PapelUsuario", ativo, criado_em, criado_por
            FROM usuario WHERE email = $1 AND ativo = true"#,
            payload.email
    )
    .fetch_optional(&state.db)
    .await;

    let usuario = match usuario {
        Ok(Some(u)) => u,
        Ok(None) => return ApiResponse::error(StatusCode::UNAUTHORIZED, "Credenciais inválidas"),
        Err(_) => return ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "Erro interno"),
    };

    match verificar_senha(&payload.senha, &usuario.senha_hash) {
        Ok(true) => {}
        _ => return ApiResponse::error(StatusCode::UNAUTHORIZED, "Credenciais inválidas")
    }

    let token = match gerar_jwt(
        &usuario.uuid.to_string(),
        usuario.id,
        &usuario.email,
        usuario.papel,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    ) {
        Ok(t) => t,
        Err(_) => return ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "Erro ao gerar token"),
    };

    ApiResponse::success("Login realizado", LoginResponse { token })

}

pub async fn registrar(
    State(state): State<AppState>,
    Json(payload): Json<CriarUsuarioDto>,
) -> (StatusCode, Json<ApiResponse<()>>) {
    let existe = sqlx::query_scalar!(
    "SELECT EXISTS(SELECT 1 FROM usuario WHERE email = $1)",
    payload.email)
    .fetch_one(&state.db)
    .await;

    match existe {
        Ok(Some(true)) => return ApiResponse::error(StatusCode::CONFLICT, "Email já cadastrado"),
        Err(_) => return ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "Erro interno"),
        _ => {}
    }

    let senha_hash = match gerar_hash(&payload.senha) {
        Ok(h) => h,
        Err(_) => return ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "Erro ao processar a senha"),
    };

    let papel = payload.papel.unwrap_or(PapelUsuario::Aluno);

    let result = sqlx::query!(
        r#"INSERT INTO usuario (nome, email, senha_hash, papel, telefone)
            VALUES ($1, $2, $3, $4, $5)"#,
        payload.nome,
        payload.email,
        senha_hash,
        papel as PapelUsuario,
        payload.telefone
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => ApiResponse::success("Usuário criado", ()),
        Err(_) => ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, "Erro ao criar usuário")
    }
}