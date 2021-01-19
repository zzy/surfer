use futures::stream::StreamExt;
use mongodb::Database;
use bson::oid::ObjectId;
use async_graphql::{Error, ErrorExtensions};

use crate::util::constant::GqlResult;
use crate::projects::models::{Project, NewProject};

pub async fn add_project(db: Database, new_project: NewProject) -> Project {
    let coll = db.collection("projects");

    let exist_document = coll
        .find_one(
            bson::doc! {"user_id": &new_project.user_id,  "subject": &new_project.subject},
            None,
        )
        .await
        .unwrap();
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let new_project_bson = bson::to_bson(&new_project).unwrap();

        if let bson::Bson::Document(document) = new_project_bson {
            // Insert into a MongoDB collection
            coll.insert_one(document, None)
                .await
                .expect("Failed to insert into a MongoDB collection!");
        } else {
            println!("Error converting the BSON object into a MongoDB document");
        };
    }

    let project_document = coll
        .find_one(
            bson::doc! {"user_id": &new_project.user_id,  "subject": &new_project.subject},
            None,
        )
        .await
        .expect("Document not found")
        .unwrap();

    let project: Project = bson::from_bson(bson::Bson::Document(project_document)).unwrap();
    project
}

pub async fn all_projects(db: Database) -> GqlResult<Vec<Project>> {
    let coll = db.collection("projects");

    let mut projects: Vec<Project> = vec![];

    // Query all documents in the collection.
    let mut cursor = coll.find(None, None).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let project = bson::from_bson(bson::Bson::Document(document)).unwrap();
                projects.push(project);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    if projects.len() > 0 {
        Ok(projects)
    } else {
        Err(Error::new("7-all-projects").extend_with(|_, e| e.set("details", "No records")))
    }
}

pub async fn all_projects_by_user(db: Database, user_id: ObjectId) -> GqlResult<Vec<Project>> {
    let coll = db.collection("projects");

    let mut projects: Vec<Project> = vec![];

    // Query all documents in the collection.
    let mut cursor = coll.find(bson::doc! {"user_id": user_id}, None).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let project = bson::from_bson(bson::Bson::Document(document)).unwrap();
                projects.push(project);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(projects)
}
