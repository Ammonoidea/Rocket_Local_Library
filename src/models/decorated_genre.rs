use crate::models::genre::Genre;
use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DecoratedGenre {
    pub name: String,
    pub url: String,
}

impl DecoratedGenre {
    pub fn from_genre(genre: &Genre) -> DecoratedGenre {
        let mut url: String = "/catalog/genre/".to_string();
        url.push_str(&genre._id.to_hex());
        DecoratedGenre {
            name: genre.name.clone(),
            url: url,
        }
    }
}
