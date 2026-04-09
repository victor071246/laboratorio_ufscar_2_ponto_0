use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;
use crate::models::agendamento::StatusAgendamento;

#[derive(Debug, Deserialize)]
pub struct CriarAgendamentoDto {
    pub equipamento_uuid: Uuid,
    pub data_inicio: DateTime<Utc>,
    pub data_fim: DateTime<Utc>,
    pub notificar_email: Option<bool>,
    pub notificar_whatsapp: Option<bool>,
    pub observacao: Option<String>,
}

#[derive(Debug,Deserialize)]
pub struct AtualizarAgendamentoDto {
    pub status: Option<StatusAgendamento>,
    pub data_inicio: Option<DateTime<Utc>>,
    pub data_fim: Option<DateTime<Utc>>,
    pub notificar_email: Option<bool>,
    pub notificar_whatsapp: Option<bool>,
    pub observacao: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct FiltroAgendamentoDto {
    pub equipamento_id: Option<i32>,
    pub usuario_id: Option<i32>,
    pub status: Option<StatusAgendamento>,
    pub data_inicio_min: Option<DateTime<Utc>>,
    pub data_inicio_max: Option<DateTime<Utc>>,
    pub observacao: Option<String>,
}