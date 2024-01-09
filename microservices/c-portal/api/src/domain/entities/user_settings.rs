use serde_derive::{Deserialize, Serialize};
use shared::bson::{Bson, DateTime, Uuid};
use shared::mongodb::bson::oid::ObjectId;
use shared::bson;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Model {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: Uuid,
    pub theme: String,
    pub font_size: u8,
    pub background_color: String,
    pub language: String,
    pub notifications_enabled: bool,
    pub auto_dark_mode_enabled: bool,
    pub timezone: String,
    pub sidebar_enabled: bool,
    pub sidebar_width: u16,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Into<Bson> for Model {
    fn into(self) -> Bson {
        Bson::from(bson::to_document(&self).unwrap())
    }
}
