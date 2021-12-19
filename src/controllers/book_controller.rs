#[get("/create")]
pub fn book_create_get() -> &'static str {
    "NOT IMPLEMENTED: Book create get"
}

#[post("/create")]
pub fn book_create_post() -> &'static str {
    "NOT IMPLEMENTED: Book create post"
}

// this should be a DELETE, not a GET, but I am following the express tutorial
#[get("/<id>/delete")]
pub fn book_delete_get(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book delete get".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a DELETE, not a POST, but I am following the express tutorial
#[post("/<id>/delete")]
pub fn book_delete_post(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book delete post".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a PUT, not a GET, but I am following the express tutorial
#[get("/<id>/update")]
pub fn book_update_get(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book update get".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a PUT, not a POST, but I am following the express tutorial
#[post("/<id>/update")]
pub fn book_update_post(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book update post".to_owned();
    owned_string.push_str(id);
    owned_string
}

#[get("/")]
pub fn book_list() -> &'static str {
    "NOT IMPLEMENTED: Book list"
}

#[get("/<id>")]
pub fn book_detail(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Book detail".to_owned();
    owned_string.push_str(id);
    owned_string
}
