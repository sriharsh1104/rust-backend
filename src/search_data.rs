use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    // Define fields for your search criteria here
    pub keyword: String,
}
