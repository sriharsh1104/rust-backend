// search_data.rs

use actix_web::{web, HttpRequest, HttpResponse, Result};
use futures_util::TryStreamExt; // Import TryStreamExt trait
use mongodb::bson::doc;
use mongodb::{Collection, Database};
use std::error::Error;

pub async fn search_patents(db: web::Data<Database>, req: HttpRequest) -> Result<HttpResponse> {
    // Extract the search query from the request parameters
    let query = req.query_string();
    println!("Received search request with query: {}", query);
    println!("Received search request");
    // Perform a simple search using the query in the patent_data collection
    let collection: Collection = db.collection("patent_data");
    let filter = doc! { "paragraph": { "$regex": query, "$options": "i" } };
    let cursor = collection.find(filter, None).await.unwrap();
    let collection_count = collection.count_documents(doc! {}, None).await.unwrap();
    println!(
        "Number of documents in the collection: {}",
        collection_count
    );

    // Collect the results into a Vec
    let results: Result<Vec<_>, Box<dyn Error>> = cursor
        .try_fold(vec![], |mut acc, doc| async {
            acc.push(doc);
            Ok(acc)
        })
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error>);

    // Check if there was an error during the collection
    match results {
        Ok(results) => {
            // Return the results as JSON
            Ok(HttpResponse::Ok().json(results))
        }
        Err(e) => {
            // Handle the error, you might want to log it or return an error response
            eprintln!("Error fetching results from MongoDB: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
