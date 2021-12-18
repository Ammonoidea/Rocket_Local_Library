#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod repositories;

use mongodb::bson::{doc, Bson, Document};
use mongodb::sync::{Client, Database};

use controllers::author_controller;
use repositories::book_collection::BookCollection;

use rocket_dyn_templates::Template;
use std::collections::HashMap;

use rocket::fs::{relative, FileServer};
use rocket::{State};

#[get("/")]
fn index(book_coll: &State<BookCollection>) -> Template {
	// the type for estimated_document_count is Into<Option<...>> and I don't understand what
	// the Into is doing here
	let num_books = match book_coll.count_books() {
		Ok(num) => num,
		Err(e) => panic!("Error: could not get an estimated number of books")
	};
	print!("Found {:?} book(s)\n", num_books);

    let mut context = HashMap::<String, String>::new();
    context.insert("num_books".to_string(), num_books.to_string());
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
    	.manage(BookCollection::build_book_collection(db))
        .mount("/", routes![index])
        .mount("/authors", routes![author_controller::author_list])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
