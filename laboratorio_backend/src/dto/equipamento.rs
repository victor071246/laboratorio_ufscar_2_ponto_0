use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::models::equipamento::EstadoEquipamento;

#[derive(Debug, Deserialize)]
pub struct CriarEquipamentoDto {
    pub nome: String,
    pub descricao: Option<String>,
    pub estado: Option<EstadoEquipamento>,
    pub data_aquisicao: Option<NaiveDate>,
    pub peso_kg: Option<Decimal>,
    pub largura_cm: Option<Decimal>,
    pub altura_cm: Option<Decimal>,
    pub profundidade_cm: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct AtualizarEquipamentoDto {
    pub nome: Option<String>,
    pub descricao: Option<String>,
    pub estado: Option<EstadoEquipamento>,
    pub data_aquisicao: Option<NaiveDate>,
    pub peso_kg: Option<Decimal>,
    pub largura_cm: Option<Decimal>,
    pub altura_cm: Option<Decimal>,
    pub profundidade_cm: Option<Decimal>
}

#[derive(Debug, Serialize)]
pub struct EquipamentoResponse {
    pub id: i32,
    pub uuid: uuid::Uuid,
    pub nome: String,
    pub descricao: Option<String>,
    pub estado: EstadoEquipamento,
    pub data_aquisicao: Option<NaiveDate>,
    pub peso_kg: Option<Decimal>,
    pub altura_cm: Option<Decimal>,
    pub profundidade_cm: Option<Decimal>,
}