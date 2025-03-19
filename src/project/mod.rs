use std::{
    fs::{create_dir, File},
    io::Write,
    path::PathBuf,
    process::{self, Command, Stdio},
};

use colored::Colorize;

use crate::{DEFAULT_CLANGD, DEFAULT_CMAKELISTS, DEFAULT_GITIGNORE, DEFAULT_MAIN, DEFAULT_MAIN23};

pub mod build;
pub mod init;
pub mod new;
pub mod run;
pub mod setup;

pub enum CreationMode {
    New,
    Init,
    InitSameDir,
}

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum CppStandard {
    Cpp20,
    Cpp23,
}

impl ToString for CppStandard {
    fn to_string(&self) -> String {
        match self {
            CppStandard::Cpp20 => "cpp20".to_owned(),
            CppStandard::Cpp23 => "cpp23".to_owned(),
        }
    }
}

impl CppStandard {
    pub fn version(&self) -> String {
        match self {
            CppStandard::Cpp20 => "20".to_owned(),
            CppStandard::Cpp23 => "23".to_owned(),
        }
    }
}

pub struct ProjectDetails {
    pub name: String,
    pub path: PathBuf,
    pub standard: CppStandard,
    pub mode: CreationMode,
}

type ProjectResult = Result<ProjectDetails, String>;

pub fn create_all_files(details: ProjectDetails) -> ProjectResult {
    // Create CMakeLists.txt, .gitignore, .clangd
    let cmake_file_path = {
        let mut dir = details.path.clone();
        dir.push("CMakeLists.txt");
        dir
    };

    let cmake_file_contents = DEFAULT_CMAKELISTS
        .replace("{{PROJECT_NAME}}", &details.name)
        .replace("{{PROJECT_VERSION}}", &details.standard.version());
    let mut cmake_file = match File::create(cmake_file_path) {
        Ok(file) => file,
        Err(error) => return Err(format!("failed to create CMakeLists.txt file: {}", error)),
    };
    match cmake_file.write_all(cmake_file_contents.as_bytes()) {
        Ok(_) => success_print("wrote CMakeLists.txt file".to_owned()),
        Err(error) => {
            return Err(format!(
                "failed to write into CMakeLists.txt file: {}",
                error
            ))
        }
    }

    let gitignore_file_path = {
        let mut dir = details.path.clone();
        dir.push(".gitignore");
        dir
    };

    let mut gitignore_file = match File::create(gitignore_file_path) {
        Ok(file) => file,
        Err(error) => return Err(format!("failed to create .gitignore file: {}", error)),
    };
    match gitignore_file.write_all(DEFAULT_GITIGNORE.as_bytes()) {
        Ok(_) => success_print("wrote .gitignore file".to_owned()),
        Err(error) => return Err(format!("failed to write into .gitignore file: {}", error)),
    }

    let clangd_file_path = {
        let mut dir = details.path.clone();
        dir.push(".clangd");
        dir
    };

    let mut clangd_file = match File::create(clangd_file_path) {
        Ok(file) => file,
        Err(error) => return Err(format!("failed to create .clangd file: {}", error)),
    };
    match clangd_file.write_all(DEFAULT_CLANGD.as_bytes()) {
        Ok(_) => success_print("wrote .clangd file".to_owned()),
        Err(error) => return Err(format!("failed to write into .clangd file: {}", error)),
    }

    // Create src/ and src/main.cpp
    let src_dir_path = {
        let mut dir = details.path.clone();
        dir.push("src");
        dir
    };

    match create_dir(&src_dir_path) {
        Ok(_) => success_print("created src directory".to_owned()),
        Err(error) => return Err(format!("failed to create src directory: {}", error)),
    }

    let main_cpp_file_path = {
        let mut dir = src_dir_path.clone();
        dir.push("main.cpp");
        dir
    };

    let mut main_cpp_file = match File::create(main_cpp_file_path) {
        Ok(file) => file,
        Err(error) => return Err(format!("Failed to create main.cpp file: {}", error)),
    };

    let main_cpp_contents = match details.standard {
        CppStandard::Cpp20 => DEFAULT_MAIN,
        CppStandard::Cpp23 => DEFAULT_MAIN23,
    };

    match main_cpp_file.write_all(main_cpp_contents.as_bytes()) {
        Ok(_) => success_print("wrote main.cpp file".to_owned()),
        Err(error) => return Err(format!("failed to write into main.cpp file: {}", error)),
    }

    // Initialise git repo
    match Command::new("git")
        .arg("init")
        .current_dir(&details.path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(_) => success_print("initialised git repo in project".to_owned()),
        Err(error) => return Err(format!("failed to initialise git repo: {}", error)),
    }

    Ok(details)
}

pub fn success_print(message: String) {
    println!("{} {}", "✓".green(), message.black())
}

pub fn error_print(error: String) {
    eprintln!("{} {}", "Error:".red().bold(), error);
    process::exit(1);
}
