use futures::StreamExt;
use mongodb::{Client, bson::doc, options::FindOptions};
use serde::{Deserialize, Serialize};
use crate::utils::get_mongo_url;
use mongodb::bson::oid::ObjectId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Go {
    pub _id: ObjectId,
    pub slug: String,
    pub url: String,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Specials{
    pub _id: ObjectId,
    pub tag: String,
    pub title: String,
    pub description: String,
    pub current: String,
    pub date: String,
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

async fn get_page_conn() -> mongodb::Collection<Page> {
    let db = get_db().await.unwrap();
    let collection = db.collection::<Page>("page");
    collection
}

async fn get_code_conn()->mongodb::Collection<Code>{
    let db = get_db().await.unwrap();
    let collection = db.collection::<Code>("code");
    collection
}

async fn get_go_conn()->mongodb::Collection<Go>{
    let db = get_db().await.unwrap();
    let collection = db.collection::<Go>("go");
    collection
}

async fn get_specials_conn()->mongodb::Collection<Specials>{
    let db = get_db().await.unwrap();
    let collection = db.collection::<Specials>("specials");
    collection
}

pub async fn get_page(hash: &str)->Page{
    let collection = get_page_conn().await;
    let result = collection.find_one(doc! {"hash": hash}, None).await.unwrap();
    result.expect("Page Not Found")
}

pub async fn update_page(hash: &str, page:&Page)->mongodb::results::UpdateResult{
    let collection = get_page_conn().await;
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
    let collection = get_page_conn().await;
    let result = collection.insert_one(page.clone(), None).await.unwrap();
    result
}

pub async fn delete_page(hash: &str)->mongodb::results::DeleteResult{
    let collection = get_page_conn().await;
    let result = collection.delete_one(doc! {"hash": hash}, None).await.unwrap();
    result
}

pub async fn get_code(hash: &str)->Code{
    let collection = get_code_conn().await;
    let result = collection.find_one(doc! {"hash": hash}, None).await.unwrap();
    result.expect("Code Not Found")
}

pub async fn update_code(hash: &str, code:&Code)->mongodb::results::UpdateResult{
    let collection = get_code_conn().await;
    let result = collection.update_one(doc! {"hash": hash}, doc! {"$set": 
        {
            "title": &code.title,
            "content": &code.content,
            "lang": &code.lang,
            "author": &code.author,
        }
}, None).await.unwrap();
    result
}

pub async fn insert_code(code:&Code)->mongodb::results::InsertOneResult{
    let collection = get_code_conn().await;
    let result = collection.insert_one(code.clone(), None).await.unwrap();
    result
}

pub async fn delete_code(hash: &str)->mongodb::results::DeleteResult{
    let collection = get_code_conn().await;
    let result = collection.delete_one(doc! {"hash": hash}, None).await.unwrap();
    result
}

pub async fn get_go(slug: &str)->Go{
    let collection = get_go_conn().await;
    let result = collection.find_one(doc! {"slug": slug}, None).await.unwrap();
    result.unwrap()
}

pub async fn update_go(slug: &str, go:&Go)->mongodb::results::UpdateResult{
    let collection = get_go_conn().await;
    let result = collection.update_one(doc! {"slug": slug}, doc! {"$set": 
        {
            "url": &go.url,
        }
}, None).await.unwrap();
    result
}

pub async fn insert_go(go:&Go)->mongodb::results::InsertOneResult{
    let collection = get_go_conn().await;
    let result = collection.insert_one(go.clone(), None).await.unwrap();
    result
}

pub async fn delete_go(slug: &str)->mongodb::results::DeleteResult{
    let collection = get_go_conn().await;
    let result = collection.delete_one(doc! {"slug": slug}, None).await.unwrap();
    result
}

pub async fn get_all_pages()->Vec<Page>{
    let collection = get_page_conn().await;
    let find_options = FindOptions::builder().build();
    let mut result = collection.find(None, find_options).await.unwrap();
    let mut result_vec = Vec::new();
    while let Some(doc) = result.next().await {
        result_vec.push(doc.expect("Failed to push"));
    }
    result_vec
}

pub async fn get_all_codes()->Vec<Code>{
    let collection = get_code_conn().await;
    let find_options = FindOptions::builder().build();
    let mut result = collection.find(None, find_options).await.unwrap();
    let mut result_vec = Vec::new();
    while let Some(doc) = result.next().await {
        result_vec.push(doc.expect("Failed to push"));
    }
    result_vec
}

pub async fn _get_all_go()->Vec<Go>{
    let collection = get_go_conn().await;
    let find_options = FindOptions::builder().build();
    let mut result = collection.find(None, find_options).await.unwrap();
    let mut result_vec = Vec::new();
    while let Some(doc) = result.next().await {
        result_vec.push(doc.expect("Failed to push"));
    }
    result_vec
}

pub async fn get_specials()->Specials{
    let collection = get_specials_conn().await;
    let result = collection.find_one(doc! {"tag": "special"}, None).await.unwrap();
    result.expect("Specials Not Found")
}

pub async fn set_specials(special:&Specials)->mongodb::results::UpdateResult{
    let collection = get_specials_conn().await;
    let result = collection.update_one(doc! {"tag": "special"}, doc! {"$set": 
        {
            "title": &special.title,
            "description": &special.description,
            "current": &special.current,
            "date": &special.date,
        }
}, None).await.unwrap();
    result
}