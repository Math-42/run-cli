mod cli;
mod run_codes;
mod sub_commands;
mod templates;
mod utils;
mod zip;

fn main() {
	let cli = cli::cli();
	let matches = cli.get_matches();
	match matches.subcommand() {
		Some(("init", sub_matches)) => {
			sub_commands::init::run(sub_matches);
		}
		Some(("fetch", sub_matches)) => {
			sub_commands::fetch::run(sub_matches);
		}
		Some(("build", sub_matches)) => {
			sub_commands::build::run(sub_matches);
		}
		Some(("run", sub_matches)) => {
			sub_commands::run::run(sub_matches);
		}
		Some(("zip", sub_matches)) => {
			sub_commands::zip::run(sub_matches);
		}
		Some(("test", sub_matches)) => {
			sub_commands::test::run(sub_matches);
		}
		Some(("submit", sub_matches)) => {
			sub_commands::submit::run(sub_matches);
		}
		Some(("credentials", sub_matches)) => {
			sub_commands::credentials::run(sub_matches);
		}
		Some(("update", sub_matches)) => {
			sub_commands::update::run(sub_matches);
		}
		Some(("subscribe", sub_matches)) => {
			sub_commands::subscribe::run(sub_matches);
		}
		Some(("import", sub_matches)) => {
			sub_commands::import::run(sub_matches);
		}
		_ => {
			println!("Unknown subcommand");
			cli::cli().print_long_help().unwrap();
		}
	};
}
