use crate::run_codes;
use crate::utils;
use crate::sub_commands::zip;

pub fn run(args: &clap::ArgMatches) {
	let project = match utils::Project::read_configs() {
		Some(project) => project,
		None => {
			println!("This is not a valid project, run-cli config file not found!!");
			return;
		}
	};

	zip::zip_project().unwrap();

	let client = run_codes::init_client();
	utils::login(&client, args.is_present("local"));
	run_codes::submit_exercise(&client, &project);
}
