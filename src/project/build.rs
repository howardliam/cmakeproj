use std::{env::current_dir, fs::read_dir, process::Command};

use super::error_print;

#[derive(clap::Args)]
pub struct BuildArgs {
    #[arg(short, long)]
    pub build_dir: Option<String>,
}

pub fn build_project(args: BuildArgs) {
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
                            return error_print(
                                "CMakeCache.txt found but is a directory".to_owned(),
                            );
                        }
                    }
                    break;
                }
            }
        }
        if !cmakecache_found {
            return error_print("no CMakeCache.txt found".to_owned());
        }
    }

    match Command::new("cmake")
        .arg("--build")
        .arg("build")
        .current_dir(current_path)
        .status()
    {
        Ok(_) => {}
        Err(error) => error_print(format!("failed to build: {}", error)),
    }
}
