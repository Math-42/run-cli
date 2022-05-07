use crate::sub_commands::build;
use colored::Colorize;

pub struct TestOutput {
    pub error: bool,
    pub stdout: String,
    pub stderr: String,
    pub cpu_time: i32,
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

    let process = shell.arg("run").stdin(std::process::Stdio::from(file));

    let output = process
        .stdout(std::process::Stdio::piped())
        .output()
        .expect("Error running the test case");

    let test_error = !output.status.success();
    let test_stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let test_stderr = String::from_utf8_lossy(&output.stderr).to_string();

    return TestOutput {
        error: test_error,
        stdout: test_stdout,
        stderr: test_stderr,
        cpu_time: 2,
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

pub fn run(_args: &clap::ArgMatches) {
    println!("Building the project... \n");
    if build::build_project().is_err() {
        println!("Error building the project");
        return;
    }
    println!();

    let paths = std::fs::read_dir("./test-cases/inputs/").unwrap();
    let time_stamp = chrono::offset::Local::now()
        .format("%H:%M:%S_%d-%m-%Y")
        .to_string();

    println!("Starting the tests... \n");
    for path in paths {
        let dir_path = path.unwrap().path();
        let is_dir = dir_path.is_dir();

        if is_dir {
            let test_case_name = dir_path
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap()
                .to_string();

            std::fs::create_dir_all(format!(
                "./test-cases/local-attempts/{}/{}",
                time_stamp, test_case_name
            ))
            .unwrap();

            let test_output = run_test_case(&test_case_name);

            test_output.save(format!(
                "./test-cases/local-attempts/{}/{1}/{1}",
                time_stamp, test_case_name
            ));

            if test_output.error {
                println!("Test {}: Failed with error", test_case_name);
                println!("\n{}", test_output.stderr);
                continue;
            }

            let (diff_code, diff_stderr, diff_stdout) = diff(
                format!("./test-cases/outputs/{0}/{0}.out", test_case_name),
                format!(
                    "./test-cases/local-attempts/{1}/{0}/{0}.out",
                    test_case_name, time_stamp
                ),
            );

            match diff_code {
                Some(0) => println!(
                    "Test {}: {}",
                    test_case_name,
                    format!("Passed").bold().green()
                ),
                Some(1) => println!(
                    "Test {}: {}",
                    test_case_name,
                    format!("Failed").bold().red()
                ),
                Some(2) => println!(
                    "Test {}: Missing testing files\n\n {}",
                    test_case_name, diff_stderr
                ),
                Some(code) => panic!("Test {}: Unexpected exit code {}", test_case_name, code),
                None => (),
            }

            std::fs::write(
                format!(
                    "./test-cases/local-attempts/{0}/{1}/{1}.diff",
                    time_stamp, test_case_name
                ),
                match diff_code {
                    Some(2) => diff_stderr,
                    Some(0) => String::from("No differences founded"),
                    Some(_) => diff_stdout,
                    None => diff_stdout,
                },
            )
            .expect("failed to save test diff file");
        }
    }
}
