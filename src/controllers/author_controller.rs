#[get("/create")]
pub fn author_create_get() -> &'static str {
    "NOT IMPLEMENTED: Author create get"
}

#[post("/create")]
pub fn author_create_post() -> &'static str {
    "NOT IMPLEMENTED: Author create post"
}

// this should be a DELETE, not a GET, but I am following the express tutorial
#[get("/<id>/delete")]
pub fn author_delete_get(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Author delete get".to_owned();
    owned_string.push_str(id);
    owned_string
}

#[post("/<id>/delete")]
pub fn author_delete_post(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Author delete post".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a PUT, not a GET, but I am following the express tutorial
#[get("/<id>/update")]
pub fn author_update_get(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Author update get".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a PUT, not a POST, but I am following the express tutorial
#[post("/<id>/update")]
pub fn author_update_post(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Author update post".to_owned();
    owned_string.push_str(id);
    owned_string
}

#[get("/")]
pub fn author_list() -> &'static str {
    "NOT IMPLEMENTED: Author list"
}

#[get("/<id>")]
pub fn author_detail(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Author detail".to_owned();
    owned_string.push_str(id);
    owned_string
}
