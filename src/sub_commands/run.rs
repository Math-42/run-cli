use crate::sub_commands::build;

pub fn run_project() -> Result<bool, std::io::Error> {
	let mut cmd = std::process::Command::new("make")
		.arg("run")
		.stdout(std::process::Stdio::inherit())
		.stderr(std::process::Stdio::inherit())
		.spawn()?;

	let status = cmd.wait()?;

	if status.success() {
		return Ok(true);
	};

	Err(std::io::Error::new(
		std::io::ErrorKind::Other,
		"Error running the project",
	))
}

pub fn run(_args: &clap::ArgMatches) {
	match build::build_project() {
		Ok(_) => println!("Project builded with success!"),
		Err(_) => {
			println!("Error building the project");
			return;
		}
	}
	
	match run_project() {
		Ok(_) => println!("running..."),
		Err(_) => println!("Error running the project"),
	}
}
