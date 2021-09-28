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

use super::models::{Topic, TopicNew, TopicArticle, TopicArticleNew};

// Create new topics
pub async fn topics_new(
    db: Database,
    topic_names: &str,
) -> GqlResult<Vec<Topic>> {
    let mut topics: Vec<Topic> = vec![];

    let names = topic_names.split(",");
    for name in names {
        let topic_new = TopicNew {
            name: name.trim().to_string(),
            quotes: 1,
            slug: "".to_string(),
            uri: "".to_string(),
        };

        let topic = self::topic_new(db.clone(), topic_new).await?;
        topics.push(topic);
    }

    Ok(topics)
}

// Create new topic
pub async fn topic_new(
    db: Database,
    mut topic_new: TopicNew,
) -> GqlResult<Topic> {
    let coll = db.collection::<Document>("topics");

    topic_new.name = topic_new.name.to_lowercase();

    let exist_document =
        coll.find_one(doc! {"name": &topic_new.name}, None).await?;
    if let Some(document) = exist_document {
        let topic: Topic = from_document(document)?;
        coll.update_one(
            doc! {"_id": &topic._id},
            doc! {"$set": {"quotes": &topic.quotes + 1}},
            None,
        )
        .await
        .expect("Failed to update a MongoDB collection!");
    } else {
        let slug = slugify(&topic_new.name).await;
        let uri = format!("/topic/{}", &slug);

        topic_new.slug = slug;
        topic_new.uri = uri;

        let mut topic_new_document = to_document(&topic_new)?;
        let now = DateTime::now();
        topic_new_document.insert("created_at", now);
        topic_new_document.insert("updated_at", now);

        // Insert into a MongoDB collection
        coll.insert_one(topic_new_document, None)
            .await
            .expect("Failed to insert into a MongoDB collection!");
    }

    let topic_document = coll
        .find_one(doc! {"name": &topic_new.name}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let topic: Topic = from_document(topic_document)?;
    Ok(topic)
}

// Create new topic_article
pub async fn topic_article_new(
    db: Database,
    topic_article_new: TopicArticleNew,
) -> GqlResult<TopicArticle> {
    let coll = db.collection::<Document>("topics_articles");

    let exist_document = coll
        .find_one(doc! {"topic_id": &topic_article_new.topic_id, "article_id": &topic_article_new.article_id}, None)
        .await
        .unwrap();
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let topic_article_new_document = to_document(&topic_article_new)?;
        // Insert into a MongoDB collection
        coll.insert_one(topic_article_new_document, None)
            .await
            .expect("Failed to insert into a MongoDB collection!");
    }

    let topic_article_document = coll
        .find_one(doc! {"topic_id": &topic_article_new.topic_id, "article_id": &topic_article_new.article_id}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let topic_article: TopicArticle = from_document(topic_article_document)?;
    Ok(topic_article)
}

// get all topics
pub async fn topics(db: Database) -> GqlResult<Vec<Topic>> {
    let coll = db.collection::<Document>("topics");

    let find_options = FindOptions::builder().sort(doc! {"quotes": -1}).build();
    let mut cursor = coll.find(None, find_options).await.unwrap();

    let mut topics: Vec<Topic> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let topic = from_document(document)?;
                topics.push(topic);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(topics)
}

// get topic info by id
pub async fn topic_by_id(db: Database, id: ObjectId) -> GqlResult<Topic> {
    let coll = db.collection::<Document>("topics");

    let topic_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let topic: Topic = from_document(topic_document)?;
    Ok(topic)
}

// get topic info by slug
pub async fn topic_by_slug(db: Database, slug: &str) -> GqlResult<Topic> {
    let coll = db.collection::<Document>("topics");

    let topic_document = coll
        .find_one(doc! {"slug": slug.to_lowercase()}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let topic: Topic = from_document(topic_document)?;
    Ok(topic)
}

// get topics by article_id
pub async fn topics_by_article_id(
    db: Database,
    article_id: ObjectId,
) -> GqlResult<Vec<Topic>> {
    let topics_articles =
        self::topics_articles_by_article_id(db.clone(), article_id).await;

    let mut topic_ids = vec![];
    for topic_article in topics_articles {
        topic_ids.push(topic_article.topic_id);
    }

    let coll = db.collection::<Document>("topics");
    let mut cursor = coll.find(doc! {"_id": {"$in": topic_ids}}, None).await?;

    let mut topics: Vec<Topic> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let topic = from_document(document)?;
                topics.push(topic);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }
    topics.sort_by(|a, b| b.quotes.cmp(&a.quotes));

    Ok(topics)
}

// get topics by user_id
pub async fn topics_by_user_id(
    db: Database,
    user_id: ObjectId,
) -> GqlResult<Vec<Topic>> {
    let topics_articles =
        self::topics_articles_by_user_id(db.clone(), user_id).await;

    let mut topic_ids_dup = vec![];
    for topic_article in topics_articles {
        topic_ids_dup.push(topic_article.topic_id);
    }

    let mut topic_ids = topic_ids_dup.clone();
    topic_ids.sort();
    topic_ids.dedup();

    let mut topics: Vec<Topic> = vec![];
    let coll = db.collection::<Document>("topics");
    let mut cursor = coll.find(doc! {"_id": {"$in": topic_ids}}, None).await?;

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let mut topic: Topic = from_document(document)?;
                topic.quotes =
                    topic_ids_dup.iter().filter(|&id| *id == topic._id).count()
                        as i64;
                topics.push(topic);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }
    topics.sort_by(|a, b| b.quotes.cmp(&a.quotes));

    Ok(topics)
}

// get topics by username
pub async fn topics_by_username(
    db: Database,
    username: &str,
) -> GqlResult<Vec<Topic>> {
    let user = users::services::user_by_username(db.clone(), username).await?;
    self::topics_by_user_id(db, user._id).await
}

// get topics by category_id
pub async fn topics_by_category_id(
    db: Database,
    category_id: ObjectId,
    published: i32,
) -> GqlResult<Vec<Topic>> {
    let articles = crate::articles::services::articles_by_category_id(
        db.clone(),
        category_id,
        published,
    )
    .await?;

    let mut article_ids = vec![];
    for article in articles {
        article_ids.push(article._id);
    }

    let mut topic_ids_dup = vec![];
    let coll_topics_articles = db.collection::<Document>("topics_articles");
    let mut cursor_topics_articles = coll_topics_articles
        .find(doc! {"article_id": {"$in": article_ids}}, None)
        .await?;

    while let Some(result) = cursor_topics_articles.next().await {
        match result {
            Ok(document) => {
                let topic_article: TopicArticle = from_document(document)?;
                topic_ids_dup.push(topic_article.topic_id);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    let mut topic_ids = topic_ids_dup.clone();
    topic_ids.sort();
    topic_ids.dedup();

    let mut topics: Vec<Topic> = vec![];
    let coll_topics = db.collection::<Document>("topics");
    let mut cursor_topics =
        coll_topics.find(doc! {"_id": {"$in": topic_ids}}, None).await?;

    while let Some(result) = cursor_topics.next().await {
        match result {
            Ok(document) => {
                let mut topic: Topic = from_document(document)?;
                topic.quotes =
                    topic_ids_dup.iter().filter(|&id| *id == topic._id).count()
                        as i64;
                topics.push(topic);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }
    topics.sort_by(|a, b| b.quotes.cmp(&a.quotes));

    Ok(topics)
}

// get all TopicArticle list by user_id
async fn topics_articles_by_user_id(
    db: Database,
    user_id: ObjectId,
) -> Vec<TopicArticle> {
    let coll_topics_articles = db.collection::<Document>("topics_articles");
    let mut cursor_topics_articles = coll_topics_articles
        .find(doc! {"user_id": user_id}, None)
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

// get all TopicArticle list by article_id
async fn topics_articles_by_article_id(
    db: Database,
    article_id: ObjectId,
) -> Vec<TopicArticle> {
    let coll_topics_articles = db.collection::<Document>("topics_articles");
    let mut cursor_topics_articles = coll_topics_articles
        .find(doc! {"article_id": article_id}, None)
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
