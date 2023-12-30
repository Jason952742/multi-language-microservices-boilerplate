use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tera::Tera;

pub mod entities;

#[derive(Clone)]
pub struct AppState {
    pub(crate) templates: Tera,
    pub(crate) conn: DatabaseConnection,
}

#[derive(Deserialize)]
pub struct Params {
    pub(crate) page: Option<u64>,
    pub(crate) posts_per_page: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FlashData {
    pub(crate) kind: String,
    pub(crate) message: String,
}
