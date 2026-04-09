use serde::Deserialize;
use uuid::Uuid;

use crate::models::ocorrencia::TipoOcorrencia;

#[derive(Debug, Deserialize)]
pub struct CriarOcorrenciaDto {
    pub equipamento_uuid: Uuid,
    pub tipo: TipoOcorrencia,
    pub descricao: String,
}

#[derive(Debug, Deserialize)]
pub struct FiltroOcorrenciaDto {
    pub equipamento_id: Option<i32>,
    pub tipo: Option<TipoOcorrencia>,
    pub resolvida: Option<bool>,
}