use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "papel_usuario", rename_all = "snake_case")]
#[serde(rename_all="snake_case")]
pub enum PapelUsuario {
    Admin,
    Supervisor,
    Aluno
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Usuario {
    pub id: i32,
    pub uuid: Uuid,
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub papel: PapelUsuario,
    pub telefone: Option<String>,
    pub ativo: bool,
    pub criado_em: DateTime<Utc>,
    pub criado_por: Option<i32>
}

impl std::fmt::Display for PapelUsuario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PapelUsuario::Aluno => write!(f, "aluno"),
            PapelUsuario::Supervisor => write!(f, "supervisor"),
            PapelUsuario::Admin => write!(f, "admin"),
        }
    }
}