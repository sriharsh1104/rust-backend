    use actix_web::{web, HttpResponse, Responder};
    use mongodb::{Database, Collection};
    use serde::{Deserialize, Serialize};
    use mongodb::bson::{Document, doc, oid::ObjectId}; // Import ObjectId explicitly
    use crate::model::UserData;
    use crate::patent_data::PatentDataForUser;
    use mongodb::options::FindOneAndUpdateOptions;
    use mongodb::options::ReturnDocument;

    use jsonwebtoken::{encode,decode, Header, EncodingKey, Algorithm,Validation};
    use chrono::Utc;
    use jsonwebtoken::DecodingKey;
    use std::env;

    #[derive(Debug, Serialize, Deserialize)]
    struct ApiResponse {
        message: String,
        token: Option<String>,
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct LoginRequest {
        email: String,
        password: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct UserClaims {
        sub: String, // User ID
        exp: usize,  // Expiration time
        // ... other claims as needed
    }
  

    fn generate_jwt_token(user_id: &ObjectId) -> String {
        // let key = EncodingKey::from_secret(b"your_secret_key");
        let key = env::var("JWT_SECRET").expect("JWT_SECRET environment variable not set");
        let key = EncodingKey::from_secret(key.as_bytes());


        // Set expiration time, e.g., 1 hour from now
        let exp = Utc::now() + chrono::Duration::hours(1);

        let claims = UserClaims {
            sub: user_id.to_hex(),
            exp: exp.timestamp() as usize,
            // ... other claims as needed
        };

        encode(&Header::default(), &claims, &key).unwrap()
    }

    fn verify_jwt_token(token: &str) -> Result<UserClaims, jsonwebtoken::errors::Error> {
        let key = env::var("JWT_SECRET").expect("JWT_SECRET environment variable not set");
        let key = DecodingKey::from_secret(key.as_bytes());
        println!("JWT Token request");


        decode::<UserClaims>(token, &key, &Validation::default())
            .map(|data| data.claims)
    }



    async fn register(user_data: web::Json<UserData>, db: web::Data<Database>) -> impl Responder {
        // Validation logic
        println!("Received registration request");
        if user_data.password != user_data.confirm_password {
            return HttpResponse::BadRequest().json(ApiResponse { message: "Password and confirm password do not match".to_string(),token:None });
        }


        let collection: Collection = db.collection("users");
        let filter = doc! { "email": &user_data.email };
        if collection.find_one(filter, None).await.unwrap().is_some() {
            return HttpResponse::Conflict().json(ApiResponse { message: "User with this email already exists".to_string(),token:None });

        }

        // Insert the user data into MongoDB
        let user_document = mongodb::bson::to_document(&user_data.into_inner()).unwrap();
        if let Err(err) = collection.insert_one(user_document, None).await {
            eprintln!("Error inserting user data: {:?}", err);
            return HttpResponse::InternalServerError().json(ApiResponse { message: "Error registering user".to_string(),token:None });
        }

        HttpResponse::Ok().json(ApiResponse { message: "User registered successfully".to_string(),token:None })
    }
    async fn login(login_data: web::Json<LoginRequest>, db: web::Data<Database>) -> impl Responder {
        // Validation logic
        println!("Received login request");
        println!("Received login request");

        // Check if the user with the given email exists
        let collection: Collection = db.collection("users");
        let filter = doc! { "email": &login_data.email };
        if let Some(user_document) = collection.find_one(filter, None).await.unwrap() {
            // Validate the password
            let stored_password = user_document.get_str("password").unwrap();
            if stored_password == login_data.password {
                // Generate JWT token
                let user_id = user_document.get_object_id("_id").unwrap(); // Assuming "_id" is the field for user ID
                let jwt_token = generate_jwt_token(&user_id);

                // Return JWT token in the response
                return HttpResponse::Ok().json(ApiResponse { message: "Login successful".to_string(), token: Some(jwt_token) });
            } else {
                return HttpResponse::Unauthorized().json(ApiResponse { message: "Invalid password".to_string(),token:None });
            }
        } else {
            return HttpResponse::NotFound().json(ApiResponse { message: "User not found".to_string(),token:None });
        }
    }
    async fn insert_patent_data(
    patent_data: web::Json<PatentDataForUser>,
    db: web::Data<Database>,
) -> impl Responder {
    // Assuming you have a collection named "patent_data" for storing PatentDataForUser
    let collection: Collection = db.collection("patent_data");

    // Find and increment the counter, and get the next value
    let result = collection.find_one_and_update(
        doc! {},
        doc! { "$inc": { "id": 1 } },
        FindOneAndUpdateOptions::builder()
            .upsert(true)
            .return_document(ReturnDocument::After)
            .build(),
    ).await;

    match result {
        Ok(Some(doc)) => {
            let new_id = doc.get_i32("id").unwrap();
            println!("New ID: {}", new_id);

            // Set the new ID in the patent data
            let mut patent_data_inner = patent_data.into_inner();
            patent_data_inner.id = new_id;

            // Insert the patent data into MongoDB
            let patent_document = mongodb::bson::to_document(&patent_data_inner).unwrap();
            println!("Received patent request");
            if let Err(err) = collection.insert_one(patent_document, None).await {
                eprintln!("Error inserting patent data: {:?}", err);
                return HttpResponse::InternalServerError().json(ApiResponse {
                    message: "Error inserting patent data".to_string(),
                    token: None,
                });
            }

            HttpResponse::Ok().json(ApiResponse {
                message: "Patent data inserted successfully".to_string(),
                token: None,
            })
        }
        Ok(None) => {
            eprintln!("Counter document not found");
            HttpResponse::InternalServerError().json(ApiResponse {
                message: "Counter document not found".to_string(),
                token: None,
            })
        }
        Err(err) => {
            eprintln!("Error updating counter: {:?}", err);
            HttpResponse::InternalServerError().json(ApiResponse {
                message: "Error updating counter".to_string(),
                token: None,
            })
        }
    }
}

    
    async fn verify_token(token: web::Path<String>) -> impl Responder {
        println!("JWT Token request1");
        match verify_jwt_token(&token) {
            Ok(claims) => HttpResponse::Ok().json(claims),
            Err(_) => HttpResponse::Unauthorized().finish(),
            
        }
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/register").route(web::post().to(register)))
        .service(web::resource("/login").route(web::post().to(login)))
        .service(
            web::resource("/insert_patent_data")
                .route(web::post().to(insert_patent_data)),
        )
        .service(web::resource("/verify_token/{token}").route(web::get().to(verify_token)));
        
        
    }
