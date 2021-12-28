use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Book {
    pub title: String,
    pub author_id: String,
    pub summary: String,
    pub isbn: String,
    pub genre_ids: Vec<String>,
}
