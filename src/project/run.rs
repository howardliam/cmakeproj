use std::{env::current_dir, fs::read_dir, path::PathBuf, process::Command};

use is_executable::IsExecutable;

use super::error_print;

#[derive(clap::Args)]
pub struct RunArgs {
    #[arg(short, long)]
    pub build_dir: Option<String>,
}

pub fn run_project(args: RunArgs) {
    let current_path = match current_dir() {
        Ok(directory) => directory,
        Err(error) => return error_print(format!("failed to get current directory: {}", error)),
    };

    let build_dir = match args.build_dir {
        Some(build_dir) => build_dir,
        None => "build".to_owned(),
    };

    let build_path = {
        let mut dir = current_path.clone();
        dir.push(&build_dir);
        dir
    };

    if let Ok(dir) = read_dir(&build_path) {
        let mut cmakecache_found = false;
        for item in dir {
            if let Ok(item) = item {
                cmakecache_found = item.file_name().to_string_lossy() == "CMakeCache.txt";
                if cmakecache_found {
                    if let Ok(file_type) = item.file_type() {
                        if file_type.is_dir() {
                            return error_print("not a suitable build directory".to_owned());
                        }
                    }
                    break;
                }
            }
        }
        if !cmakecache_found {
            return error_print("not a suitable build directory".to_owned());
        }
    }

    let success = match Command::new("cmake")
        .arg("--build")
        .arg("build")
        .current_dir(current_path)
        .status()
    {
        Ok(status) => status.success(),
        Err(error) => return error_print(format!("failed to build: {}", error)),
    };

    if !success {
        return;
    }

    let mut executable_path = PathBuf::new();
    if let Ok(dir) = read_dir(&build_path) {
        let mut executables = Vec::new();
        for item in dir {
            if let Ok(item) = item {
                if item.path().is_executable() {
                    let metadata = match item.metadata() {
                        Ok(metadata) => metadata,
                        Err(error) => {
                            return error_print(format!("failed to get metadata: {}", error))
                        }
                    };
                    let modified_at = match metadata.modified() {
                        Ok(modified_at) => modified_at,
                        Err(error) => {
                            return error_print(format!(
                                "failed to get last modified time: {}",
                                error
                            ))
                        }
                    };
                    executables.push((item.path(), modified_at));
                }
            }
        }
        executables.sort_by(|a, b| b.1.cmp(&a.1));
        if let Some(first) = executables.first() {
            executable_path = first.0.clone();
        }
    }

    if let Err(error) = Command::new(executable_path).status() {
        return error_print(format!("failed to run: {}", error));
    }
}
