#[get("/create")]
pub fn genre_create_get() -> &'static str {
    "NOT IMPLEMENTED: Genre create get"
}

#[post("/create")]
pub fn genre_create_post() -> &'static str {
    "NOT IMPLEMENTED: Genre create post"
}

// this should be a DELETE, not a GET, but I am following the express tutorial
#[get("/<id>/delete")]
pub fn genre_delete_get(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Genre delete get".to_owned();
    owned_string.push_str(id);
    owned_string
}

#[post("/<id>/delete")]
pub fn genre_delete_post(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Genre delete post".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a PUT, not a GET, but I am following the express tutorial
#[get("/<id>/update")]
pub fn genre_update_get(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Genre update get".to_owned();
    owned_string.push_str(id);
    owned_string
}

// this should be a PUT, not a POST, but I am following the express tutorial
#[post("/<id>/update")]
pub fn genre_update_post(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Genre update post".to_owned();
    owned_string.push_str(id);
    owned_string
}

#[get("/")]
pub fn genre_list() -> &'static str {
    "NOT IMPLEMENTED: Genre list"
}

#[get("/<id>")]
pub fn genre_detail(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Genre detail".to_owned();
    owned_string.push_str(id);
    owned_string
}
