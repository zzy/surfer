use futures::stream::StreamExt;
use async_graphql::{Error, ErrorExtensions};
use mongodb::{Database, options::FindOptions};
use bson::{doc, oid::ObjectId};
use unicode_segmentation::UnicodeSegmentation;
use pinyin::ToPinyin;

use crate::util::{constant::GqlResult, common::web_base_uri};
use crate::articles::models::{Article, ArticleNew};
use crate::users;

pub async fn article_new(
    db: Database,
    mut article_new: ArticleNew,
) -> GqlResult<Article> {
    let coll = db.collection("articles");

    let exist_document = coll
        .find_one(
            doc! {"user_id": &article_new.user_id,  "subject": &article_new.subject},
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

        let user =
            users::services::user_by_id(db.clone(), &article_new.user_id)
                .await?;
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
            doc! {"user_id": &article_new.user_id,  "subject": &article_new.subject},
            None,
        )
        .await
        .expect("Document not found")
        .unwrap();

    let article: Article =
        bson::from_bson(bson::Bson::Document(article_document)).unwrap();
    Ok(article)
}

pub async fn articles_list(
    db: Database,
    published: &i32,
) -> GqlResult<Vec<Article>> {
    let mut find_doc = doc! {};
    if published > &0 {
        find_doc.insert("published", true);
    } else if published < &0 {
        find_doc.insert("published", false);
    }
    let coll = db.collection("articles");
    let mut cursor = coll.find(find_doc, None).await.unwrap();

    let mut articles: Vec<Article> = vec![];
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
    published: &i32,
) -> GqlResult<Vec<Article>> {
    let mut find_doc = doc! {"user_id": user_id};
    if published > &0 {
        find_doc.insert("published", true);
    } else if published < &0 {
        find_doc.insert("published", false);
    }
    let coll = db.collection("articles");
    let mut cursor = coll.find(find_doc, None).await?;

    let mut articles: Vec<Article> = vec![];
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
    published: &i32,
) -> GqlResult<Vec<Article>> {
    let user = users::services::user_by_username(db.clone(), username).await?;
    self::articles_by_user_id(db, &user._id, published).await
}

pub async fn articles_by_category_id(
    db: Database,
    category_id: &ObjectId,
    published: &i32,
) -> GqlResult<Vec<Article>> {
    let mut find_doc = doc! {"category_id": category_id};
    if published > &0 {
        find_doc.insert("published", true);
    } else if published < &0 {
        find_doc.insert("published", false);
    }
    let coll = db.collection("articles");
    let mut cursor = coll.find(find_doc, None).await?;

    let mut articles: Vec<Article> = vec![];
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
        .find_one(doc! {"username": username, "slug": slug}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let article: Article =
        bson::from_bson(bson::Bson::Document(article_document)).unwrap();
    Ok(article)
}

pub async fn articles_in_position(
    db: Database,
    username: &str,
    position: &str,
    limit: i64,
) -> GqlResult<Vec<Article>> {
    let coll = db.collection("articles");

    let mut find_doc = doc! {"published": true};
    if "".ne(username.trim()) {
        let user =
            users::services::user_by_username(db.clone(), username).await?;
        find_doc.insert("user_id", &user._id);
    }
    if "top".eq(position.trim()) {
        find_doc.insert("top", true);
    }
    if "recommended".eq(position.trim()) {
        find_doc.insert("recommended", true);
    }

    let find_options = FindOptions::builder().limit(limit).build();
    let mut cursor = coll.find(find_doc, find_options).await?;

    let mut articles: Vec<Article> = vec![];
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
