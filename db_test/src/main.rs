#![allow(unused)]

mod test;

// This trait is required to use `try_next()` on the cursor
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, Client};
use serde::{Deserialize, Serialize};
use test::custom_print;

const MONGO_URI: &str = "";

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

#[tokio::main]
async fn main() {
    let client = Client::with_uri_str(MONGO_URI).await.unwrap();

    // Get a handle to a collection in the database.
    let collection = client.database("testing").collection::<Book>("books");

    // custom_print("Starting to insert documents into the database...");

    // let books = vec![
    //     Book {
    //         title: "The Grapes of Wrath".to_string(),
    //         author: "John Steinbeck".to_string(),
    //     },
    //     Book {
    //         title: "To Kill a Mockingbird".to_string(),
    //         author: "Harper Lee".to_string(),
    //     },
    // ];

    // collection.insert_many(books, None).await.unwrap();

    // custom_print("Finished inserting documents into the database!");

    // custom_print("Starting to query the database...");

    // Query the books in the collection with a filter and an option.
    // let filter = doc! { "author": "John Steinbeck" };
    // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    // let mut cursor = collection.find(filter, find_options).await.unwrap();

    // // Iterate over the results of the cursor.
    // while let Some(book) = cursor.try_next().await.unwrap() {
    //     println!("Book data: {:?}", book);
    // }

    // custom_print("Finished querying the database!");

    for i in 0..5 {
        // Perform operations that work with directly our model.
        collection.insert_one(Book { author: i.to_string(), title: i.to_string() }, None).await;
        custom_print("Inserted a document into the database!");
    }

    custom_print("Finished running main.rs!")
}
