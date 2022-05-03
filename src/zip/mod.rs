use crate::utils::Project;
use std::{fs, io};

fn print_extracted_file(file: &zip::read::ZipFile) {
    println!("{} ({} bytes)", file.name(), file.size());
}

fn create_folder_if_not_exist(file_path: &String) {
    if !std::path::Path::new(&file_path).exists() {
        std::fs::create_dir_all(&file_path).unwrap();
    }
}

fn save_file(file: &mut zip::read::ZipFile, file_path: &String) {
    let mut output_file = fs::File::create(format!("{}/{}", file_path, file.name())).unwrap();
    io::copy(file, &mut output_file).unwrap();
}

fn split_file_name(file_name: &String) -> (String, String) {
    let file = std::path::Path::new(file_name);

    return (
        file.file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap()
            .to_string(),
        file.extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap()
            .to_string(),
    );
}

pub fn create_new_test_case_file(
    file_type: &String,
    file_path: &String,
    file: &mut zip::read::ZipFile,
) {
    let (name, _) = split_file_name(&file.name().to_string());
    let final_name = format!("{}/{}/{}", file_path, file_type, name).to_string();

    create_folder_if_not_exist(&final_name);

    save_file(file, &final_name);
}

pub fn unzip_test_cases(project: &Project) {
    let test_cases_path = format!("{}/{}", project.name, "test-cases");
    let zip_file_path = format!("{}/{}", test_cases_path, "test-cases.zip");

    let fname = std::path::Path::new(&zip_file_path);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        print_extracted_file(&file);

        if (*file.name()).ends_with(".in") {
            create_new_test_case_file(&"inputs".to_string(), &test_cases_path, &mut file);
        } else if (*file.name()).ends_with(".out") {
            create_new_test_case_file(&"outputs".to_string(), &test_cases_path, &mut file);
        } else {
            save_file(&mut file, &test_cases_path);
        }
    }
}
