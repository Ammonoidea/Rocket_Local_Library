use rocket::serde::Serialize;

use crate::models::book::Book;
use crate::models::expanded_book::ExpandedBook;

use crate::models::decorated_author::DecoratedAuthor;

#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DecoratedBook {
    pub title: String,
    pub author: Option<DecoratedAuthor>,
    pub summary: String,
    pub isbn: String,
    pub genre: Vec<String>,
    pub url: String,
}

impl DecoratedBook {
    pub fn from_expanded_book(expanded_book: ExpandedBook) -> DecoratedBook {
        let mut url: String = "book/".to_string();
        url.push_str(&expanded_book._id.to_string());

        let mut genres: Vec<String> = Vec::new();
        for object_id in expanded_book.genre {
            genres.push(object_id.to_hex());
        }
        DecoratedBook {
            title: expanded_book.title,
            author: Some(DecoratedAuthor::from_author(&expanded_book.author_obj[0])),
            summary: expanded_book.summary,
            isbn: expanded_book.isbn,
            genre: genres,
            url: url,
        }
    }

    pub fn from_book(book: Book) -> DecoratedBook {
        let mut url: String = "catalog/book/".to_string();
        url.push_str(&book._id.to_string());

        let mut genres: Vec<String> = Vec::new();
        for object_id in book.genre {
            genres.push(object_id.to_hex());
        }
        DecoratedBook {
            title: book.title,
            author: None,
            summary: book.summary,
            isbn: book.isbn,
            genre: genres,
            url: url,
        }
    }
}
