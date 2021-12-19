#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod repositories;

use mongodb::bson::{doc, Bson, Document};
use mongodb::sync::Client;

use controllers::author_controller;
use repositories::author_collection::AuthorCollection;
use repositories::book_collection::BookCollection;
use repositories::book_instance_collection::BookInstanceCollection;

use rocket_dyn_templates::Template;
use std::collections::HashMap;

use rocket::fs::{relative, FileServer};
use rocket::State;

#[get("/")]
fn index(
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
    context.insert("num_instances".to_string(), book_instance_coll.count_book_instances().unwrap().to_string());
    context.insert("num_authors".to_string(), author_coll.count_authors().unwrap().to_string());
    Template::render("index", context)
}

#[launch]
fn rocket() -> _ {
    let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
    let db = client.database("library");
    let coll = db.collection::<Document>("books");
    let mut cursor = coll
        .find(Some(doc!()), None)
        .ok()
        .expect("Failed to execute find.");

    let item = cursor.next();

    // cursor.next() returns an Option<Result<Document>>
    match item {
        Some(Ok(doc)) => match doc.get("title") {
            Some(&Bson::String(ref title)) => println!("{}", title),
            _ => panic!("Expected title to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned no results!"),
    }
    rocket::build()
        .manage(BookCollection::build(&db))
        .manage(BookInstanceCollection::build(&db))
        .manage(AuthorCollection::build(&db))
        .mount("/", routes![index])
        .mount("/authors", routes![author_controller::author_list])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
