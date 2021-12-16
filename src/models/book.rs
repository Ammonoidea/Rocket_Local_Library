use mongodb::bson::{doc, Document};

use crate::models::documentable::Persistable;

#[derive(Debug, PartialEq, Eq)]
pub struct Book {
    pub title: String,
    pub author_id: String,
    pub summary: String,
    pub isbn: String,
    pub genre_ids: Vec<String>
}

impl Persistable for Book {
    fn to_document(&self) -> Document {
        doc!{
            "title": self.title.clone(),
            "authorId": self.author_id.clone(),
            "summary": self.summary.clone(),
            "isbn": self.isbn.clone(),
            "genreIds": self.genre_ids.join(",")
        }
    }

    fn from_document(document: &Document) -> Self {
        unimplemented!();
    }

    fn coll_name() -> String {
        unimplemented!();
    }
}
