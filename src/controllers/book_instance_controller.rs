use rocket::serde::Serialize;
use rocket::State;
use rocket_dyn_templates::Template;

use crate::models::decorated_book_instance::DecoratedBookInstance;
use crate::models::expanded_book_instance::ExpandedBookInstance;
use crate::repositories::book_instance_collection::BookInstanceCollection;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct BookInstanceTemplateContext<'r> {
    bookinstance_list: &'r Vec<DecoratedBookInstance>,
}

#[get("/create")]
pub fn book_instance_create_get() -> &'static str {
    "NOT IMPLEMENTED: Book Instance create get"
}

#[post("/create")]
pub fn book_instance_create_post() -> &'static str {
    "NOT IMPLEMENTED: Book Instance create post"
}

// this should be a DELETE, not a GET, but I am following the express tutorial
#[get("/<id>/delete")]
pub fn book_instance_delete_get(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book Instance delete get".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a DELETE, not a POST, but I am following the express tutorial
#[post("/<id>/delete")]
pub fn book_instance_delete_post(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book Instance delete post".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a PUT, not a GET, but I am following the express tutorial
#[get("/<id>/update")]
pub fn book_instance_update_get(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book Instance update get".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a PUT, not a POST, but I am following the express tutorial
#[post("/<id>/update")]
pub fn book_instance_update_post(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book Instance update post".to_owned();
    owned_string.push_str(id);
    owned_string
}

#[get("/")]
pub fn book_instance_list(book_instance_coll: &State<BookInstanceCollection>) -> Template {
    let expanded_book_instance_list: Vec<ExpandedBookInstance> = book_instance_coll.list_book_instances();
    let mut decorated_book_instances: Vec<DecoratedBookInstance> = Vec::new();
    for expanded_book_instance in expanded_book_instance_list {
        let decorated_book_instance =
            DecoratedBookInstance::from_expanded_book_instance(expanded_book_instance);
        decorated_book_instances.push(decorated_book_instance);
    }
    Template::render(
        "bookinstance_list",
        &BookInstanceTemplateContext {
            bookinstance_list: &decorated_book_instances,
        },
    )
}

#[get("/<id>")]
pub fn book_instance_detail(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book Instance detail".to_owned();
    owned_string.push_str(id);
    owned_string
}
