use bson::oid::ObjectId;
use bson::DateTime;
use serde::Deserialize;

use crate::models::book_status::BookStatus;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct BookInstance {
    pub _id: ObjectId,
    pub book_id: ObjectId,
    pub imprint: String,
    pub status: BookStatus,
    pub due_back: Option<DateTime>,
}
