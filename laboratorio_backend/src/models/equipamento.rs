use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "estado_equipamento", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EstadoEquipamento {
    Disponivel,
    EmManutencao,
    Quebrado,
    Reservado,
    Desativado
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Equipamento {
    pub id: i32,
    pub uuid: Uuid,
    pub nome: String,
    pub descricao: Option<String>,
    pub estado: EstadoEquipamento,
    pub data_aquisicao: Option<NaiveDate>,
    pub altura_cm: Option<Decimal>,
    pub peso_kg: Option<Decimal>,
    pub largura_cm: Option<Decimal>,
    pub profundidade_cm: Option<Decimal>,
    pub ultima_vez_disponivel: Option<DateTime<Utc>>,
    pub ultima_vez_em_manutencao: Option<DateTime<Utc>>,
    pub criado_em: DateTime<Utc>,
    pub criado_por: Option<i32>,
}