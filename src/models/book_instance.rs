use chrono::prelude::*;
use mongodb::bson::{doc, Document};
use crate::models::documentable::Persistable;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct BookInstance {
    pub book_id: String,
    pub imprint: String,
    pub status: BookStatus,
    pub due_back: NaiveDate
}

#[derive(Debug, PartialEq, Eq)]
pub enum BookStatus {
    Available,
    Maintenance,
    Loaned,
    Reserved
}

impl fmt::Display for BookStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Persistable for BookInstance {
    fn to_document(&self) -> Document {
        doc!{
            "bookId": self.book_id.clone(),
            "imprint": self.imprint.clone(),
            "status": self.status.to_string(),
            "dueBack": self.due_back.to_string()
        }
    }
}