use std::{
    env::current_dir,
    fs::{create_dir_all, read_dir},
    process::Command,
};

use super::error_print;

#[derive(clap::Args)]
pub struct SetupArgs {
    #[arg(short, long)]
    pub build_dir: Option<String>,
}

pub fn setup_project(args: SetupArgs) {
    let current_path = match current_dir() {
        Ok(directory) => directory,
        Err(error) => return error_print(format!("failed to get current directory: {}", error)),
    };

    if let Ok(dir) = read_dir(&current_path) {
        let mut cmakelists_found = false;
        for item in dir {
            if let Ok(item) = item {
                cmakelists_found = item.file_name().to_string_lossy() == "CMakeLists.txt";
                if cmakelists_found {
                    if let Ok(file_type) = item.file_type() {
                        if file_type.is_dir() {
                            return error_print(
                                "CMakeLists.txt found but is a directory".to_owned(),
                            );
                        }
                    }
                    break;
                }
            }
        }
        if !cmakelists_found {
            return error_print("no CMakeLists.txt found".to_owned());
        }
    }

    let build_dir = match args.build_dir {
        Some(build_dir) => build_dir,
        None => "build".to_owned(),
    };

    let build_path = {
        let mut dir = current_path.clone();
        dir.push(&build_dir);
        dir
    };

    match create_dir_all(&build_path) {
        Ok(_) => {}
        Err(error) => return error_print(format!("failed to create src directory: {}", error)),
    }

    match Command::new("cmake")
        .arg("-B")
        .arg("build")
        .arg("-G")
        .arg("Ninja")
        .current_dir(current_path)
        .status()
    {
        Ok(_) => {}
        Err(error) => error_print(format!("failed to set up build: {}", error)),
    }
}
