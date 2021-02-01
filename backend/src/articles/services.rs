use futures::stream::StreamExt;
use async_graphql::{Error, ErrorExtensions};
use mongodb::Database;
use bson::oid::ObjectId;
use unicode_segmentation::UnicodeSegmentation;
use pinyin::ToPinyin;

use crate::util::{constant::GqlResult, common::web_base_uri};
use crate::articles::models::{Article, ArticleNew};
use crate::users::services::user_by_id;

pub async fn article_new(
    db: Database,
    mut article_new: ArticleNew,
) -> GqlResult<Article> {
    let coll = db.collection("articles");

    let exist_document = coll
        .find_one(
            bson::doc! {"user_id": &article_new.user_id,  "subject": &article_new.subject},
            None,
        )
        .await
        .unwrap();
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let subject_low = article_new.subject.to_lowercase();
        let mut subject_seg: Vec<&str> = subject_low.unicode_words().collect();
        for n in 0..subject_seg.len() {
            let seg = subject_seg[n];
            if !seg.is_ascii() {
                let seg_py =
                    seg.chars().next().unwrap().to_pinyin().unwrap().plain();
                subject_seg[n] = seg_py;
            }
        }
        let slug = subject_seg.join("-");

        let user = user_by_id(db.clone(), &article_new.user_id).await?;
        let uri =
            format!("{}/{}/{}", web_base_uri().await, &user.username, &slug);

        article_new.slug = slug;
        article_new.uri = uri;
        article_new.published = false;
        article_new.top = false;
        article_new.recommended = false;

        let article_new_bson = bson::to_bson(&article_new).unwrap();

        if let bson::Bson::Document(document) = article_new_bson {
            // Insert into a MongoDB collection
            coll.insert_one(document, None)
                .await
                .expect("Failed to insert into a MongoDB collection!");
        } else {
            println!(
                "Error converting the BSON object into a MongoDB document"
            );
        };
    }

    let article_document = coll
        .find_one(
            bson::doc! {"user_id": &article_new.user_id,  "subject": &article_new.subject},
            None,
        )
        .await
        .expect("Document not found")
        .unwrap();

    let article: Article =
        bson::from_bson(bson::Bson::Document(article_document)).unwrap();
    Ok(article)
}

pub async fn articles_list(db: Database) -> GqlResult<Vec<Article>> {
    let coll = db.collection("articles");

    let mut articles: Vec<Article> = vec![];

    // Query all documents in the collection.
    let mut cursor = coll.find(None, None).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article =
                    bson::from_bson(bson::Bson::Document(document)).unwrap();
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
        Err(Error::new("7-all-articles")
            .extend_with(|_, e| e.set("details", "No records")))
    }
}

pub async fn articles_by_user_id(
    db: Database,
    user_id: &ObjectId,
) -> GqlResult<Vec<Article>> {
    let coll = db.collection("articles");

    let mut articles: Vec<Article> = vec![];

    // Query all documents in the collection.
    let mut cursor = coll.find(bson::doc! {"user_id": user_id}, None).await?;

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = bson::from_bson(bson::Bson::Document(document))?;
                articles.push(article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(articles)
}

pub async fn articles_by_username(
    db: Database,
    username: &str,
) -> GqlResult<Vec<Article>> {
    let coll = db.collection("articles");

    let mut articles: Vec<Article> = vec![];

    // Query all documents in the collection.
    let mut cursor = coll.find(bson::doc! {"username": username}, None).await?;

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = bson::from_bson(bson::Bson::Document(document))?;
                articles.push(article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(articles)
}

pub async fn articles_by_category_id(
    db: Database,
    category_id: &ObjectId,
) -> GqlResult<Vec<Article>> {
    let coll = db.collection("articles");

    let mut articles: Vec<Article> = vec![];

    // Query all documents in the collection.
    let mut cursor =
        coll.find(bson::doc! {"category_id": category_id}, None).await?;

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = bson::from_bson(bson::Bson::Document(document))?;
                articles.push(article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(articles)
}

pub async fn article_by_slug(
    db: Database,
    username: &str,
    slug: &str,
) -> GqlResult<Article> {
    let coll = db.collection("articles");

    // Query all documents in the collection.
    let article_document = coll
        .find_one(bson::doc! {"username": username, "slug": slug}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let article: Article =
        bson::from_bson(bson::Bson::Document(article_document)).unwrap();
    Ok(article)
}
