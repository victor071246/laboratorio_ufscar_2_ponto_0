use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FiltroDto {
    pub campo: String,
    pub operador: String,
    pub valor: String,
}