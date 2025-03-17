use std::{
    fs::{create_dir, File},
    io::Write,
    path::PathBuf,
    process::{self, Command, Stdio},
};

use clap::Parser;
use colored::Colorize;
use init::{init_project, InitArgs};
use new::{new_project, NewArgs};

mod init;
mod new;

pub const DEFAULT_CONFIG: &str = include_str!("../templates/CMakeLists.txt");
pub const DEFAULT_GITIGNORE: &str = include_str!("../templates/gitignore");
pub const DEFAULT_MAIN: &str = include_str!("../templates/main.cpp");

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

#[derive(clap::Parser)]
enum CMakeProjCli {
    #[command(name = "new")]
    New(NewArgs),

    #[command(name = "init")]
    Init(InitArgs),
}

fn main() {
    let result = match CMakeProjCli::parse() {
        CMakeProjCli::New(args) => new_project(args),
        CMakeProjCli::Init(args) => init_project(args),
    };

    match result {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{} {}", "Error:".red().bold(), error);
            process::exit(1);
        }
    }
}

pub fn create_all_files(
    project_name: &String,
    project_path: &PathBuf,
    standard: CppStandard,
) -> Result<(), String> {
    // Create CMakeLists.txt, .gitignore
    let cmake_file_path = {
        let mut dir = project_path.clone();
        dir.push("CMakeLists.txt");
        dir
    };

    let cmake_file_contents = DEFAULT_CONFIG
        .replace("{{PROJECT_NAME}}", &project_name)
        .replace("{{PROJECT_VERSION}}", &standard.version());
    let mut cmake_file = match File::create(cmake_file_path) {
        Ok(file) => file,
        Err(error) => return Err(format!("failed to create CMakeLists.txt file: {}", error)),
    };
    match cmake_file.write_all(cmake_file_contents.as_bytes()) {
        Ok(_) => println!("{} {}", "✓".green(), "wrote CMakeLists.txt file".black()),
        Err(error) => {
            return Err(format!(
                "failed to write into CMakeLists.txt file: {}",
                error
            ))
        }
    }

    let gitignore_file_path = {
        let mut dir = project_path.clone();
        dir.push(".gitignore");
        dir
    };

    let mut gitignore_file = match File::create(gitignore_file_path) {
        Ok(file) => file,
        Err(error) => return Err(format!("failed to create .gitignore file: {}", error)),
    };
    match gitignore_file.write_all(DEFAULT_GITIGNORE.as_bytes()) {
        Ok(_) => println!("{} {}", "✓".green(), "wrote .gitignore file".black()),
        Err(error) => return Err(format!("failed to write into .gitignore file: {}", error)),
    }

    // Create src/ and src/main.cpp
    let src_dir_path = {
        let mut dir = project_path.clone();
        dir.push("src");
        dir
    };

    match create_dir(&src_dir_path) {
        Ok(_) => println!("{} {}", "✓".green(), "created src directory".black()),
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
    match main_cpp_file.write_all(DEFAULT_MAIN.as_bytes()) {
        Ok(_) => println!("{} {}", "✓".green(), "wrote main.cpp file".black()),
        Err(error) => return Err(format!("failed to write into main.cpp file: {}", error)),
    }

    // Initialise git repo
    match Command::new("git")
        .arg("init")
        .current_dir(&project_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(_) => println!(
            "{} {}",
            "✓".green(),
            "initialised git repo in project".black()
        ),
        Err(error) => return Err(format!("failed to initialise git repo: {}", error)),
    }

    println!();
    println!("Project `{}` created", project_name);
    println!();
    println!("cd {}", project_name);
    println!("cmake -B build -G 'Ninja'");
    println!("cmake --build build && ./build/{}", project_name);

    Ok(())
}
