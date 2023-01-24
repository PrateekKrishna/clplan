use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct user{
    pub id: u64, 
    pub name: String,
    pub location: String,
    pub password: String
}