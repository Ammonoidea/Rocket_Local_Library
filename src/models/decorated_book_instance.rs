use crate::date_utils::date_utility::format_date_3m_dsfx_year;
use crate::models::book::Book;
use crate::models::book_instance::BookInstance;
use crate::models::book_status::BookStatus;
use crate::models::expanded_book_instance::ExpandedBookInstance;

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DecoratedBookInstance {
    pub id: String,
    pub imprint: String,
    pub status: BookStatus,
    pub due_back: Option<String>,
    pub book_obj: Option<Book>,
    pub url: String,
}

impl DecoratedBookInstance {
    pub fn from_expanded_book_instance(
        expanded_book_instance: ExpandedBookInstance,
    ) -> DecoratedBookInstance {
        let mut url: String = "/catalog/bookinstance/".to_string();
        url.push_str(&expanded_book_instance._id.to_string());

        let pretty_date = expanded_book_instance
            .due_back
            .map(format_date_3m_dsfx_year);
        DecoratedBookInstance {
            id: expanded_book_instance._id.to_hex(),
            imprint: expanded_book_instance.imprint,
            status: expanded_book_instance.status,
            due_back: pretty_date,
            book_obj: Some(expanded_book_instance.book_obj[0].clone()),
            url: url,
        }
    }

    pub fn from_book_instance(book_instance: BookInstance) -> DecoratedBookInstance {
        let mut url: String = "/catalog/bookinstance/".to_string();
        url.push_str(&book_instance._id.to_string());

        let pretty_date = book_instance.due_back.map(format_date_3m_dsfx_year);
        DecoratedBookInstance {
            id: book_instance._id.to_hex(),
            imprint: book_instance.imprint,
            status: book_instance.status,
            due_back: pretty_date,
            book_obj: None,
            url: url,
        }
    }
}
