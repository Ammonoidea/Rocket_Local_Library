#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate chrono;
#[macro_use(bson, doc)] extern crate mongodb;

use mongodb::{Bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

mod models;

#[get("/")]
fn index() -> &'static str {
	"Hello, Christina!"
}

fn main() {
	let client = Client::connect("localhost", 3306)
		.expect("Failed to initialize standalone client.");
	let coll = client.db("library").collection("books");
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
	rocket::ignite().mount("/", routes![index]).launch();

}
