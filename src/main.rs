#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod repositories;

use mongodb::bson::{doc, Bson, Document};
use mongodb::sync::Client;

use controllers::author_controller;
use controllers::book_controller;
use controllers::book_instance_controller;
use controllers::genre_controller;
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
        .mount(
            "/catalog/author",
            routes![
                author_controller::author_create_get,
                author_controller::author_create_post,
                author_controller::author_delete_get,
                author_controller::author_delete_post,
                author_controller::author_update_get,
                author_controller::author_update_post,
                author_controller::author_detail
            ],
        )
        .mount("/catalog/authors", routes![author_controller::author_list])
        .mount(
            "/catalog/book",
            routes![
                book_controller::book_create_get,
                book_controller::book_create_post,
                book_controller::book_delete_get,
                book_controller::book_delete_post,
                book_controller::book_update_get,
                book_controller::book_update_post,
                book_controller::book_detail
            ],
        )
        .mount("/catalog/books", routes![book_controller::book_list])
        .mount(
            "/catalog/bookinstance",
            routes![
                book_instance_controller::book_instance_create_get,
                book_instance_controller::book_instance_create_post,
                book_instance_controller::book_instance_delete_get,
                book_instance_controller::book_instance_delete_post,
                book_instance_controller::book_instance_update_get,
                book_instance_controller::book_instance_update_post,
                book_instance_controller::book_instance_detail,
            ],
        )
        .mount(
            "/catalog/bookinstances",
            routes![book_instance_controller::book_instance_list],
        )
        .mount(
            "/catalog/genre",
            routes![
                genre_controller::genre_create_get,
                genre_controller::genre_create_post,
                genre_controller::genre_delete_get,
                genre_controller::genre_delete_post,
                genre_controller::genre_update_get,
                genre_controller::genre_update_post,
                genre_controller::genre_detail
            ],
        )
        .mount("/catalog/genres", routes![genre_controller::genre_list])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
