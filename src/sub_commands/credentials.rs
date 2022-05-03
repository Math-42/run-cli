use crate::run_codes;
use crate::utils::{Project, UserCredentials};

pub fn run(args: &clap::ArgMatches) {
	let client = run_codes::init_client();

	if args.is_present("local") {
		match Project::read_configs() {
			Some(_) => {
				let credentials: UserCredentials = run_codes::login_loop(&client);
				credentials.save_credentials_locally();
			}
			None => {
				println!("This is not a valid project, run-cli config file not found!!");
			}
		}
	} else {
		let credentials = run_codes::login_loop(&client);
		credentials.save_credentials_globally();
	}
}
