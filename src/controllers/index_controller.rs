use crate::AuthorCollection;
use crate::BookCollection;
use crate::BookInstanceCollection;
use rocket::State;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
pub async fn index(
    book_coll: &State<BookCollection>,
    book_instance_coll: &State<BookInstanceCollection>,
    author_coll: &State<AuthorCollection>,
) -> Template {
    // the type for estimated_document_count is Into<Option<...>> and I don't understand what
    // the Into is doing here
    let f_num_books = book_coll.count_books();
    let f_available_books = book_instance_coll.count_books_by_status(&"Available");
    let f_num_book_instances = book_instance_coll.count_book_instances();
    let f_num_authors = author_coll.count_authors();
    let (num_books, available_books, num_book_instances, num_authors) = futures::join!(
        f_num_books, 
        f_available_books,
        f_num_book_instances, 
        f_num_authors);

    let mut context = HashMap::<String, String>::new();
    context.insert("num_books".to_string(), num_books.unwrap().to_string());
    context.insert(
        "num_instances_available".to_string(),
        available_books
            .to_string(),
    );
    context.insert(
        "num_instances".to_string(),
        num_book_instances.unwrap()
            .to_string(),
    );
    context.insert(
        "num_authors".to_string(),
        num_authors.unwrap().to_string(),
    );
    Template::render("index", context)
}
