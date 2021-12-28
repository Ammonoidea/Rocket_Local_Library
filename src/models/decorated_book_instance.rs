use crate::models::book::Book;
use crate::models::book_status::BookStatus;
use crate::models::expanded_book_instance::ExpandedBookInstance;

use bson::DateTime;
use serde::{Serialize};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DecoratedBookInstance {
    pub imprint: String,
    pub status: BookStatus,
    pub due_back: Option<DateTime>,
    pub book_obj: Book,
    pub url: String,
}

impl DecoratedBookInstance {
    pub fn from_expanded_book_instance(
        expanded_book_instance: ExpandedBookInstance,
    ) -> DecoratedBookInstance {
        let mut url: String = "catalog/bookinstance/".to_string();
        url.push_str(&expanded_book_instance._id.to_string());
        DecoratedBookInstance {
            imprint: expanded_book_instance.imprint,
            status: expanded_book_instance.status,
            due_back: expanded_book_instance.due_back,
            book_obj: expanded_book_instance.book_obj[0].clone(),
            url: url,
        }
    }
}
