use std::{env, fs};

use crate::{create_all_files, CppStandard};

#[derive(clap::Args)]
pub struct NewArgs {
    #[arg(short, long, default_value_t = CppStandard::Cpp20)]
    pub standard: CppStandard,

    #[arg(value_name = "PATH")]
    pub project_path: String,
}

pub fn new_project(args: NewArgs) {
    let current_path = match env::current_dir() {
        Ok(directory) => directory,
        Err(error) => panic!("failed to get current directory: {}", error),
    };

    // Create project directory
    let project_path = {
        let mut dir = current_path.clone();
        dir.push(&args.project_path);
        dir
    };

    match fs::create_dir_all(&project_path) {
        Ok(_) => println!("created project directory"),
        Err(error) => panic!("failed to create directory: {}", error),
    }

    let project_name = match project_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => panic!("failed to get directory file name"),
    };

    create_all_files(&project_name, &project_path, args.standard);
}
