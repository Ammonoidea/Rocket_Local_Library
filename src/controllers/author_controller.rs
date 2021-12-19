use rocket_dyn_templates::Template;
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
