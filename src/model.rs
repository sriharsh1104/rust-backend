use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub confirm_password: String,
    
}
