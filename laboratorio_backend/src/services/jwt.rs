use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

use crate::models::usuario::{PapelUsuario};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // uuid - identificador público
    pub id: i32, // id inteiro - uso interno nas queries
    pub email: String,
    pub papel: PapelUsuario,
    pub exp: i64, // expiration em unix timestamp
    pub iat: i64 // emitido em unix timestamp
}

pub fn gerar_hash(senha: &str) -> String {
    hash(senha, DEFAULT_COST)
    .expect("Erro ao gerar hash da senha")
}

pub fn verificar_senha(senha: &str, hash: &str) -> bool {
    verify(senha, hash)
    .expect("Erro ao verificar a senha")
}

pub fn gerar_jwt(
    uuid: &str,
    id: i32,
    email: &str,
    papel: PapelUsuario,
    secret: &str,
    exp_hours: i64,
) -> String {
    let agora = Utc::now();
    let claims = Claims {
        sub: uuid.to_string(),
        id,
        email: email.to_string(),
        papel,
        exp: (agora + Duration::hours(exp_hours)).timestamp(),
        iat: agora.timestamp()
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Erro ao gerar JWT")
}