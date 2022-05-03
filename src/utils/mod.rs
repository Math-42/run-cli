use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Project {
	pub name: String,
	pub course: String,
	pub exercise: String,
	pub submission_type: String,
	pub global: bool,
}

#[derive(Deserialize, Serialize)]
pub struct UserCredentials {
	pub user_email: String,
	pub user_password: String,
}
