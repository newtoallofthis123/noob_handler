use mongodb::{Client, bson::doc};
use serde::{Deserialize, Serialize};
use crate::utils::get_mongo_url;
use mongodb::bson::oid::ObjectId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Go {
    _id: ObjectId,
    slug: String,
    url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Page{
    pub _id: ObjectId,
    pub hash: String,
    pub name: String,
    pub content: String,
    pub date: String,
    pub author: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Code{
    pub _id: ObjectId,
    pub hash: String,
    pub title: String,
    pub content: String,
    pub lang: String,
    pub author: String,
}

async fn get_connection() -> Result<Client, mongodb::error::Error> {
    let client = Client::with_uri_str(get_mongo_url()).await?;
    Ok(client)
}

pub async fn get_db() -> Result<mongodb::Database, mongodb::error::Error> {
    let client = get_connection().await?;
    let db = client.database("updates");
    Ok(db)
}

pub async fn _get_go(slug: &str)->Go{
    let db = get_db().await.unwrap();
    let collection = db.collection::<Go>("go");
    let result = collection.find_one(doc! {"slug": slug}, None).await.unwrap();
    result.unwrap()
}

pub async fn get_page(hash: &str)->Page{
    let db = get_db().await.unwrap();
    let collection = db.collection::<Page>("page");
    let result = collection.find_one(doc! {"hash": hash}, None).await.unwrap();
    result.expect("Page Not Found")
}

pub async fn update_page(hash: &str, page:&Page)->mongodb::results::UpdateResult{
    let db = get_db().await.unwrap();
    let collection = db.collection::<Page>("page");
    let result = collection.update_one(doc! {"hash": hash}, doc! {"$set": 
        {
            "name": &page.name,
            "content": &page.content,
            "date": &page.date,
            "author": &page.author,
        }
}, None).await.unwrap();
    result
}

pub async fn insert_page(page:&Page)->mongodb::results::InsertOneResult{
    let db = get_db().await.unwrap();
    let collection = db.collection::<Page>("page");
    let result = collection.insert_one(page.clone(), None).await.unwrap();
    result
}

pub async fn get_code(hash: &str)->Code{
    let db = get_db().await.unwrap();
    let collection = db.collection::<Code>("code");
    let result = collection.find_one(doc! {"hash": hash}, None).await.unwrap();
    result.expect("Page Not Found")
}