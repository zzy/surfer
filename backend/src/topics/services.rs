use futures::stream::StreamExt;
use mongodb::{Database, options::FindOptions};
use bson::{doc, oid::ObjectId};
use async_graphql::{Error, ErrorExtensions};
use unicode_segmentation::UnicodeSegmentation;
use pinyin::ToPinyin;

use crate::util::constant::GqlResult;

use super::models::{Topic, TopicNew, TopicArticle, TopicArticleNew};

// Create new topic
pub async fn topic_new(
    db: Database,
    mut topic_new: TopicNew,
) -> GqlResult<Topic> {
    let coll = db.collection("topics");

    let exist_document =
        coll.find_one(doc! {"name": &topic_new.name}, None).await?;
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let name_low = topic_new.name.to_lowercase();
        let mut name_seg: Vec<&str> = name_low.unicode_words().collect();
        for n in 0..name_seg.len() {
            let seg = name_seg[n];
            if !seg.is_ascii() {
                let seg_py =
                    seg.chars().next().unwrap().to_pinyin().unwrap().plain();
                name_seg[n] = seg_py;
            }
        }
        let slug = name_seg.join("-");
        let uri = format!("/topics/{}", &slug);

        topic_new.slug = slug;
        topic_new.uri = uri;

        let topic_new_bson = bson::to_bson(&topic_new).unwrap();

        if let bson::Bson::Document(document) = topic_new_bson {
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

    let topic_document = coll
        .find_one(doc! {"name": &topic_new.name}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let topic: Topic =
        bson::from_bson(bson::Bson::Document(topic_document)).unwrap();
    Ok(topic)
}

// Create new topic_article
pub async fn topic_article_new(
    db: Database,
    topic_article_new: TopicArticleNew,
) -> GqlResult<TopicArticle> {
    let coll = db.collection("topics_articles");

    let exist_document = coll
        .find_one(doc! {"topic_id": &topic_article_new.topic_id, "article_id": &topic_article_new.article_id}, None)
        .await
        .unwrap();
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let topic_article_new_bson = bson::to_bson(&topic_article_new).unwrap();

        if let bson::Bson::Document(document) = topic_article_new_bson {
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

    let topic_article_document = coll
        .find_one(doc! {"topic_id": &topic_article_new.topic_id, "article_id": &topic_article_new.article_id}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let topic_article: TopicArticle =
        bson::from_bson(bson::Bson::Document(topic_article_document)).unwrap();
    Ok(topic_article)
}

// get all topics
pub async fn topics(db: Database) -> GqlResult<Vec<Topic>> {
    let coll = db.collection("topics");

    let find_options = FindOptions::builder().sort(doc! {"quotes": -1}).build();
    let mut cursor = coll.find(None, find_options).await.unwrap();

    let mut topics: Vec<Topic> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let topic =
                    bson::from_bson(bson::Bson::Document(document)).unwrap();
                topics.push(topic);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    if topics.len() > 0 {
        Ok(topics)
    } else {
        Err(Error::new("all-topics")
            .extend_with(|_, e| e.set("details", "No records")))
    }
}

// get topic info by id
pub async fn topic_by_id(db: Database, id: &ObjectId) -> GqlResult<Topic> {
    let coll = db.collection("topics");

    let topic_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let topic: Topic =
        bson::from_bson(bson::Bson::Document(topic_document)).unwrap();
    Ok(topic)
}

// get topics by article_id
pub async fn topics_by_article_id(
    db: Database,
    article_id: &ObjectId,
) -> GqlResult<Vec<Topic>> {
    let topics_articles =
        self::topics_articles_by_article_id(db.clone(), article_id).await;

    let mut topic_ids = vec![];
    for topic_article in topics_articles {
        topic_ids.push(topic_article.topic_id);
    }

    let coll = db.collection("topics");
    let mut cursor = coll.find(doc! {"_id": {"$in": topic_ids}}, None).await?;

    let mut topics: Vec<Topic> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let topic = bson::from_bson(bson::Bson::Document(document))?;
                topics.push(topic);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(topics)
}

// get all TopicArticle list by article_id
async fn topics_articles_by_article_id(
    db: Database,
    article_id: &ObjectId,
) -> Vec<TopicArticle> {
    let coll_topics_articles = db.collection("topics_articles");
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
                    bson::from_bson(bson::Bson::Document(document)).unwrap();
                topics_articles.push(topic_article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    topics_articles
}

// get topics by user_id
pub async fn topics_by_user_id(
    db: Database,
    user_id: &ObjectId,
) -> GqlResult<Vec<Topic>> {
    let topics_articles =
        self::topics_articles_by_user_id(db.clone(), user_id).await;

    let mut topic_ids_dup = vec![];
    for topic_article in topics_articles {
        topic_ids_dup.push(topic_article.topic_id);
    }

    let mut topic_ids = topic_ids_dup.clone();
    topic_ids.dedup();

    let mut topics: Vec<Topic> = vec![];
    for topic_id in topic_ids {
        let mut topic = self::topic_by_id(db.clone(), &topic_id).await?;
        topic.quotes =
            topic_ids_dup.iter().filter(|&id| *id == topic_id).count() as i64;

        topics.push(topic);
    }

    Ok(topics)
}

// get all TopicArticle list by user_id
async fn topics_articles_by_user_id(
    db: Database,
    user_id: &ObjectId,
) -> Vec<TopicArticle> {
    let coll_topics_articles = db.collection("topics_articles");
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
                    bson::from_bson(bson::Bson::Document(document)).unwrap();
                topics_articles.push(topic_article);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    topics_articles
}
