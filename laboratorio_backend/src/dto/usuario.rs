use serde::Deserialize;
use crate::models::usuario::PapelUsuario;

#[derive(Debug, Deserialize)]
pub struct CriarUsuarioDto {
    pub nome: String,
    pub email: String,
    pub senha: String,
    pub papel: Option<PapelUsuario>,
    pub telefone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub senha: String,
}

#[derive(Debug, Deserialize)]
pub struct AtualizarUsuarioDto {
    pub nome: String,
    pub email: String,
    pub senha: String,
    pub papel: Option<PapelUsuario>,
    pub telefone: Option<String>
}

