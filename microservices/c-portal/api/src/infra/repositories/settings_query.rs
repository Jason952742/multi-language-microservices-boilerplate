use futures::{stream, TryStreamExt};
use tokio_stream::StreamExt;
use shared::mongo::MongoPool;
use shared::mongodb;
use shared::mongodb::{Client, Collection};
use shared::mongodb::bson::Document;
use shared::mongodb::bson::oid::ObjectId;
use shared::mongodb::options::FindOptions;
use crate::domain::entities::user_settings;

pub struct SettingsDbQuery;

impl SettingsDbQuery {

    fn get_coll(client: &Client) -> Collection<user_settings::Model> {
        client.database("multi_lang").collection::<user_settings::Model>("user_settings")
    }

    pub async fn find_settings_by_id(
        client: &Client,
        id: ObjectId
    ) -> Result<Option<user_settings::Model>, mongodb::error::Error> {
        let collection = Self::get_coll(client);

        Ok(MongoPool::find_item_by_id(collection, id).await?)
    }

    pub(crate) async fn find_settings_in_page(
        client: &Client,
        filter: Document,
        find_options: Option<FindOptions>,
        page: u64,
        limit: u64,
    ) -> Result<(u64, Vec<user_settings::Model>), mongodb::error::Error> {
        let collection = Self::get_coll(client);

        // Statistics on the total number of documents that satisfy the query conditions
        let total_items = collection.count_documents(filter.clone(), None).await?;

        // Counting the number of skipped documents
        let offset = (page - 1) * limit;

        // Setting the number of skipped documents to FindOptions
        let mut options = find_options.unwrap_or(FindOptions::default());
        options.skip = Some(offset);

        // perform a search
        let cursor = collection.find(filter, options).await?;
        let book_stream = stream::try_unfold(cursor, |mut cursor| async move {
            match StreamExt::try_next(&mut cursor).await {
                Ok(Some(item)) => Ok(Some((item, cursor))),
                Ok(None) => Ok(None),
                Err(err) => Err(err),
            }
        });
        let books: Vec<user_settings::Model> = book_stream.try_collect().await?;

        // Calculate the total number of pages
        let total_pages = (total_items as f64 / limit as f64).ceil() as u64;

        Ok((total_pages, books))
    }
}
