use rocket::serde::Serialize;
use rocket::State;
use rocket_dyn_templates::Template;

use crate::repositories::genre_collection::GenreCollection;
use crate::models::genre::Genre;
use crate::models::decorated_genre::DecoratedGenre;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct GenreTemplateContext<'r> {
    genres: &'r Vec<DecoratedGenre>,
}

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
pub fn genre_list(genre_coll: &State<GenreCollection>) -> Template {
    let genres: Vec<Genre> = genre_coll.list_genres();
    let mut decorated_genres: Vec<DecoratedGenre> = Vec::new();
    for genre in genres {
        let decorated_genre = DecoratedGenre::from_genre(&genre);
        decorated_genres.push(decorated_genre);
    }
    Template::render(
        "genre_list",
        &GenreTemplateContext {
            genres: &decorated_genres,
        },
    )
}

#[get("/<id>")]
pub fn genre_detail(id: &str) -> String {
    let mut owned_string: String = "NOT IMPLEMENTED: Genre detail".to_owned();
    owned_string.push_str(id);
    owned_string
}
