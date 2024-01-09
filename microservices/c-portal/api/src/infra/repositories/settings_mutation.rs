use shared::mongo::MongoPool;
use shared::{mongodb};
use shared::mongodb::bson::oid::ObjectId;
use shared::mongodb::{Client, Collection};
use crate::domain::entities::{user_settings};

pub struct SettingsDbMutation;

impl SettingsDbMutation {
    fn get_coll(client: &Client) -> Collection<user_settings::Model> {
        client.database("multi_lang").collection::<user_settings::Model>("user_settings")
    }

    pub async fn create_settings(
        client: &Client,
        form_data: user_settings::Model,
    ) -> Result<ObjectId, mongodb::error::Error> {
        let collection = Self::get_coll(client);

        let created_id = MongoPool::create_item(collection, form_data).await?;
        Ok(created_id)
    }

    pub async fn update_settings_by_id(
        client: &Client,
        id: ObjectId,
        form_data: user_settings::Model,
    ) -> Result<(), mongodb::error::Error> {
        let collection = Self::get_coll(client);

        MongoPool::update_item(collection, id, form_data).await?;

        Ok(())
    }

    pub async fn delete_settings(
        client: &Client,
        id: ObjectId,
    ) -> Result<(), mongodb::error::Error> {
        let collection = Self::get_coll(client);

        MongoPool::delete_item(collection.clone(), id).await?;

        Ok(())
    }
}
