use crate::AuthorCollection;
use crate::BookCollection;
use crate::BookInstanceCollection;
use crate::GenreCollection;
use rocket::response::Redirect;
use rocket::{uri, State};
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
pub fn bare_root() -> Redirect {
    Redirect::to(uri!("/catalog/"))
}

#[get("/catalog")]
pub fn index(
    book_coll: &State<BookCollection>,
    book_instance_coll: &State<BookInstanceCollection>,
    author_coll: &State<AuthorCollection>,
    genre_coll: &State<GenreCollection>,
) -> Template {
    // the type for estimated_document_count is Into<Option<...>> and I don't understand what
    // the Into is doing here
    let num_books = book_coll.count_books();
    let available_books = book_instance_coll.count_books_by_status(&"Available");
    let num_book_instances = book_instance_coll.count_book_instances();
    let num_authors = author_coll.count_authors();
    let num_genres = genre_coll.count_genres();

    let mut context = HashMap::<String, String>::new();
    context.insert("num_books".to_string(), num_books.unwrap().to_string());
    context.insert(
        "num_instances_available".to_string(),
        available_books.to_string(),
    );
    context.insert(
        "num_instances".to_string(),
        num_book_instances.unwrap().to_string(),
    );
    context.insert("num_authors".to_string(), num_authors.unwrap().to_string());
    context.insert("num_genres".to_string(), num_genres.unwrap().to_string());
    Template::render("index", context)
}
