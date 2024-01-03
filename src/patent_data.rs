use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct PatentDataForUser {
    pub id: i32,
    pub paragraph: String,
}
