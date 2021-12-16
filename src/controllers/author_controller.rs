use rocket_contrib::templates::Template;
use crate::models::author::Author;
use std::collections::HashMap;

#[get("/")]
pub fn author_list() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", context)
}

pub fn author_detail() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", context)
}

pub fn author_create_post() -> Template {
    let context = HashMap::<String, String>::new();                            
    Template::render("index", context)
}
