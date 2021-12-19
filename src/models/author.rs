// use chrono::prelude::*;
// use chrono::Duration;

use mongodb::bson::{doc, Bson, Document};

use crate::models::documentable::Persistable;

#[derive(Debug, PartialEq, Eq)]
pub struct Author {
    pub first_name: String,
    pub family_name: Option<String>,
    // pub date_of_birth: Option<NaiveDate>,
    // pub date_of_death: Option<NaiveDate>
}

impl Author {
    pub fn name(&self) -> String {
        let mut full_name = self.first_name.clone();
        full_name.push_str(" ");
        full_name.push_str(&(self.family_name.clone().unwrap_or_else(|| String::from(""))));
        return full_name;
    }

    // pub fn life_span(&self) -> Option<Duration>{
    // 	return self.date_of_death.and_then(|death|
    // 		self.date_of_birth.map(|birth| death.signed_duration_since(birth)))
    // }
}

impl Persistable for Author {
    fn to_document(&self) -> Document {
        doc! {
            "firstName": self.first_name.clone(),
            "familyName": self.family_name.clone().unwrap_or_else(|| String::from("")),
            // "dateOfBirth": self.date_of_birth.clone()
            // 	.map(|date| date.to_string()).unwrap_or_else(|| String::from("")),
            // "dateOfDeath": self.date_of_death.clone()
            // 	.map(|date| date.to_string()).unwrap_or_else(|| String::from("")),
        }
    }

    fn from_document(document: &Document) -> Self {
        Author {
            first_name: match document.get("firstName") {
                Some(&Bson::String(ref first_name)) => first_name.clone(),
                _ => panic!("Expected first name to be a string"),
            },
            family_name: match document.get("lastName") {
                Some(&Bson::String(ref last_name)) => Some(last_name.clone()),
                None => None,
                _ => panic!("Expected last name to be a string or None"),
            },
            // date_of_birth
        }
    }

    fn coll_name() -> String {
        unimplemented!()
    }
}
