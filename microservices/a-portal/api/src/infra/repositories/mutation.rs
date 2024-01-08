use shared::mongo::MongoPool;
use shared::{mongodb};
use shared::mongodb::bson::oid::ObjectId;
use shared::mongodb::{Client, Collection};
use crate::domain::entities::post;

pub struct PostDbMutation;

impl PostDbMutation {
    fn get_coll(client: &Client) -> Collection<post::Model> {
        client.database("multi_lang").collection::<post::Model>("post")
    }

    pub async fn create_post(
        client: &Client,
        form_data: post::Model,
    ) -> Result<ObjectId, mongodb::error::Error> {
        let collection = Self::get_coll(client);

        let created_id = MongoPool::create_item(collection, form_data).await?;
        Ok(created_id)
    }

    pub async fn update_post_by_id(
        client: &Client,
        id: ObjectId,
        form_data: post::Model,
    ) -> Result<(), mongodb::error::Error> {
        let collection = Self::get_coll(client);

        MongoPool::update_item(collection, id, form_data).await?;

        Ok(())
    }

    pub async fn delete_post(
        client: &Client,
        id: ObjectId,
    ) -> Result<(), mongodb::error::Error> {
        let collection = Self::get_coll(client);

        MongoPool::delete_item(collection.clone(), id).await?;

        Ok(())
    }

    pub async fn delete_all_post(
        client: &Client
    ) -> Result<(), mongodb::error::Error> {
        let collection = Self::get_coll(client);

        MongoPool::delete_all(collection.clone()).await?;

        Ok(())
    }
}
