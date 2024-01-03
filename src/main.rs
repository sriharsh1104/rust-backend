use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use mongodb::{Client, Database};
use std::env;

mod api;
mod model;
mod patent_data;
mod search_data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // Connect to MongoDB
    dotenv().ok();

    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let db = client.database("organizer_backend");

    // Print a message to the console
    println!("Connected to MongoDB");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(api::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
