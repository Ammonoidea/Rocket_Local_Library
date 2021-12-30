use rocket::http::Status;
use rocket::serde::Serialize;
use rocket::State;
use rocket_dyn_templates::Template;

use crate::models::decorated_book::DecoratedBook;
use crate::models::decorated_book_instance::DecoratedBookInstance;
use crate::models::decorated_book_with_genre::DecoratedBookWithGenre;
use crate::models::expanded_book::ExpandedBook;
use crate::repositories::book_collection::BookCollection;
use crate::responses::template_or_status_response::TemplateOrStatusResponse;
use crate::BookInstanceCollection;

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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct BookListTemplateContext<'r> {
    book_list: &'r Vec<DecoratedBook>,
}

#[get("/")]
pub fn book_list(book_coll: &State<BookCollection>) -> Template {
    let expanded_book_list: Vec<ExpandedBook> = book_coll.list_books();
    let mut decorated_books: Vec<DecoratedBook> = Vec::new();
    for expanded_book in expanded_book_list {
        let decorated_book = DecoratedBook::from_expanded_book(expanded_book);
        decorated_books.push(decorated_book);
    }
    Template::render(
        "books_list",
        &BookListTemplateContext {
            book_list: &decorated_books,
        },
    )
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct BookDetailTemplateContext<'r> {
    book: &'r DecoratedBookWithGenre,
    book_instances: &'r Vec<DecoratedBookInstance>,
}
#[get("/<id>")]
pub fn book_detail(
    id: &str,
    book_coll: &State<BookCollection>,
    book_instance_coll: &State<BookInstanceCollection>,
) -> TemplateOrStatusResponse {
    let book = match book_coll.get_book_by_id(id) {
        Some(book) => book,
        None => return TemplateOrStatusResponse::Status(Status::NotFound),
    };
    let decorated_book = DecoratedBookWithGenre::from_expanded_book(book);

    let book_instances = book_instance_coll.get_book_instance_by_book(id);
    let mut decorated_book_instances: Vec<DecoratedBookInstance> = Vec::new();
    for book_instance in book_instances {
        let decorated_book_instance = DecoratedBookInstance::from_book_instance(book_instance);
        decorated_book_instances.push(decorated_book_instance);
    }

    TemplateOrStatusResponse::Template(Template::render(
        "book_detail",
        &BookDetailTemplateContext {
            book: &decorated_book,
            book_instances: &decorated_book_instances,
        },
    ))
}
