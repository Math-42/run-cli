use crate::utils::UserCredentials;
use colored::Colorize;
use reqwest::Url;
use scraper::{Html, Selector};
use std::io::Write;

pub struct Exercise {
	pub name: String,
	pub status: String,
	pub ratio: String,
	pub grade: String,
	pub dead_line: String,
	pub link: String,
	pub code: String,
}

pub struct Course {
	pub name: String,
	pub university: String,
	pub class: String,
	pub link: String,
	pub end_date: String,
}

pub fn init_client() -> reqwest::blocking::Client {
	return reqwest::blocking::Client::builder()
		.cookie_store(true)
		.build()
		.unwrap();
}

impl Exercise {
	pub fn formatted_option(&self) -> String {
		format!(
			"{:<32}{:<14}{:<8}{:<7}{:<19}",
			self.name, self.status, self.ratio, self.grade, self.dead_line
		)
	}
}

impl Course {
	pub fn formatted_option(&self) -> String {
		format!(
			"{:<5}{:<45}{:<45}{:<15}",
			self.university, self.name, self.class, self.end_date
		)
	}
}

fn get_nth_table_row_value(index: usize, element: scraper::ElementRef) -> String {
	return element
		.select(&Selector::parse("td").unwrap())
		.nth(index)
		.unwrap()
		.text()
		.collect();
}

pub fn read_user_credentials() -> UserCredentials {
	print!("Email: ");
	std::io::stdout().flush().unwrap();

	let mut email = String::new();
	std::io::stdin().read_line(&mut email).unwrap();
	email.pop();

	print!("Password: ");
	std::io::stdout().flush().unwrap();

	let password = rpassword::read_password().unwrap();

	return UserCredentials {
		user_email: email,
		user_password: password,
	};
}

pub fn login(client: &reqwest::blocking::Client, credentials: &UserCredentials) -> bool {
	client
		.post("https://run.codes")
		.form(&[
			("action", "/"),
			("method", "post"),
			("data[User][email]", &credentials.user_email),
			("data[User][password]", &credentials.user_password),
		])
		.send()
		.unwrap();

	let account_url = "https://run.codes/home";
	let response = client.get(account_url).send().unwrap();
	return response.url().eq(&Url::parse(account_url).unwrap());
}

pub fn login_loop(client: &reqwest::blocking::Client) -> UserCredentials {
	let mut logged = false;
	let mut valid_credentials = UserCredentials {
		user_email: "".to_string(),
		user_password: "".to_string(),
	};

	while logged == false {
		valid_credentials = read_user_credentials();

		logged = login(&client, &valid_credentials);

		if !logged {
			println!("Invalid user's credentials");
		}
	}

	return valid_credentials;
}

pub fn get_next_exercises(client: &reqwest::blocking::Client) {
	let response = client
		.get("https://run.codes/home")
		.send()
		.unwrap()
		.text()
		.unwrap();

	let parsed_html = Html::parse_document(&response);

	let open_courses_selector =
		&Selector::parse("table[class~=\"table-striped\"] > tbody > tr").unwrap();

	for element in parsed_html.select(&open_courses_selector) {
		let status = get_nth_table_row_value(1, element);
		let ratio = get_nth_table_row_value(2, element);
		let grade = get_nth_table_row_value(3, element);
		let dead_line = get_nth_table_row_value(4, element);

		let name_course = element
			.select(&Selector::parse("td").unwrap())
			.nth(0)
			.unwrap()
			.text()
			.collect::<Vec<_>>();

		let name = name_course[0];
		let course = name_course[1];

		let status_view: String = if status == "Finalizado" {
			status.bold().green().to_string()
		} else if status == "Incompleto" {
			status.bold().blue().to_string()
		} else {
			status.bold().red().to_string()
		};
		let (grade_view, ratio_view) = if grade == "10.00" {
			(
				grade.bold().green().to_string(),
				ratio.bold().green().to_string(),
			)
		} else if grade == "0.00" {
			(
				grade.bold().red().to_string(),
				ratio.bold().red().to_string(),
			)
		} else {
			(
				grade.bold().blue().to_string(),
				ratio.bold().blue().to_string(),
			)
		};

		println!(
			"{:40} {:<20} {:<25} {:<12} {:<12} {:<18}",
			course, name, status_view, ratio_view, grade_view, dead_line
		);
	}
}
pub fn get_all_exercises(client: &reqwest::blocking::Client, course: &Course) -> Vec<Exercise> {
	let mut current_courses: Vec<Exercise> = Vec::new();

	let response = client.get(&course.link).send().unwrap().text().unwrap();

	let parsed_html = Html::parse_document(&response);

	let open_courses_selector =
		&Selector::parse("table[class~=\"table-striped\"] > tbody > tr").unwrap();

	for element in parsed_html.select(&open_courses_selector) {
		let name = get_nth_table_row_value(1, element);
		let status = get_nth_table_row_value(2, element);
		let ratio = get_nth_table_row_value(3, element);
		let grade = get_nth_table_row_value(4, element);
		let dead_line = get_nth_table_row_value(5, element);

		let code: String = element
			.select(&Selector::parse("td").unwrap())
			.nth(6)
			.unwrap()
			.select(&Selector::parse("a").unwrap())
			.nth(0)
			.unwrap()
			.value()
			.attr("href")
			.unwrap()
			.to_string();

		current_courses.push(Exercise {
			name: name.trim_start().trim_end().to_string(),
			status: status.trim_start().trim_end().to_string(),
			link: "https://run.codes".to_string() + &code.trim_start().to_string(),
			dead_line: dead_line.trim_start().trim_end().to_string(),
			grade: grade.trim_start().trim_end().to_string(),
			ratio: ratio.trim_start().trim_end().to_string(),
			code: code
				.trim_start()
				.trim_end()
				.to_string()
				.split("/")
				.last()
				.unwrap()
				.to_string(),
		});
	}

	return current_courses;
}

pub fn get_all_courses(client: &reqwest::blocking::Client) -> Vec<Course> {
	let mut current_courses: Vec<Course> = Vec::new();

	let response = client
		.get("https://run.codes/Offerings/my")
		.send()
		.unwrap()
		.text()
		.unwrap();

	let parsed_html = Html::parse_document(&response);

	let open_courses_selector = &Selector::parse(
		"aside[class~=\"main-content\"]>:nth-child(2) table[class~=\"table-striped\"] > tbody > tr",
	)
	.unwrap();

	let closed_courses_selector = &Selector::parse(
		"aside[class~=\"main-content\"]>:nth-child(3) table[class~=\"table-striped\"] > tbody > tr",
	)
	.unwrap();

	for element in parsed_html.select(&open_courses_selector) {
		let university: String = get_nth_table_row_value(0, element);
		let name: String = get_nth_table_row_value(1, element);
		let class: String = get_nth_table_row_value(2, element);

		let code: String = element
			.select(&Selector::parse("td").unwrap())
			.nth(5)
			.unwrap()
			.select(&Selector::parse("a").unwrap())
			.nth(0)
			.unwrap()
			.value()
			.attr("href")
			.unwrap()
			.to_string();

		current_courses.push(Course {
			name: name.trim_start().trim_end().to_string(),
			link: "https://run.codes".to_string() + &code.trim_start().to_string(),
			university: university.trim_start().trim_end().to_string(),
			class: class.trim_start().trim_end().to_string(),
			end_date: "enrolled".to_string(),
		});
	}

	for element in parsed_html.select(&closed_courses_selector) {
		let university: String = get_nth_table_row_value(0, element);
		let name: String = get_nth_table_row_value(1, element);
		let class: String = get_nth_table_row_value(2, element);
		let end_date: String = get_nth_table_row_value(5, element);

		let code: String = element
			.select(&Selector::parse("td").unwrap())
			.nth(6)
			.unwrap()
			.select(&Selector::parse("a").unwrap())
			.nth(0)
			.unwrap()
			.value()
			.attr("href")
			.unwrap()
			.to_string();

		current_courses.push(Course {
			name: name.trim_start().trim_end().to_string(),
			link: "https://run.codes".to_string() + &code.trim_start().to_string(),
			university: university.trim_start().trim_end().to_string(),
			class: class.trim_start().trim_end().to_string(),
			end_date: end_date.trim_start().trim_end().to_string(),
		});
	}

	return current_courses;
}

pub fn download_exercise_files(client: &reqwest::blocking::Client, exercise: &Exercise) {
	let response = client.get(&exercise.link).send().unwrap().text().unwrap();
	let parsed_html = Html::parse_document(&response);
	let exercise_files_selector = &Selector::parse("div[class~=\"panel-body\"] li > a").unwrap();

	let mut files_links_and_links: Vec<(String, String)> = Vec::new();

	for document_element_link in parsed_html.select(&exercise_files_selector) {
		files_links_and_links.push((
			format!(
				"https://run.codes{}",
				document_element_link
					.value()
					.attr("href")
					.unwrap()
					.to_string()
			),
			document_element_link.text().collect(),
		));
	}

	for (link, name) in files_links_and_links {
		println!("{}", link);

		let mut response = client.get(link).send().unwrap();

		let mut out = std::fs::File::create(format!("{}/docs/{}", &exercise.name, name))
			.expect("failed to create file");

		std::io::copy(&mut response, &mut out).expect("failed to copy content");
	}
}

pub fn download_test_cases(client: &reqwest::blocking::Client, exercise: &Exercise) {
	let test_cases_download_link = format!(
		"https://run.codes/Exercises/downloadCases/{}",
		exercise.code
	);
	println!("{}", test_cases_download_link);

	let mut response = client.get(test_cases_download_link).send().unwrap();
	let mut out = std::fs::File::create(format!(
		"{}/test-cases/{}",
		&exercise.name, "test-cases.zip"
	))
	.expect("failed to save test-cases.zip file");

	std::io::copy(&mut response, &mut out).expect("failed to save test-cases.zip file");
}

pub fn get_all_submission_options(
	client: &reqwest::blocking::Client,
	exercise: &Exercise,
) -> Vec<String> {
	let response = client.get(&exercise.link).send().unwrap().text().unwrap();
	let parsed_html = Html::parse_document(&response);
	let submission_selector = &Selector::parse("li > span").unwrap();

	return parsed_html
		.select(&submission_selector)
		.map(|e| e.text().collect())
		.collect();
}

pub fn get_exercise_description(client: &reqwest::blocking::Client, exercise: &Exercise) {
	let response = client.get(&exercise.link).send().unwrap().text().unwrap();
	let parsed_html = Html::parse_document(&response);

	let exercise_body_selector = &Selector::parse("div[class=\"panel-body\"]").unwrap();
	let advisor_files_selector = &Selector::parse("div[class~=\"panel-body\"] small>p").unwrap();

	let description_element = parsed_html.select(&exercise_body_selector).nth(0).unwrap();
	let advisor_element = parsed_html.select(&advisor_files_selector).nth(0).unwrap();

	let description_wrong_links_text = format!(
		"# {}\n\n{}\n{}.\n",
		exercise.name,
		html2md::parse_html(&description_element.html()),
		advisor_element.text().collect::<String>()
	);

	let description_text = str::replace(
		&description_wrong_links_text,
		"/Exercise",
		"https://run.codes/Exercise",
	);

	std::fs::write(
		format!("{}/docs/{}", &exercise.name, "description.md"),
		&description_text,
	)
	.unwrap();
}
