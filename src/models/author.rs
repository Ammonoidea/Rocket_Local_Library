use chrono::prelude::*;
use chrono::Duration;

#[derive(Debug, PartialEq, Eq)]
pub struct Author {
	pub first_name: String,
	pub family_name: Option<String>,
	pub date_of_birth: Option<NaiveDate>,
	pub date_of_death: Option<NaiveDate>
}

impl Author {
	fn name(&self) -> String {
		let mut full_name = self.first_name.clone();
		full_name.push_str(" ");
		full_name.push_str(&(self.family_name.clone()
			.unwrap_or_else(|| String::from(""))));
		return full_name;
	}

	fn life_span(&self) -> Option<Duration>{
		return self.date_of_death.and_then(|death|
			self.date_of_birth.map(|birth| death.signed_duration_since(birth)))
	}
}
