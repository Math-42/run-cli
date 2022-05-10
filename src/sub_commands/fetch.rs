use crate::run_codes;
use crate::utils;

pub fn run(args: &clap::ArgMatches) {
	let client = run_codes::init_client();
	utils::login(&client, args.is_present("login"));
	println!("Logged with success!");
	run_codes::get_next_exercises(&client);
}
