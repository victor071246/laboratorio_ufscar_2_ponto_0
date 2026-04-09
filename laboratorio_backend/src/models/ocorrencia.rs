use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::equipamento::EstadoEquipamento;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "tipo_ocorrencia", rename_all = "snake_case")]
pub enum TipoOcorrencia {
    Manutencao,
    Defeito,
    Acidente,
    Outro
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Ocorrencia {
    pub id: i32,
    pub uuid: Uuid,
    pub equipamento_id: i32,
    pub registrado_por: Option<i32>,
    pub tipo: TipoOcorrencia,
    pub descricao: String,
    pub estado_anterior: Option<EstadoEquipamento>,
    pub removida_por_prazo: bool,
    pub resolvida_em: Option<DateTime<Utc>>,
    pub criado_em: DateTime<Utc>,
}