use chrono::Utc;
use serde_derive::{Deserialize, Serialize};
use shared::bson::serde_helpers::serialize_object_id_as_hex_string;
use uuid::Uuid;
use validator::Validate;
use shared::utils::{bson_uuid_to_uuid, uuid_to_bson_uuid};
use shared::bson::DateTime;
use shared::bson::oid::ObjectId;
use crate::domain::entities::user_settings;

#[derive(Default, Debug, Validate, Deserialize)]
pub struct UserSettingsForm {
    pub user_id: Uuid,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub theme: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub language: String,
}

impl Into<user_settings::Model> for UserSettingsForm {
    fn into(self) -> user_settings::Model {
        let now: DateTime = Utc::now().into();
        user_settings::Model {
            id: None,
            user_id: uuid_to_bson_uuid(self.user_id),
            theme: self.theme,
            font_size: 12,
            background_color: "blue".to_string(),
            language: self.language,
            notifications_enabled: false,
            auto_dark_mode_enabled: false,
            timezone: "utc".to_string(),
            sidebar_enabled: false,
            sidebar_width: 20,
            updated_at: now,
            created_at: now,
        }
    }
}


#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct UserSettingsItem {
    #[serde(rename = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub user_id: Uuid,
    pub theme: String,
    pub language: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl From<user_settings::Model> for UserSettingsItem {
    fn from(value: user_settings::Model) -> Self {
       Self {
           id: value.id.unwrap(),
           user_id: bson_uuid_to_uuid(value.user_id),
           theme: value.theme,
           language: value.language,
           created_at: value.created_at.into(),
           updated_at: value.updated_at.into()
       }
    }
}