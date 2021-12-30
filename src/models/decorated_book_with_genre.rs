use rocket::serde::Serialize;

use crate::models::expanded_book_with_genre::ExpandedBookWithGenre;

use crate::models::decorated_author::DecoratedAuthor;
use crate::models::decorated_genre::DecoratedGenre;

#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DecoratedBookWithGenre {
    pub title: String,
    pub author: Option<DecoratedAuthor>,
    pub summary: String,
    pub isbn: String,
    pub genres: Vec<DecoratedGenre>,
    pub url: String,
}

impl DecoratedBookWithGenre {
    pub fn from_expanded_book(expanded_book: ExpandedBookWithGenre) -> DecoratedBookWithGenre {
        let mut url: String = "book/".to_string();
        url.push_str(&expanded_book._id.to_string());

        let mut decorated_genres: Vec<DecoratedGenre> = Vec::new();
        for genre in expanded_book.genre_objs {
            let decorated_genre = DecoratedGenre::from_genre(&genre);
            decorated_genres.push(decorated_genre);
        }
        DecoratedBookWithGenre {
            title: expanded_book.title,
            author: Some(DecoratedAuthor::from_author(&expanded_book.author_obj[0])),
            summary: expanded_book.summary,
            isbn: expanded_book.isbn,
            genres: decorated_genres,
            url: url,
        }
    }
}
