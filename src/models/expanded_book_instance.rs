use crate::models::book::Book;
use crate::models::book_status::BookStatus;

use bson::oid::ObjectId;
use bson::DateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ExpandedBookInstance {
    pub _id: ObjectId,
    pub book: ObjectId,
    pub imprint: String,
    pub status: BookStatus,
    pub due_back: Option<DateTime>,
    pub book_obj: Vec<Book>,
}
