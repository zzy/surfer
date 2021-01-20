use futures::stream::StreamExt;
use mongodb::Database;
use bson::oid::ObjectId;
use async_graphql::{Error, ErrorExtensions};

use crate::util::constant::GqlResult;
use crate::articles::models::{Article, NewArticle};

pub async fn add_article(db: Database, new_article: NewArticle) -> Article {
    let coll = db.collection("articles");

    let exist_document = coll
        .find_one(
            bson::doc! {"user_id": &new_article.user_id,  "subject": &new_article.subject},
            None,
        )
        .await
        .unwrap();
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let new_article_bson = bson::to_bson(&new_article).unwrap();

        if let bson::Bson::Document(document) = new_article_bson {
            // Insert into a MongoDB collection
            coll.insert_one(document, None)
                .await
                .expect("Failed to insert into a MongoDB collection!");
        } else {
            println!("Error converting the BSON object into a MongoDB document");
        };
    }

    let article_document = coll
        .find_one(
            bson::doc! {"user_id": &new_article.user_id,  "subject": &new_article.subject},
            None,
        )
        .await
        .expect("Document not found")
        .unwrap();

    let article: Article = bson::from_bson(bson::Bson::Document(article_document)).unwrap();
    article
}

pub async fn all_articles(db: Database) -> GqlResult<Vec<Article>> {
    let coll = db.collection("articles");

    let mut articles: Vec<Article> = vec![];

    // Query all documents in the collection.
    let mut cursor = coll.find(None, None).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = bson::from_bson(bson::Bson::Document(document)).unwrap();
                articles.push(article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    if articles.len() > 0 {
        Ok(articles)
    } else {
        Err(Error::new("7-all-articles").extend_with(|_, e| e.set("details", "No records")))
    }
}

pub async fn all_articles_by_user(db: Database, user_id: ObjectId) -> GqlResult<Vec<Article>> {
    let coll = db.collection("articles");

    let mut articles: Vec<Article> = vec![];

    // Query all documents in the collection.
    let mut cursor = coll.find(bson::doc! {"user_id": user_id}, None).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = bson::from_bson(bson::Bson::Document(document)).unwrap();
                articles.push(article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(articles)
}
