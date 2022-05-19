use crate::run_codes;
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

impl Project {
	pub fn save_configs(&self) {
		let toml = toml::to_string(&self).unwrap();
		std::fs::File::create(format!("./{}/.run-cli/run-cli.toml", self.name))
			.expect("failed to create run-cli config file");

		std::fs::write(
			format!("./{}/.run-cli/run-cli.toml", self.name),
			&toml.to_string(),
		)
		.expect("failed to save run-cli config file");
	}

	pub fn read_configs() -> Option<Project> {
		match std::fs::read_to_string("./run-cli/run-cli.toml") {
			Ok(string) => {
				let toml: Project = toml::from_str(&string).unwrap();
				return Some(toml);
			}
			Err(_) => return None,
		};
	}
}

impl UserCredentials {
	fn save_credentials(&self, path: String) {
		let toml = toml::to_string(&self).unwrap();
		std::fs::File::create(&path).expect("failed to create run-cli credentials file");

		std::fs::write(path, &toml.to_string()).expect("failed to save run-cli credentials file");
	}

	pub fn save_credentials_globally(&self) {
		std::fs::create_dir_all(format!(
			"{}/.config/.run-cli/",
			home::home_dir().unwrap().display()
		))
		.unwrap();
		self.save_credentials(format!(
			"{}/.config/.run-cli/run-cli-credentials.toml",
			home::home_dir().unwrap().display()
		));
	}

	pub fn save_credentials_locally(&self) {
		self.save_credentials("./.run-cli/run-cli-credentials.toml".to_string());
	}

	fn read_credentials(path: String) -> Option<UserCredentials> {
		match std::fs::read_to_string(path) {
			Ok(string) => {
				let toml: UserCredentials = toml::from_str(&string).unwrap();
				return Some(toml);
			}
			Err(_) => return None,
		};
	}

	pub fn read_credentials_globally() -> Option<UserCredentials> {
		return UserCredentials::read_credentials(format!(
			"{}/.config/.run-cli/run-cli-credentials.toml",
			home::home_dir().unwrap().display()
		));
	}

	pub fn read_credentials_locally(project: Project) -> Option<UserCredentials> {
		return UserCredentials::read_credentials(format!(
			"./{}/.run-cli/run-cli-credentials.toml",
			project.name
		));
	}
}

pub fn login(client: &reqwest::blocking::Client, local: bool) -> UserCredentials {
	let mut user_credentials = UserCredentials {
		user_email: "".to_string(),
		user_password: "".to_string(),
	};

	if local {
		run_codes::login_loop(&client);
	} else {
		match UserCredentials::read_credentials_globally() {
			Some(credentials) => {
				if !run_codes::login(&client, &credentials) {
					println!("No valid credentials founded!!");
					println!("Please insert your credentials");
					user_credentials = run_codes::login_loop(&client);
					user_credentials.save_credentials_globally();
				}
			}
			None => {
				println!("No valid credentials founded!!");
				println!("Please insert your credentials");
				user_credentials = run_codes::login_loop(&client);
				user_credentials.save_credentials_globally();
			}
		}
	}
	return user_credentials;
}

pub fn is_a_project() -> bool {
	match Project::read_configs() {
		Some(_) => true,
		None => {
			println!("This is not a valid project, run-cli config file not found!!");
			return false;
		}
	}
}
