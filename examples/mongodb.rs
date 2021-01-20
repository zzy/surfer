use mongodb::{Client, options::ClientOptions, bson::doc};
use futures::stream::StreamExt;
use mongodb::{bson::Bson, options::FindOptions};

#[async_std::main]
async fn main() {
    let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await.expect("msg");
    client_options.app_name = Some("My App".to_string());

    let client = Client::with_options(client_options).expect("msg");

    for db_name in client.list_database_names(None, None).await {
        println!("{:?}", db_name);
    }

    let db = client.database("budshome");

    // Get a handle to a collection in the database.
    let collection = db.collection("books");

    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    // Insert some documents into the "budshome.books" collection.
    collection.insert_many(docs, None).await.expect("msg");

    let filter = doc! { "author": "George Orwell" };
    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = collection.find(filter, find_options).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                if let Some(title) = document.get("title").and_then(Bson::as_str) {
                    println!("title: {}", title);
                } else {
                    println!("no title found");
                }
            }
            Err(_) => return (),
        }
    }
}
