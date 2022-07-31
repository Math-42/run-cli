use crate::sub_commands::build;
use crate::utils;
use colored::Colorize;

pub struct TestOutput {
	pub error: bool,
	pub stdout: String,
	pub stderr: String,
	pub cpu_time: u128,
}

impl TestOutput {
	pub fn save(&self, path: String) {
		std::fs::write(format!("{}.out", &path), &self.stdout)
			.expect("failed to save test output file");

		if self.error {
			std::fs::write(format!("{}.err", &path), &self.stderr)
				.expect("failed to save test error file");
		}
	}
}

pub fn run_test_case(test_case_name: &String) -> TestOutput {
	let file =
		std::fs::File::open(format!("./test-cases/inputs/{0}/{0}.in", test_case_name)).unwrap();

	let mut shell = std::process::Command::new("make");

	let start = std::time::Instant::now();
	let process = shell.arg("run").stdin(std::process::Stdio::from(file));

	let output = process
		.stdout(std::process::Stdio::piped())
		.output()
		.expect("Error running the test case");

	let test_error = !output.status.success();
	let cpu_time = start.elapsed();

	let test_stdout = String::from_utf8_lossy(&output.stdout).to_string();
	let test_stderr = String::from_utf8_lossy(&output.stderr).to_string();

	return TestOutput {
		error: test_error,
		stdout: test_stdout,
		stderr: test_stderr,
		cpu_time: cpu_time.as_millis(),
	};
}

pub fn diff(
	output_base_file_path: String,
	output_test_file_path: String,
) -> (Option<i32>, String, String) {
	let mut shell = std::process::Command::new("diff");

	let process = shell.arg(output_base_file_path).arg(output_test_file_path);

	let output = process
		.stdout(std::process::Stdio::piped())
		.output()
		.expect("Error comparing the test case output");

	return (
		output.status.code(),
		String::from_utf8_lossy(&output.stderr).to_string(),
		String::from_utf8_lossy(&output.stdout).to_string(),
	);
}

pub fn save_diff(
	diff_stdout: &String,
	diff_stderr: &String,
	diff_code: &Option<i32>,
	time_stamp: &String,
	test_case_name: &String,
) {
	std::fs::write(
		format!(
			"./test-cases/local-attempts/{0}/{1}/{1}.diff",
			time_stamp, test_case_name
		),
		match diff_code {
			Some(2) => diff_stderr,
			Some(0) => "No differences found",
			Some(_) => diff_stdout,
			None => diff_stdout,
		},
	)
	.expect("failed to save test diff file");
}

pub fn run_single_test(test_case_name: &String) -> Result<Option<i32>, std::io::Error> {
	let test_output = run_test_case(&test_case_name);
	let time_stamp = chrono::offset::Local::now()
		.format("%d-%m-%Y_%H:%M:%S")
		.to_string();

	std::fs::create_dir_all(format!(
		"./test-cases/local-attempts/{}/{}",
		&time_stamp, test_case_name
	))
	.unwrap();

	test_output.save(format!(
		"./test-cases/local-attempts/{}/{1}/{1}",
		time_stamp, test_case_name
	));

	if test_output.error {
		println!("Test {}: {}", test_case_name, "Failed".bold().red());
		println!("\n{}", test_output.stderr);
		return Err(std::io::Error::new(
				std::io::ErrorKind::Other, "Test Output Failed",));
	}

	let (diff_code, diff_stderr, diff_stdout) = diff(
		format!("./test-cases/outputs/{0}/{0}.out", test_case_name),
		format!("./test-cases/local-attempts/{1}/{0}/{0}.out",
			test_case_name, time_stamp
		),
	);

	match diff_code {
		Some(0) => println!("Test {}: {}, {} ms",
			test_case_name,
			"Passed".bold().green(),
			test_output.cpu_time
		),
		Some(1) => println!("Test {}: {}", test_case_name, "Failed".bold().red()),
		Some(2) => println!(
			"Test {}: Missing test files\n\n {}",
			test_case_name, diff_stderr
		),
		Some(code) => panic!("Test {}: Unexpected exit code {}", test_case_name, code),
		None => (),
	}

	save_diff(
		&diff_stdout,
		&diff_stderr,
		&diff_code,
		&time_stamp,
		&test_case_name,
	);

	Ok(diff_code)
}

pub fn run_all_tests() {
	let mut errors_counter = 0;
	let mut passed_counter = 0;
	let mut issues_counter = 0;
	let mut failed_counter = 0;

	let paths = std::fs::read_dir("./test-cases/inputs/").unwrap();
	let time_stamp = chrono::offset::Local::now()
		.format("%d-%m-%Y_%H:%M:%S")
		.to_string();

	println!("Starting tests...\n");

	for path in paths {
		let dir_path = path.unwrap().path();

		if !dir_path.is_dir() {
			continue;
		}

		let test_case_name = dir_path
			.file_stem()
			.and_then(std::ffi::OsStr::to_str)
			.unwrap()
			.to_string();

		std::fs::create_dir_all(format!(
			"./test-cases/local-attempts/{}/{}",
			&time_stamp, test_case_name
		))
		.unwrap();

		let diff_code = match run_single_test(&test_case_name) {
			Ok(diff_code) => diff_code,
			Err(_error) => {
				errors_counter += 1;
				continue;
			},
		};

		match diff_code {
			Some(0) => passed_counter += 1,
			Some(1) => failed_counter += 1,
			Some(_) => issues_counter += 1,
			None => issues_counter += 1,
		}
	}
	println!(
		"\n{} Passed, {} Failed, {} erros and {} issue(s)",
		passed_counter.to_string().bold().green(),
		failed_counter.to_string().bold().red(),
		errors_counter.to_string().bold().red(),
		issues_counter
	);
}

pub fn run(args: &clap::ArgMatches) {
	if !utils::is_a_project() {
		return;
	};

	println!("Building project...");
	if build::build_project().is_err() {
		println!("Error building the project");
		return;
	}

	if args.is_present("number") {
		match run_single_test(&args.value_of("number").unwrap().to_string()) {
			Ok(diff_code) => diff_code,
			Err(_error) => {
				println!("{}", format!("Error running the test case").bold().red());
				return;
			},
		};
	} else {
		run_all_tests();
	}
}
