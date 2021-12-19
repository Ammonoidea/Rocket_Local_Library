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
pub fn book_instance_list() -> &'static str {
    "NOT IMPLEMENTED: Book Instance list"
}

#[get("/<id>")]
pub fn book_instance_detail(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book Instance detail".to_owned();
    owned_string.push_str(id);
    owned_string
}
