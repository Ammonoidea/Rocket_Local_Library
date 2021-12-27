use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Author {
    pub id: String,
    pub first_name: String,
    pub family_name: Option<String>,
    pub date_of_birth: Option<String>,
    // pub date_of_death: Option<NaiveDate>
}

impl Author {
    pub fn name(&self) -> String {
        let mut full_name = self.first_name.clone();
        full_name.push_str(" ");
        full_name.push_str(&(self.family_name.clone().unwrap_or_else(|| String::from(""))));
        full_name
    }

    // pub fn life_span(&self) -> Option<Duration>{
    // 	return self.date_of_death.and_then(|death|
    // 		self.date_of_birth.map(|birth| death.signed_duration_since(birth)))
    // }
}
