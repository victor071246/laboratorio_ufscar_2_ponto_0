use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "status_agendamento", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum StatusAgendamento {
    Pendente,
    Confirmado,
    EmUso,
    Concluido,
    Cancelado
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Agendamento {
    pub id: i32,
    pub uuid: Uuid,
    pub equipamento_id: i32,
    pub usuario_id: i32,
    pub status: StatusAgendamento,
    pub data_inicio: DateTime<Utc>,
    pub data_fim: DateTime<Utc>,
    pub notificar_email: bool,
    pub notificar_whatsapp: bool,
    pub observacao: Option<String>,
    pub criado_em: DateTime<Utc>
}