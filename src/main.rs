mod cli;
mod run_codes;
mod sub_commands;
mod templates;
mod utils;
mod zip;

fn main() {
	let matches = cli::cli().get_matches();

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
		Some(("test", sub_matches)) => {
			sub_commands::test::run(sub_matches);
		}
		Some(("send", sub_matches)) => {
			sub_commands::send::run(sub_matches);
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
		_ => unreachable!(),
	};
}
