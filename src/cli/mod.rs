use clap::{arg, command, Command};

pub fn cli() -> Command<'static> {
	return command!()
		.subcommand_required(true)
		.arg_required_else_help(true)
		.allow_external_subcommands(true)
		.allow_invalid_utf8_for_external_subcommands(true)
		.subcommand(
			Command::new("init")
				.about("Start a new project")
				.arg(
					arg!(-n --name <PROJECT_NAME> "Set a custom project name")
						.required(false)
						.allow_invalid_utf8(true),
				)
				.arg(arg!(-l --local "Set the user credentials locally")),
		)
		.subcommand(
			Command::new("fetch")
				.about("Show the next exercises")
				.arg(arg!(-l --login "Login with custom credentials")),
		)
		.subcommand(Command::new("build").about("Compile the project"))
		.subcommand(Command::new("run").about("Runs the project"))
		.subcommand(
			Command::new("test").about("Run all test cases")
			.arg(
				arg!(-n --number <TEST_CASE_NUMBER> "Specify witch test run")
					.required(false)
			)
			.arg(arg!(-v --verbose "Print the expected output and the output")
				.required(false)
			),
		)
		.subcommand(
			Command::new("submit")
				.about("Submit the current project to run.codes")
				.arg(arg!(-l --local "Use local credentials").required(false)),
		)
		.subcommand(
			Command::new("credentials")
				.about("Set the user credentials")
				.arg(arg!(-l --local "Change only the local credentials").required(false)),
		)
		.subcommand(Command::new("secret").about("Get the closed test cases"))
		.subcommand(Command::new("zip").about("zip your main and the src folder"))
		.subcommand(Command::new("update").about("Update the data of the exercise"))
		.subcommand(
			Command::new("subscribe")
				.about("Enroll to a new class")
				.arg(arg!(-c --class_code <CLASS_CODE> "Code of the new class").required(true)),
		)
		.subcommand(
			Command::new("import")
				.about("Import a project from a git repo or from run codes")
				.arg(arg!(-r --run_codes <RUN_CODES_LINK> "Code of the new class").required(true))
				.arg(arg!(-r --git <GIT_REPO_LINK> "Code of the new class").required(true)),
		);
}
