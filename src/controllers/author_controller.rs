use rocket::http::Status;
use rocket::serde::Serialize;
use rocket::State;
use rocket_dyn_templates::Template;

use crate::models::author::Author;
use crate::models::decorated_author::DecoratedAuthor;
use crate::models::decorated_book::DecoratedBook;
use crate::repositories::author_collection::AuthorCollection;
use crate::repositories::book_collection::BookCollection;
use crate::responses::template_or_status_response::TemplateOrStatusResponse;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct AuthorTemplateContext<'r> {
    authors: &'r Vec<DecoratedAuthor>,
}

#[get("/create")]
pub fn author_create_get() -> &'static str {
    "NOT IMPLEMENTED: Author create get"
}

#[post("/create")]
pub fn author_create_post() -> &'static str {
    "NOT IMPLEMENTED: Author create post"
}

// this should be a DELETE, not a GET, but I am following the express tutorial
#[get("/<id>/delete")]
pub fn author_delete_get(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Author delete get".to_owned();
    owned_string.push_str(id);
    owned_string
}

#[post("/<id>/delete")]
pub fn author_delete_post(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Author delete post".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a PUT, not a GET, but I am following the express tutorial
#[get("/<id>/update")]
pub fn author_update_get(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Author update get".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a PUT, not a POST, but I am following the express tutorial
#[post("/<id>/update")]
pub fn author_update_post(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Author update post".to_owned();
    owned_string.push_str(id);
    owned_string
}

#[get("/")]
pub fn author_list(author_coll: &State<AuthorCollection>) -> Template {
    let authors: Vec<Author> = author_coll.list_authors();
    let mut decorated_authors: Vec<DecoratedAuthor> = Vec::new();
    for author in authors {
        let decorated_author = DecoratedAuthor::from_author(&author);
        decorated_authors.push(decorated_author);
    }
    Template::render(
        "authors_list",
        &AuthorTemplateContext {
            authors: &decorated_authors,
        },
    )
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct AuthorDetailTemplateContext<'r> {
    author: &'r DecoratedAuthor,
    author_books: &'r Vec<DecoratedBook>,
}
#[get("/<id>")]
pub fn author_detail(
    id: &str,
    author_coll: &State<AuthorCollection>,
    book_coll: &State<BookCollection>,
) -> TemplateOrStatusResponse {
    let author = match author_coll.get_author_by_id(id) {
        Some(a) => a,
        None => return TemplateOrStatusResponse::Status(Status::NotFound),
    };
    let decorated_author = DecoratedAuthor::from_author(&author);

    let books = book_coll.get_books_by_author(id);
    let mut decorated_books: Vec<DecoratedBook> = Vec::new();
    for book in books {
        let decorated_book = DecoratedBook::from_book(book);
        decorated_books.push(decorated_book);
    }
    TemplateOrStatusResponse::Template(Template::render(
        "author_detail",
        &AuthorDetailTemplateContext {
            author: &decorated_author,
            author_books: &decorated_books,
        },
    ))
}
