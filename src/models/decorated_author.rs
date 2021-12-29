use crate::models::author::Author;
use bson::DateTime;
use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DecoratedAuthor {
    pub first_name: String,
    pub family_name: Option<String>,
    pub date_of_birth: Option<DateTime>,
    pub date_of_death: Option<DateTime>,
    pub name: String,
    pub url: String,
}

impl DecoratedAuthor {
    pub fn from_author(author: &Author) -> DecoratedAuthor {
        let mut url: String = "catalog/author/".to_string();
        url.push_str(&author._id.to_hex());
        DecoratedAuthor {
            first_name: author.first_name.clone(),
            family_name: author.family_name.clone(),
            date_of_birth: author.date_of_birth.clone(),
            date_of_death: author.date_of_death.clone(),
            name: author.name(),
            url: url,
        }
    }
}
