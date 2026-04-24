use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use crate::dto::usuario::{CriarUsuarioDto, LoginDto};
use crate::models::usuario::PapelUsuario;
use crate::response::{ApiResponse, DinamicResponse};
use crate::services::jwt::{gerar_hash, gerar_jwt, verificar_senha};
use crate::AppState;

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<LoginDto>
) -> Result<(CookieJar, ApiResponse<()>), ApiResponse<()>> {
    let usuario = sqlx::query_as!(
        crate::models::usuario::Usuario,
        r#"SELECT id, uuid, nome, email, telefone, senha_hash,
            papel as "papel: PapelUsuario", ativo, criado_em, criado_por
            FROM usuario WHERE email = $1 AND ativo = true"#,
            payload.email
    )
    .fetch_optional(&state.db)
    .await;

    let usuario = match usuario {
        Ok(Some(u)) => u,
        Ok(None) => return Err(ApiResponse(
            StatusCode::UNAUTHORIZED,
            DinamicResponse::error("Credenciais inválidas"),
        )),
        Err(_) => return Err(ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error("Erro interno"),
        ))
    };

    if !verificar_senha(&payload.senha, &usuario.senha_hash) {
        return Err(ApiResponse(
            StatusCode::UNAUTHORIZED,
            DinamicResponse::error("Credenciais inválidas"),
        ))
    }

    let token = gerar_jwt(
        &usuario.uuid.to_string(),
        usuario.id,
        &usuario.email,
        usuario.papel.clone(),
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    );

    let cookie = Cookie::build(("token", token))
        .http_only(true)
        .path("/")
        .build();

    Ok((
        jar.add(cookie),
        ApiResponse(StatusCode::OK, DinamicResponse::success("Login realizado", ()))
    ))
}

pub async fn registrar(
    State(state): State<AppState>,
    Json(payload): Json<CriarUsuarioDto>,
) -> ApiResponse<()> {
    let existe_email = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM usuario WHERE email = $1)",
        payload.email
    )
    .fetch_one(&state.db)
    .await;

    match existe_email {
        Ok(Some(true)) => return ApiResponse(
            StatusCode::CONFLICT,
            DinamicResponse::error("Email já cadastrado"),
        ),
        Err(e) => return ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro interno: {}", e)),
        ),
        _ => {}
    };

    let senha_hash = gerar_hash(&payload.senha);

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
        Ok(_) => ApiResponse(
            StatusCode::CREATED,
            DinamicResponse::success("Usuário criado", ()),
        ),
        Err(e) => ApiResponse(
            StatusCode::INTERNAL_SERVER_ERROR,
            DinamicResponse::error(format!("Erro ao criar usuário: {}", e)),
        )
    }


}