use bson::oid::ObjectId;
use bson::DateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Author {
    pub _id: ObjectId,
    pub first_name: String,
    pub family_name: Option<String>,
    pub date_of_birth: Option<DateTime>,
    pub date_of_death: Option<DateTime>,
}

impl Author {
    pub fn name(&self) -> String {
        match &self.family_name {
            Some(family) => {
                let mut full_name = family.clone();
                full_name.push_str(", ");
                full_name.push_str(&self.first_name);
                full_name
            }
            None => self.first_name.clone(),
        }
    }

    // pub fn life_span(&self) -> Option<Duration>{
    // 	return self.date_of_death.and_then(|death|
    // 		self.date_of_birth.map(|birth| death.signed_duration_since(birth)))
    // }
}
