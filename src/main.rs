use clap::Parser;
use colored::Colorize;
use project::{
    create_all_files, error_print,
    init::{init_project, InitArgs},
    new::{new_project, NewArgs},
    CreationMode,
};

mod project;

pub const DEFAULT_CMAKELISTS: &str = include_str!("../templates/CMakeLists.txt");
pub const DEFAULT_GITIGNORE: &str = include_str!("../templates/gitignore");
pub const DEFAULT_MAIN: &str = include_str!("../templates/main.cpp");

#[derive(clap::Parser)]
enum CMakeProjCli {
    #[command(name = "new")]
    New(NewArgs),

    #[command(name = "init")]
    Init(InitArgs),
}

fn main() {
    println!("\n{}\n", "cmakeproj".underline().bold().blue());

    let result = match CMakeProjCli::parse() {
        CMakeProjCli::New(args) => new_project(args),
        CMakeProjCli::Init(args) => init_project(args),
    };

    let details = match result {
        Ok(details) => match create_all_files(details) {
            Ok(details) => details,
            Err(error) => return error_print(error),
        },
        Err(error) => return error_print(error),
    };

    println!("\nProject `{}` created\n", details.name.magenta());
    if let CreationMode::New | CreationMode::Init = details.mode {
        println!("{} {}", "cd".green(), details.name);
    }
    println!("{} -B build -G {}", "cmake".green(), "'Ninja'".yellow());
    println!(
        "{} --build build && {}{}",
        "cmake".green(),
        "./build/".green(),
        details.name.green()
    );
}
