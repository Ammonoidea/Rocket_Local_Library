use rocket::serde::Serialize;

use crate::models::expanded_book::ExpandedBook;

use crate::models::decorated_author::DecoratedAuthor;

#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DecoratedBook {
    pub title: String,
    pub author: DecoratedAuthor,
    pub summary: String,
    pub isbn: String,
    pub genre: Vec<String>,
    pub url: String
}

impl DecoratedBook {
    pub fn from_expanded_book(expanded_book: ExpandedBook) -> DecoratedBook {
        let mut url: String = "catalog/book".to_string();
        url.push_str(&expanded_book.id);
        DecoratedBook {
            title: expanded_book.title,
            author: DecoratedAuthor::from_author(expanded_book.author_obj),
            summary: expanded_book.summary,
            isbn: expanded_book.isbn,
            genre: expanded_book.genre,
            url: url,
        }
    }
}
