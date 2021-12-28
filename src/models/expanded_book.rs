use bson::oid::ObjectId;
use serde::Deserialize;

use crate::models::author::Author;

#[derive(Deserialize, Debug)]
pub struct ExpandedBook {
    // this must be the bad one
    pub _id: ObjectId,
    pub title: String,
    pub author: ObjectId,
    pub summary: String,
    pub isbn: String,
    pub genre: Vec<ObjectId>,
    pub author_obj: Vec<Author>,
}
