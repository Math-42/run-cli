use crate::run_codes;
use crate::templates;
use crate::utils;
use crate::utils::{Project, UserCredentials};
use crate::zip;
use dialoguer::{theme::ColorfulTheme, Select};
use git2::Repository;

fn create_project_structure(project: &Project) {
	std::fs::create_dir_all(format!("{}/test-cases/{}", project.name, "inputs")).unwrap();
	std::fs::create_dir_all(format!("{}/test-cases/{}", project.name, "outputs")).unwrap();
	std::fs::create_dir_all(format!("{}/test-cases/{}", project.name, "local-attempts")).unwrap();

	std::fs::create_dir_all(format!("{}/{}", project.name, "docs")).unwrap();

	std::fs::create_dir_all(format!("{}/{}", project.name, ".run-cli")).unwrap();
	std::fs::File::create(format!("{}/.run-cli/run-cli.toml", project.name)).unwrap();
	std::fs::File::create(format!(
		"{}/.run-cli/{}",
		project.name, "run-cli-credentials.toml"
	))
	.unwrap();

	Repository::init(format!("./{}", project.name)).unwrap();
}

pub fn run(args: &clap::ArgMatches) {
	println!("Initializing project...");

	let client = run_codes::init_client();
	let user_credentials: UserCredentials = utils::login(&client, args.is_present("local"));
	println!("Logged with success!");

	let courses = run_codes::get_all_courses(&client);
	let courses_options: Vec<String> = courses.iter().map(|e| e.formatted_option()).collect();
	let selected_course_idx = Select::with_theme(&ColorfulTheme::default())
		.with_prompt("Select one course:")
		.default(0)
		.items(&courses_options)
		.interact()
		.unwrap();

	let selected_course = &courses[selected_course_idx];

	let exercises = run_codes::get_all_exercises(&client, &selected_course);
	let exercises_options: Vec<String> = exercises.iter().map(|e| e.formatted_option()).collect();
	let selected_exercise_idx = Select::with_theme(&ColorfulTheme::default())
		.with_prompt("Select one exercise:")
		.default(0)
		.items(&exercises_options)
		.interact()
		.unwrap();

	let selected_exercise = &exercises[selected_exercise_idx];

	let submission_options = run_codes::get_all_submission_options(&client, selected_exercise);
	let selected_submission_idx = Select::with_theme(&ColorfulTheme::default())
		.with_prompt("Select one submission type:")
		.default(0)
		.items(&submission_options)
		.interact()
		.unwrap();

	let selected_submission_type = &submission_options[selected_submission_idx];

	let project = Project {
		name: selected_exercise.name.to_string(),
		submission_type: selected_submission_type.to_string(),
		course: selected_course.name.to_string(),
		exercise: selected_exercise.link.to_string(),
		global: false,
	};

	println!("Creating project structure");
	create_project_structure(&project);
	templates::build_struct(&project);

	println!("Downloading exercise files...");
	run_codes::download_exercise_files(&client, &selected_exercise);

	println!("Downloading exercise description...");
	run_codes::get_exercise_description(&client, &selected_exercise);

	println!("Downloading test cases...");
	run_codes::download_test_cases(&client, &selected_exercise);

	println!("Extracting test cases...");
	zip::unzip_test_cases(&project);

	project.save_configs();

	if args.is_present("local") {
		user_credentials.save_credentials_locally();
	}
}
