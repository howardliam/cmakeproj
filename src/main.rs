use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    process::{self, Command},
};

use clap::Parser;

const DEFAULT_CONFIG: &str = include_str!("../templates/CMakeLists.txt");
const DEFAULT_GITIGNORE: &str = include_str!("../templates/gitignore");
const DEFAULT_MAIN: &str = include_str!("../templates/main.cpp");

#[derive(clap::Parser)]
enum CMakeProjCli {
    #[command(name = "new")]
    New(Args),
}

#[derive(clap::Args)]
struct Args {
    #[arg(value_name = "NAME")]
    pub project_name: String,
}

fn main() {
    let args = match CMakeProjCli::parse() {
        CMakeProjCli::New(args) => args,
    };

    let project_name = args.project_name.clone();

    let config_with_project_name = DEFAULT_CONFIG.replace("{{PROJECT_NAME}}", &project_name);

    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(error) => {
            eprintln!("failed to get current directory: {}", error);
            process::exit(1);
        }
    };

    let mut project_dir = current_dir;
    project_dir.push(&project_name);

    let mut cmakelists_file = project_dir.clone();
    cmakelists_file.push("CMakeLists.txt");

    let mut gitignore_file = project_dir.clone();
    gitignore_file.push(".gitignore");

    let mut src_dir = project_dir.clone();
    src_dir.push("src");

    let mut main_file = src_dir.clone();
    main_file.push("main.cpp");

    match fs::create_dir(&project_dir) {
        Ok(_) => {}
        Err(_) => {
            eprintln!(
                "error: destination `{}` already exists",
                project_dir.to_str().unwrap()
            );

            process::exit(1);
        }
    }

    match fs::create_dir(&src_dir) {
        Ok(_) => {}
        Err(_) => {
            eprintln!(
                "error: destination `{}` already exists",
                src_dir.to_str().unwrap()
            );

            process::exit(1);
        }
    }

    let mut file = match OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&cmakelists_file)
    {
        Ok(file) => file,
        Err(_) => {
            eprintln!(
                "error: failed to create file `{}`",
                cmakelists_file.to_str().unwrap()
            );

            process::exit(1);
        }
    };
    let _ = file.write_all(config_with_project_name.as_bytes());

    let mut file = match OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&main_file)
    {
        Ok(file) => file,
        Err(_) => {
            eprintln!(
                "error: failed to create file `{}`",
                main_file.to_str().unwrap()
            );

            process::exit(1);
        }
    };
    let _ = file.write_all(DEFAULT_MAIN.as_bytes());

    let mut file = match OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&gitignore_file)
    {
        Ok(file) => file,
        Err(_) => {
            eprintln!(
                "error: failed to create file `{}`",
                gitignore_file.to_str().unwrap()
            );

            process::exit(1);
        }
    };
    let _ = file.write_all(DEFAULT_GITIGNORE.as_bytes());

    Command::new("git").arg("init").current_dir(&project_dir);

    println!("new project created");
    println!("cd {}", project_name);
    println!("cmake -B build -G 'Ninja'");
    println!("cmake --build build && ./build/{}", project_name);
}
