use futures::stream::StreamExt;
use mongodb::{
    Database,
    options::FindOptions,
    bson::{
        oid::ObjectId, DateTime, Document, doc, to_document, from_document,
    },
};

use crate::util::{constant::GqlResult, common::slugify};

use super::models::{Category, CategoryUser, CategoryNew, CategoryUserNew};

// Create new category
pub async fn category_new(
    db: Database,
    mut category_new: CategoryNew,
) -> GqlResult<Category> {
    let coll = db.collection::<Document>("categories");

    let exist_document = coll
        .find_one(doc! {"name": &category_new.name.to_lowercase()}, None)
        .await?;
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let slug = slugify(&category_new.name).await;
        let uri = format!("/category/{}", &slug);

        category_new.slug = slug;
        category_new.uri = uri;

        let mut category_new_document = to_document(&category_new)?;
        let now = DateTime::now();
        category_new_document.insert("created_at", now);
        category_new_document.insert("updated_at", now);

        // Insert into a MongoDB collection
        coll.insert_one(category_new_document, None)
            .await
            .expect("Failed to insert into a MongoDB collection!");
    }

    let category_document = coll
        .find_one(doc! {"name": &category_new.name}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let category: Category = from_document(category_document)?;
    Ok(category)
}

// Create new category_user
pub async fn category_user_new(
    db: Database,
    category_user_new: CategoryUserNew,
) -> GqlResult<CategoryUser> {
    let coll = db.collection::<Document>("categories_users");

    let exist_document = coll
        .find_one(doc! {"user_id": &category_user_new.user_id, "category_id": &category_user_new.category_id}, None)
        .await
        .unwrap();
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let category_user_new_document = to_document(&category_user_new)?;
        // Insert into a MongoDB collection
        coll.insert_one(category_user_new_document, None)
            .await
            .expect("Failed to insert into a MongoDB collection!");
    }

    let category_user_document = coll
        .find_one(doc! {"user_id": &category_user_new.user_id, "category_id": &category_user_new.category_id}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let category_user: CategoryUser = from_document(category_user_document)?;
    Ok(category_user)
}

// get all categories
pub async fn categories(db: Database) -> GqlResult<Vec<Category>> {
    let coll = db.collection::<Document>("categories");

    // Query all documents in the collection.
    let find_options = FindOptions::builder().sort(doc! {"quotes": -1}).build();
    let mut cursor = coll.find(None, find_options).await.unwrap();

    // Iterate over the results of the cursor.
    let mut categories: Vec<Category> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let category = from_document(document)?;
                categories.push(category);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(categories)
}

// get all categories by user_id
pub async fn categories_by_user_id(
    db: Database,
    user_id: ObjectId,
) -> GqlResult<Vec<Category>> {
    let categories_users =
        self::categories_users_by_user_id(db.clone(), user_id).await;

    let mut category_ids: Vec<ObjectId> = vec![];
    for category_user in categories_users {
        category_ids.push(category_user.category_id);
    }

    let coll_categories = db.collection::<Document>("categories");
    let mut cursor_categories =
        coll_categories.find(doc! {"_id": {"$in": category_ids}}, None).await?;

    let mut categories: Vec<Category> = vec![];
    while let Some(result) = cursor_categories.next().await {
        match result {
            Ok(document) => {
                let category: Category = from_document(document)?;
                categories.push(category);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(categories)
}

// get all categories by username
pub async fn categories_by_username(
    db: Database,
    username: &str,
) -> GqlResult<Vec<Category>> {
    let user =
        crate::users::services::user_by_username(db.clone(), username).await?;
    self::categories_by_user_id(db, user._id).await
}

// get category by its slug
pub async fn category_by_id(db: Database, id: ObjectId) -> GqlResult<Category> {
    let coll = db.collection::<Document>("categories");

    let category_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let category: Category = from_document(category_document)?;
    Ok(category)
}

// get category by its slug
pub async fn category_by_slug(db: Database, slug: &str) -> GqlResult<Category> {
    let coll = db.collection::<Document>("categories");

    let category_document = coll
        .find_one(doc! {"slug": slug.to_lowercase()}, None)
        .await
        .expect("Document not found")
        .unwrap();

    let category: Category = from_document(category_document)?;
    Ok(category)
}

// get all CategoryUser list by user_id
async fn categories_users_by_user_id(
    db: Database,
    user_id: ObjectId,
) -> Vec<CategoryUser> {
    let coll_categories_users = db.collection::<Document>("categories_users");
    let mut cursor_categories_users = coll_categories_users
        .find(doc! {"user_id": user_id}, None)
        .await
        .unwrap();

    let mut categories_users: Vec<CategoryUser> = vec![];
    // Iterate over the results of the cursor.
    while let Some(result) = cursor_categories_users.next().await {
        match result {
            Ok(document) => {
                let category_user: CategoryUser =
                    from_document(document).unwrap();
                categories_users.push(category_user);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    categories_users
}
