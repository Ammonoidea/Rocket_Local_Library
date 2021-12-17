#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod controllers;
mod models;

use mongodb::bson::{doc, Bson, Document};
use mongodb::sync::{Client};

use controllers::author_controller;

use rocket_dyn_templates::Template;
use std::collections::HashMap;

use rocket::fs::{FileServer, relative};

#[get("/")]
fn index() -> Template {
	let context = HashMap::<String, String>::new();
	Template::render("index", context)
}

#[launch]
fn rocket() -> _ {
	let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
	let db = client.database("library");
	let coll = db.collection::<Document>("books");
	let mut cursor = coll.find(Some(doc!()), None)
		.ok().expect("Failed to execute find.");

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
		.mount("/", routes![index])
		.mount("/authors", routes![author_controller::author_list])
        .mount("/", FileServer::from(relative!("static")))
		.attach(Template::fairing())

}
