use std::{env, fs::read_dir};

use crate::{create_all_files, CppStandard};

#[derive(clap::Args)]
pub struct InitArgs {
    #[arg(short, long, default_value_t = CppStandard::Cpp20)]
    pub standard: CppStandard,

    #[arg(value_name = "PATH")]
    pub project_path: Option<String>,
}

pub fn init_project(args: InitArgs) -> Result<(), String> {
    let current_path = match env::current_dir() {
        Ok(directory) => directory,
        Err(error) => return Err(format!("failed to get current directory: {}", error)),
    };

    let project_path = match args.project_path {
        Some(project_path) => {
            let mut dir = current_path.clone();
            dir.push(&project_path);
            dir
        }
        None => current_path,
    };

    if !project_path.exists() {
        return Err(format!("specified path does not exist"));
    } else if !project_path.is_dir() {
        return Err(format!("specified path is not a directory"));
    } else if let Ok(_) = read_dir(&project_path) {
        return Err(format!("specified path is not empty"));
    }

    let project_name = match project_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return Err(format!("failed to get directory file name")),
    };

    create_all_files(&project_name, &project_path, args.standard)
}
