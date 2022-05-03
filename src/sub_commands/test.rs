pub fn run() {
    let paths = std::fs::read_dir("./test-cases/inputs/").unwrap();
    let time_stamp = chrono::offset::Local::now()
        .format("%H:%M:%S_%d-%m-%Y")
        .to_string();


    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg("make all")
        .status()
        .expect("Could not compile");

    assert!(status.success());

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
                "./test-cases/attempts/{}/{}",
                time_stamp, test_case_name
            ))
            .unwrap();

            let command = format!(
                "make run < ./test-cases/inputs/{0}/{0}.in > ./test-cases/attempts/{1}/{0}/{0}.out",
                test_case_name, time_stamp
            );

            let status = std::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect(&format!("Could not find test case number {}", test_case_name).to_string());

            let command = format!(
                "diff ./test-cases/attempts/{0}/{1}/{1}.out ./test-cases/outputs/{1}/{1}.out > ./test-cases/attempts/{0}/{1}/{1}.diff",
                time_stamp, test_case_name
            );
            println!("{}",command);

            std::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect(&format!("Could not find test case number {}", test_case_name).to_string());

            assert!(status.success());
        }
    }
}