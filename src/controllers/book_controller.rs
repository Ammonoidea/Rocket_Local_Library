use futures::executor::block_on;
use futures::Future;

use rocket::serde::Serialize;
use rocket::State;
use rocket_dyn_templates::Template;

use std::time::Duration;
use tokio::time::{timeout, Timeout};

use crate::models::decorated_book::DecoratedBook;
use crate::models::expanded_book::ExpandedBook;
use crate::repositories::book_collection::BookCollection;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct BookTemplateContext<'r> {
    book_list: &'r Vec<DecoratedBook>,
}

#[get("/create")]
pub fn book_create_get() -> &'static str {
    "NOT IMPLEMENTED: Book create get"
}

#[post("/create")]
pub fn book_create_post() -> &'static str {
    "NOT IMPLEMENTED: Book create post"
}

// this should be a DELETE, not a GET, but I am following the express tutorial
#[get("/<id>/delete")]
pub fn book_delete_get(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book delete get".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a DELETE, not a POST, but I am following the express tutorial
#[post("/<id>/delete")]
pub fn book_delete_post(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book delete post".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a PUT, not a GET, but I am following the express tutorial
#[get("/<id>/update")]
pub fn book_update_get(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book update get".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a PUT, not a POST, but I am following the express tutorial
#[post("/<id>/update")]
pub fn book_update_post(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book update post".to_owned();
    owned_string.push_str(id);
    owned_string
}

#[get("/")]
pub fn book_list(book_coll: &State<BookCollection>) -> Template {
    let f_expanded_book_list = book_coll.list_books();
    let expanded_book_list: Vec<ExpandedBook> = block_on(f_expanded_book_list);
    let mut decorated_books: Vec<DecoratedBook> = Vec::new();
    for expanded_book in expanded_book_list {
        let decorated_book = DecoratedBook::from_expanded_book(expanded_book);
        decorated_books.push(decorated_book);
    }
    Template::render(
        "book_list",
        &BookTemplateContext {
            book_list: &decorated_books,
        },
    )
}

#[get("/<id>")]
pub fn book_detail(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book detail".to_owned();
    owned_string.push_str(id);
    owned_string
}
