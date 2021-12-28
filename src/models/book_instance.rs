use chrono::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct BookInstance {
    pub book_id: String,
    pub imprint: String,
    pub status: BookStatus,
    pub due_back: NaiveDate,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BookStatus {
    Available,
    Maintenance,
    Loaned,
    Reserved,
}

impl fmt::Display for BookStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
