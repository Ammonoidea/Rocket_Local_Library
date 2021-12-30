use rocket::http::Status;
use rocket::serde::Serialize;
use rocket::State;
use rocket_dyn_templates::Template;

use crate::models::decorated_book::DecoratedBook;
use crate::models::decorated_genre::DecoratedGenre;
use crate::models::genre::Genre;
use crate::repositories::book_collection::BookCollection;
use crate::repositories::genre_collection::GenreCollection;
use crate::responses::template_or_status_response::TemplateOrStatusResponse;

#[get("/create")]
pub fn genre_create_get() -> &'static str {
    "NOT IMPLEMENTED: Genre create get"
}

#[post("/create")]
pub fn genre_create_post() -> &'static str {
    "NOT IMPLEMENTED: Genre create post"
}

// this should be a DELETE, not a GET, but I am following the express tutorial
#[get("/<id>/delete")]
pub fn genre_delete_get(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Genre delete get".to_owned();
    owned_string.push_str(id);
    owned_string
}

#[post("/<id>/delete")]
pub fn genre_delete_post(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Genre delete post".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a PUT, not a GET, but I am following the express tutorial
#[get("/<id>/update")]
pub fn genre_update_get(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Genre update get".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a PUT, not a POST, but I am following the express tutorial
#[post("/<id>/update")]
pub fn genre_update_post(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Genre update post".to_owned();
    owned_string.push_str(id);
    owned_string
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct GenreListTemplateContext<'r> {
    genres: &'r Vec<DecoratedGenre>,
}

#[get("/")]
pub fn genre_list(genre_coll: &State<GenreCollection>) -> Template {
    let genres: Vec<Genre> = genre_coll.list_genres();
    let mut decorated_genres: Vec<DecoratedGenre> = Vec::new();
    for genre in genres {
        let decorated_genre = DecoratedGenre::from_genre(&genre);
        decorated_genres.push(decorated_genre);
    }
    Template::render(
        "genre_list",
        &GenreListTemplateContext {
            genres: &decorated_genres,
        },
    )
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct GenreDetailTemplateContext<'r> {
    genre: &'r Genre,
    genre_books: &'r Vec<DecoratedBook>,
}
#[get("/<id>")]
pub fn genre_detail(
    id: &str,
    genre_coll: &State<GenreCollection>,
    book_coll: &State<BookCollection>,
) -> TemplateOrStatusResponse {
    println!("*** in genre detail for genre {:?}", &id);
    let genre = match genre_coll.get_genre_by_id(id) {
        Some(g) => g,
        None => return TemplateOrStatusResponse::Status(Status::NotFound),
    };
    let genre_books = book_coll.get_books_by_genre(id);
    let mut decorated_genre_books: Vec<DecoratedBook> = Vec::new();
    for book in genre_books {
        let decorated_book = DecoratedBook::from_book(book);
        decorated_genre_books.push(decorated_book);
    }
    TemplateOrStatusResponse::Template(Template::render(
        "genre_detail",
        &GenreDetailTemplateContext {
            genre: &genre,
            genre_books: &decorated_genre_books,
        },
    ))
}
