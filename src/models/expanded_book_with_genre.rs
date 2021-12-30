use bson::oid::ObjectId;
use serde::Deserialize;

use crate::models::author::Author;
use crate::models::genre::Genre;

#[derive(Deserialize, Debug)]
pub struct ExpandedBookWithGenre {
    // this must be the bad one
    pub _id: ObjectId,
    pub title: String,
    pub author: ObjectId,
    pub summary: String,
    pub isbn: String,
    pub genre: Vec<ObjectId>,
    pub genre_objs: Vec<Genre>,
    pub author_obj: Vec<Author>,
}
