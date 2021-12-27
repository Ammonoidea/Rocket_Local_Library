use serde::Deserialize;
use rocket::serde::Serialize;

use crate::models::author::Author;


#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DecoratedAuthor {
    pub first_name: String,
    pub family_name: Option<String>,
    pub date_of_birth: Option<String>,
    pub name: String,
    pub url: String,
}

impl DecoratedAuthor {
    pub fn from_author(author: Author) -> DecoratedAuthor {
        let mut url: String = "catalog/book".to_string();
        url.push_str(&author.id);

        DecoratedAuthor {
            first_name: author.first_name.clone(),
            family_name: author.family_name.clone(),
            date_of_birth: author.date_of_birth.clone(),
            name: author.name(),
            url: url,
        }
    }
}