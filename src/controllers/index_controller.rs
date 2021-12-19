use crate::AuthorCollection;
use crate::BookCollection;
use crate::BookInstanceCollection;
use rocket::State;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
pub fn index(
    book_coll: &State<BookCollection>,
    book_instance_coll: &State<BookInstanceCollection>,
    author_coll: &State<AuthorCollection>,
) -> Template {
    // the type for estimated_document_count is Into<Option<...>> and I don't understand what
    // the Into is doing here
    let num_books = match book_coll.count_books() {
        Ok(num) => num,
        Err(e) => panic!(
            "{}",
            format!(
                "Error: could not get an estimated number of books {}",
                e.to_string()
            )
        ),
    };
    print!("Found {:?} book(s)\n", num_books);

    let mut context = HashMap::<String, String>::new();
    context.insert("num_books".to_string(), num_books.to_string());
    context.insert(
        "num_instances_available".to_string(),
        book_instance_coll
            .count_books_by_status(&"Available")
            .to_string(),
    );
    context.insert(
        "num_instances".to_string(),
        book_instance_coll
            .count_book_instances()
            .unwrap()
            .to_string(),
    );
    context.insert(
        "num_authors".to_string(),
        author_coll.count_authors().unwrap().to_string(),
    );
    Template::render("index", context)
}
