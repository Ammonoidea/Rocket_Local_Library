use chrono::prelude::*;

pub enum BookStatus {
    Available,
    Maintenance,
    Loaned,
    Reserved
}

#[derive(Debug, PartialEq, Eq)]
pub struct BookInstance {
    pub book_id: String,
    pub imprint: String,
    pub status: BookStatus,
    pub due_back: NaiveDate
}