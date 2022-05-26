use crate::utils;

pub fn zip_project() -> Result<bool, std::io::Error> {
	let mut cmd = std::process::Command::new("make")
		.arg("zip")
		.stdout(std::process::Stdio::inherit())
		.stderr(std::process::Stdio::inherit())
		.spawn()?;

	let status = cmd.wait()?;

	if status.success() {
		return Ok(true);
	};

	Err(std::io::Error::new(
		std::io::ErrorKind::Other,
		"Error compressing the project",
	))
}

pub fn run(_args: &clap::ArgMatches) {
	if !utils::is_a_project() {
		return;
	}

	println!("Compressing project...");
	zip_project().unwrap();
}
