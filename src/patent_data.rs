use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PatentDataForUser {
    pub id: i32,
    pub paragraph: String,
}
