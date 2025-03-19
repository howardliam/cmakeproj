use std::{env, fs::read_dir};

use super::{CppStandard, CreationMode, ProjectDetails, ProjectResult};

#[derive(clap::Args)]
pub struct InitArgs {
    #[arg(short, long, default_value_t = CppStandard::Cpp20)]
    pub standard: CppStandard,

    #[arg(value_name = "PATH")]
    pub project_path: Option<String>,
}

pub fn init_project(args: InitArgs) -> ProjectResult {
    let current_path = match env::current_dir() {
        Ok(directory) => directory,
        Err(error) => return Err(format!("failed to get current directory: {}", error)),
    };

    let (project_path, mode) = match args.project_path {
        Some(project_path) => {
            let mut dir = current_path.clone();
            dir.push(&project_path);
            (dir, CreationMode::Init)
        }
        None => (current_path, CreationMode::InitSameDir),
    };

    if !project_path.exists() {
        return Err("specified path does not exist".to_owned());
    } else if !project_path.is_dir() {
        return Err("specified path is not a directory".to_owned());
    } else if let Ok(dir) = read_dir(&project_path) {
        if dir.count() > 0 {
            return Err("specified path is not empty".to_owned());
        }
    }

    let project_name = match project_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return Err("failed to get directory file name".to_owned()),
    };

    Ok(ProjectDetails {
        name: project_name,
        path: project_path,
        standard: args.standard,
        mode,
    })
}
