use futures::stream::StreamExt;
use mongodb::Database;
use bson::oid::ObjectId;
use unicode_segmentation::UnicodeSegmentation;
use pinyin::ToPinyin;

use crate::util::{constant::GqlResult, common::web_base_uri};
use crate::topics::models::{Topic, TopicNew};

// Create new topic
pub async fn topic_new(
    db: Database,
    mut topic_new: TopicNew,
) -> GqlResult<Topic> {
    let coll = db.collection("topics");

    let exist_document = coll
        .find_one(
            bson::doc! {"name": &topic_new.name, "article_id": &topic_new.article_id},
            None,
        )
        .await
        .unwrap();
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
        let uri = format!("{}/topics/{}", web_base_uri().await, &slug);

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
        .find_one(bson::doc! {"name": &topic_new.name, "article_id": &topic_new.article_id}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let topic: Topic =
        bson::from_bson(bson::Bson::Document(topic_document)).unwrap();
    Ok(topic)
}

// search topics by article_id
pub async fn topics_by_article_id(
    db: Database,
    article_id: &ObjectId,
) -> GqlResult<Vec<Topic>> {
    let coll = db.collection("topics");

    let mut topics: Vec<Topic> = vec![];

    // Query all documents in the collection.
    let mut cursor =
        coll.find(bson::doc! {"article_id": article_id}, None).await?;

    // Iterate over the results of the cursor.
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
