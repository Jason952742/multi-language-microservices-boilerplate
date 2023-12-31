use mongodb::bson::{Bson, doc, Document};
use mongodb::{bson, Client, Collection};
use mongodb::options::{ClientOptions, FindOptions, UpdateOptions};
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use futures::stream::TryStreamExt;
use futures::{stream, TryStream};
use mongodb::bson::oid::ObjectId;
use serde::de::DeserializeOwned;
use tokio_stream::StreamExt;

#[derive(Debug)]
pub struct MongoPool;

static CONN: OnceCell<Client> = OnceCell::const_new();

impl MongoPool {
    pub async fn conn() -> &'static Client {
        CONN.get_or_init(|| async {
            let mut options = ClientOptions::parse("mongodb://localhost:27017").await.expect("mongodb option error");
            options.app_name = Some("MultiLang".to_string());

            let client = Client::with_options(options).expect("mongodb connection failed");
            client
        }).await
    }

    async fn create_item<T: Serialize>(collection: Collection<T>, item: T) -> Result<(), mongodb::error::Error> {
        let m = collection.insert_one(item, None).await?;
        Ok(())
    }

    async fn find_item_by_id<T>(collection: Collection<T>, id: ObjectId) -> Result<Option<T>, mongodb::error::Error>
        where
            T: DeserializeOwned + Unpin + Send + Sync,
    {
        let filter = doc! { "_id": id };
        let item = collection.find_one(filter, None).await?;
        Ok(item)
    }

    async fn find_items<T>(collection: Collection<T>, filter: Document, find_options: Option<FindOptions>) -> Result<Vec<T>, mongodb::error::Error>
        where
            T: DeserializeOwned + Unpin,
    {
        let cursor = collection.find(filter, find_options).await?;
        let items: Vec<T> = cursor.try_collect().await?;
        Ok(items)
    }

    async fn update_item<T>(collection: Collection<T>, id: ObjectId, new_item: T) -> Result<(), mongodb::error::Error>
        where
            T: Serialize + Into<Bson>,
    {
        let filter = doc! { "_id": id };
        let update = doc! { "$set": new_item.into() };
        collection
            .update_one(filter, update, UpdateOptions::builder().upsert(Some(false)).build())
            .await?;
        Ok(())
    }

    async fn delete_item<T: Serialize>(collection: Collection<T>, id: ObjectId) -> Result<(), mongodb::error::Error> {
        let filter = doc! { "_id": id };
        collection.delete_one(filter, None).await?;
        Ok(())
    }
}

#[tokio::test]
async fn main() -> Result<(), mongodb::error::Error> {
    use futures::stream::TryStreamExt;
    use mongodb::{bson::doc, options::FindOptions};

    // Define a type that models our data.
    let client = MongoPool::conn().await;

    // Parameterize our collection with the model.
    let coll = client.database("items").collection::<Book>("in_stock");

    let item = Book {
        id: ObjectId::new(),
        title: "helloworld".to_string(),
        author: "George Orwell".to_string(),
    };
    coll.insert_one(item.clone(), None).await?;

    let filter = doc! { "_id": item.id };
    let found_item = coll.find_one(filter, None).await?;
    println!("Found Item: {:?}", found_item);

    let new_item = Book {
        id: item.id,
        title: "Wochao Item".to_string(),
        ..item
    };
    let filter = doc! { "_id": item.id };
    let update = doc! { "$set": <Book as Into<Bson>>::into(new_item) };
    coll
        .update_one(filter, update, UpdateOptions::builder().upsert(Some(false)).build())
        .await?;

    // Query the books in the collection with a filter and an option.
    let filter = doc! { "author": "George Orwell" };
    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = coll.find(filter, find_options).await?;
    // Iterate over the results of the cursor.
    while let Some(book) = tokio_stream::StreamExt::try_next(&mut cursor).await? {
        println!("found: {:?}", book);
    }

    let filter = doc! { "_id": item.id };
    coll.delete_one(filter, None).await?;
    let filter = doc! { "_id": item.id };
    let found_item = coll.find_one(filter, None).await?;
    println!("Found Item: {:?}", found_item);

    // let collection = client.database("items").collection::<Book>("sell");
    //
    // let book = Book {
    //     id: ObjectId::new(),
    //     title: "Example Item".to_string(),
    //     author: "John".to_string(),
    // };
    // MongoPool::create_item(collection.clone(), book.clone()).await?;
    //
    // let found_item = MongoPool::find_item_by_id(collection.clone(), book.id).await?;
    // println!("Found Item: {:?}", found_item);
    //
    // let updated_item = Book {
    //     id: book.id,
    //     title: "Updated Item".to_string(),
    //     ..book
    // };
    // MongoPool::update_item(collection.clone(), book.id, updated_item.clone()).await?;
    //
    // let filter = doc! { "author": "John"};
    // let find_options = None;
    // let found_items = find_books(collection.clone(), filter, find_options).await?;
    // println!("Found Items: {:?}", found_items);
    //
    // MongoPool::delete_item(collection.clone(), book.id).await?;
    //
    // let found_item = MongoPool::find_item_by_id(collection.clone(), book.id).await?;
    // println!("Found Item: {:?}", found_item);

    Ok(())
}

async fn find_books(
    collection: Collection<Book>,
    filter: Document,
    find_options: Option<FindOptions>,
) -> Result<Vec<Book>, mongodb::error::Error> {
    let cursor = collection.find(filter, find_options).await?;
    let book_stream = stream::try_unfold(cursor, |mut cursor| async move {
        match StreamExt::try_next(&mut cursor).await {
            Ok(Some(item)) => Ok(Some((item, cursor))),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    });
    let books: Result<Vec<Book>, mongodb::error::Error> = book_stream.try_collect().await;
    books
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Book {
    #[serde(rename = "_id")]
    id: ObjectId,
    title: String,
    author: String
}

impl Into<Bson> for Book {
    fn into(self) -> Bson {
        Bson::from(bson::to_document(&self).unwrap())
    }
}