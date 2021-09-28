use futures::stream::StreamExt;
use mongodb::{
    Database,
    options::FindOptions,
    bson::{
        oid::ObjectId, DateTime, Document, doc, to_document, from_document,
    },
};

use crate::util::{constant::GqlResult, common::slugify};
use crate::users;

use crate::topics::models::TopicArticle;
use super::models::{Article, ArticleNew};

pub async fn article_new(
    db: Database,
    mut article_new: ArticleNew,
) -> GqlResult<Article> {
    let coll = db.collection::<Document>("articles");

    let exist_document = coll
        .find_one(
            doc! {"user_id": article_new.user_id,  "subject": &article_new.subject},
            None,
        )
        .await
        .unwrap();
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let slug = slugify(&article_new.subject).await;

        let user = users::services::user_by_id(db.clone(), article_new.user_id)
            .await?;
        let uri = format!("/{}/{}", &user.username, &slug);

        article_new.slug = slug;
        article_new.uri = uri;
        article_new.published = true; // false;
        article_new.top = true; // false;
        article_new.recommended = true; // false;

        let mut article_new_document = to_document(&article_new)?;
        let now = DateTime::now();
        article_new_document.insert("created_at", now);
        article_new_document.insert("updated_at", now);

        // Insert into a MongoDB collection
        coll.insert_one(article_new_document, None)
            .await
            .expect("Failed to insert into a MongoDB collection!");
    }

    let article_document = coll
        .find_one(
            doc! {"user_id": article_new.user_id,  "subject": &article_new.subject},
            None,
        )
        .await
        .expect("Document not found")
        .unwrap();

    let article: Article = from_document(article_document)?;
    Ok(article)
}

pub async fn article_by_slug(
    db: Database,
    username: &str,
    slug: &str,
) -> GqlResult<Article> {
    let coll = db.collection::<Document>("articles");

    let user = users::services::user_by_username(db.clone(), username).await?;
    let article_document = coll
        .find_one(doc! {"user_id": user._id, "slug": slug.to_lowercase()}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let article: Article = from_document(article_document)?;
    Ok(article)
}

pub async fn articles(db: Database, published: i32) -> GqlResult<Vec<Article>> {
    let mut find_doc = doc! {};
    if published > 0 {
        find_doc.insert("published", true);
    } else if published < 0 {
        find_doc.insert("published", false);
    }
    let coll = db.collection::<Document>("articles");

    let find_options =
        FindOptions::builder().sort(doc! {"updated_at": -1}).build();
    let mut cursor = coll.find(find_doc, find_options).await.unwrap();

    let mut articles: Vec<Article> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = from_document(document)?;
                articles.push(article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(articles)
}

pub async fn articles_in_position(
    db: Database,
    username: &str,
    position: &str,
    limit: i64,
) -> GqlResult<Vec<Article>> {
    let mut find_doc = doc! {"published": true};
    if "".ne(username.trim()) && "-".ne(username.trim()) {
        let user =
            users::services::user_by_username(db.clone(), username).await?;
        find_doc.insert("user_id", user._id);
    }
    if "top".eq(position.trim()) {
        find_doc.insert("top", true);
    }
    if "recommended".eq(position.trim()) {
        find_doc.insert("recommended", true);
    }

    let coll = db.collection::<Document>("articles");

    let find_options = FindOptions::builder()
        .sort(doc! {"updated_at": -1})
        .limit(limit)
        .build();
    let mut cursor = coll.find(find_doc, find_options).await?;

    let mut articles: Vec<Article> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = from_document(document)?;
                articles.push(article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(articles)
}

pub async fn articles_by_user_id(
    db: Database,
    user_id: ObjectId,
    published: i32,
) -> GqlResult<Vec<Article>> {
    let mut find_doc = doc! {"user_id": user_id};
    if published > 0 {
        find_doc.insert("published", true);
    } else if published < 0 {
        find_doc.insert("published", false);
    }
    let find_options =
        FindOptions::builder().sort(doc! {"updated_at": -1}).build();

    let coll = db.collection::<Document>("articles");
    let mut cursor = coll.find(find_doc, find_options).await?;

    let mut articles: Vec<Article> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = from_document(document)?;
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
    published: i32,
) -> GqlResult<Vec<Article>> {
    let user = users::services::user_by_username(db.clone(), username).await?;
    self::articles_by_user_id(db, user._id, published).await
}

// Get all articles by category_id
pub async fn articles_by_category_id(
    db: Database,
    category_id: ObjectId,
    published: i32,
) -> GqlResult<Vec<Article>> {
    let mut find_doc = doc! {"category_id": category_id};
    if published > 0 {
        find_doc.insert("published", true);
    } else if published < 0 {
        find_doc.insert("published", false);
    }
    let find_options =
        FindOptions::builder().sort(doc! {"updated_at": -1}).build();

    let coll = db.collection::<Document>("articles");
    let mut cursor = coll.find(find_doc, find_options).await?;

    let mut articles: Vec<Article> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = from_document(document)?;
                articles.push(article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(articles)
}

// Get all articles by topic_id
pub async fn articles_by_topic_id(
    db: Database,
    topic_id: ObjectId,
    published: i32,
) -> GqlResult<Vec<Article>> {
    let topics_articles =
        self::topics_articles_by_topic_id(db.clone(), topic_id).await;

    let mut article_ids = vec![];
    for topic_article in topics_articles {
        article_ids.push(topic_article.article_id);
    }
    article_ids.sort();
    article_ids.dedup();

    let mut find_doc = doc! {"_id": {"$in": article_ids}};
    if published > 0 {
        find_doc.insert("published", true);
    } else if published < 0 {
        find_doc.insert("published", false);
    }
    let find_options =
        FindOptions::builder().sort(doc! {"updated_at": -1}).build();

    let coll = db.collection::<Document>("articles");
    let mut cursor = coll.find(find_doc, find_options).await?;

    let mut articles: Vec<Article> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let article = from_document(document)?;
                articles.push(article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(articles)
}

// get all TopicArticle list by topic_id
async fn topics_articles_by_topic_id(
    db: Database,
    topic_id: ObjectId,
) -> Vec<TopicArticle> {
    let coll_topics_articles = db.collection::<Document>("topics_articles");
    let mut cursor_topics_articles = coll_topics_articles
        .find(doc! {"topic_id": topic_id}, None)
        .await
        .unwrap();

    let mut topics_articles: Vec<TopicArticle> = vec![];
    // Iterate over the results of the cursor.
    while let Some(result) = cursor_topics_articles.next().await {
        match result {
            Ok(document) => {
                let topic_article: TopicArticle =
                    from_document(document).unwrap();
                topics_articles.push(topic_article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    topics_articles
}
