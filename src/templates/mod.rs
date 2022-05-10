use crate::utils::Project;
use colored::Colorize;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Templates;

pub fn get_submission_files(submission_type: &String) -> Vec<String> {
	let mut submission_type_files: Vec<String> = Vec::new();

	for file in Templates::iter() {
		let file_type = file.split("/").next().unwrap_or("");

		if file_type == submission_type {
			submission_type_files.push(String::from(file));
		}
	}

	return submission_type_files;
}

pub fn build_struct(project: &Project) {
	let mut submission_type_files = get_submission_files(&project.submission_type);
	let mut submission_type = project.submission_type.to_string();


	if submission_type_files.len() == 0 {
		println!(
			"{} is not supported yet, using a generic ZIP Makefile instead",
			project.submission_type.to_string().bold().red()
		);
		submission_type_files = get_submission_files(&String::from("ZIP Makefile"));
		submission_type = "ZIP Makefile".to_string();
	}

	for file_path in submission_type_files {
		let file = Templates::get(&file_path).unwrap();

		let path_without_type = file_path
			.strip_prefix(&format!("{}/", submission_type))
			.unwrap();
		let real_path = String::from(format!("{}/{}", project.name, path_without_type));

		let path = std::path::Path::new(&real_path);
		let prefix = path.parent().unwrap();
		std::fs::create_dir_all(prefix).unwrap();

		match std::fs::write(path, &std::str::from_utf8(file.data.as_ref()).unwrap()) {
			Ok(_) => println!("writing: {}...", path.display()),
			Err(err) => println!("Error writing: {} \n {}", path.display(), err),
		}
	}
}
