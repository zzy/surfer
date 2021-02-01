use futures::stream::StreamExt;
use async_graphql::{Error, ErrorExtensions};
use mongodb::Database;
use bson::oid::ObjectId;
use unicode_segmentation::UnicodeSegmentation;
use pinyin::ToPinyin;

use crate::util::{constant::GqlResult, common::web_base_uri};
use crate::categories::models::{Category, CategoryNew};

// Create new category
pub async fn category_new(
    db: Database,
    mut category_new: CategoryNew,
) -> GqlResult<Category> {
    let coll = db.collection("categories");

    let exist_document = coll
        .find_one(bson::doc! {"name": &category_new.name}, None)
        .await
        .unwrap();
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let name_low = category_new.name.to_lowercase();
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
        let uri = format!("{}/categories/{}", web_base_uri().await, &slug);

        category_new.slug = slug;
        category_new.uri = uri;

        let category_new_bson = bson::to_bson(&category_new).unwrap();

        if let bson::Bson::Document(document) = category_new_bson {
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

    let category_document = coll
        .find_one(bson::doc! {"name": &category_new.name}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let category: Category =
        bson::from_bson(bson::Bson::Document(category_document)).unwrap();
    Ok(category)
}

// get all categories
pub async fn categories_list(db: Database) -> GqlResult<Vec<Category>> {
    let coll = db.collection("categories");

    let mut categories: Vec<Category> = vec![];

    // Query all documents in the collection.
    let mut cursor = coll.find(None, None).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let category =
                    bson::from_bson(bson::Bson::Document(document)).unwrap();
                categories.push(category);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    if categories.len() > 0 {
        Ok(categories)
    } else {
        Err(Error::new("8-all-categories")
            .extend_with(|_, e| e.set("details", "No records")))
    }
}

// search category by its slug
pub async fn category_by_id(
    db: Database,
    id: &ObjectId,
) -> GqlResult<Category> {
    let coll = db.collection("categories");

    let category_document = coll
        .find_one(bson::doc! {"_id": id}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let category: Category =
        bson::from_bson(bson::Bson::Document(category_document)).unwrap();
    Ok(category)
}

// search category by its slug
pub async fn category_by_slug(db: Database, slug: &str) -> GqlResult<Category> {
    let coll = db.collection("categories");

    let category_document = coll
        .find_one(bson::doc! {"slug": slug}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let category: Category =
        bson::from_bson(bson::Bson::Document(category_document)).unwrap();
    Ok(category)
}
