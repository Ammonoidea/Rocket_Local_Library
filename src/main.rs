#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod repositories;

use mongodb::bson::{doc, Bson, Document};
use mongodb::sync::Client;

use controllers::author_controller;
use controllers::index_controller;
use repositories::author_collection::AuthorCollection;
use repositories::book_collection::BookCollection;
use repositories::book_instance_collection::BookInstanceCollection;

use rocket_dyn_templates::Template;
use std::collections::HashMap;

use rocket::fs::{relative, FileServer};
use rocket::State;

#[launch]
fn rocket() -> _ {
    let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
    let db = client.database("library");
    rocket::build()
        .manage(BookCollection::build(&db))
        .manage(BookInstanceCollection::build(&db))
        .manage(AuthorCollection::build(&db))
        .mount("/", routes![index_controller::index])
        .mount("/authors", routes![author_controller::author_list])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
