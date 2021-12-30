use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub _id: ObjectId,
    pub title: String,
    pub author: ObjectId,
    pub summary: String,
    pub isbn: String,
    pub genre: Vec<ObjectId>,
}
