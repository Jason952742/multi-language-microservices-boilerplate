use futures::{stream, TryStreamExt};
use tokio_stream::StreamExt;
use shared::mongo::MongoPool;
use shared::mongodb;
use shared::mongodb::{Client, Collection};
use shared::mongodb::bson::Document;
use shared::mongodb::bson::oid::ObjectId;
use shared::mongodb::options::FindOptions;
use crate::domain::entities::post;

pub struct PostOrmQuery;

impl PostOrmQuery {

    fn get_coll(client: &Client) -> Collection<post::Model> {
        client.database("multi_lang").collection::<post::Model>("post")
    }

    pub async fn find_post_by_id(
        client: &Client,
        id: ObjectId
    ) -> Result<Option<post::Model>, mongodb::error::Error> {
        let collection = Self::get_coll(client);

        Ok(MongoPool::find_item_by_id(collection, id).await?)
    }

    /// If ok, returns (post models, num pages).
    pub(crate) async fn find_posts_in_page(
        client: &Client,
        filter: Document,
        find_options: Option<FindOptions>,
        page: u32,
        per_page: u32,
    ) -> Result<(u32, Vec<post::Model>), mongodb::error::Error> {
        let collection = Self::get_coll(client);

        // Statistics on the total number of documents that satisfy the query conditions
        let total_items = collection.count_documents(filter.clone(), None).await?;

        // Counting the number of skipped documents
        let skip = (page - 1) * per_page;

        // Setting the number of skipped documents to FindOptions
        let mut options = find_options.unwrap_or(FindOptions::default());
        options.skip = Some(skip as i64 as u64);

        // perform a search
        let cursor = collection.find(filter, options).await?;
        let book_stream = stream::try_unfold(cursor, |mut cursor| async move {
            match StreamExt::try_next(&mut cursor).await {
                Ok(Some(item)) => Ok(Some((item, cursor))),
                Ok(None) => Ok(None),
                Err(err) => Err(err),
            }
        });
        let books: Vec<post::Model> = book_stream.try_collect().await?;

        // Calculate the total number of pages
        let total_pages = (total_items as f32 / per_page as f32).ceil() as u32;

        Ok((total_pages, books))
    }
}
