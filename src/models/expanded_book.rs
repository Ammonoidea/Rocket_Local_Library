use serde::Deserialize;


use crate::models::author::Author;
#[derive(Deserialize, Debug)]
pub struct ExpandedBook {
    pub id: String,
    pub title: String,
    pub author: String,
    pub summary: String,
    pub isbn: String,
    pub genre: Vec<String>,
    pub author_obj: Author,
}