use serde::Deserialize;

#[derive(Deserialize)]
pub struct FiltroDto {
    #[serde(default)]
    pub campo: String,
    #[serde(default)]
    pub operador: String,
    #[serde(default)]
    pub valor: String
}
