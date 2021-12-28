use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub title: String,
    pub author: ObjectId,
    pub summary: String,
    pub isbn: String,
    pub genre: Vec<ObjectId>,
}
