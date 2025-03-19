use clap::Parser;
use colored::Colorize;
use project::{
    build::{build_project, BuildArgs},
    create_all_files, error_print,
    init::{init_project, InitArgs},
    new::{new_project, NewArgs},
    run::{run_project, RunArgs},
    setup::{setup_project, SetupArgs},
    CreationMode,
};

mod project;

pub const DEFAULT_CMAKELISTS: &str = include_str!("../templates/CMakeLists.txt");
pub const DEFAULT_GITIGNORE: &str = include_str!("../templates/gitignore");
pub const DEFAULT_MAIN: &str = include_str!("../templates/main.cpp");
pub const DEFAULT_MAIN23: &str = include_str!("../templates/main23.cpp");
pub const DEFAULT_CLANGD: &str = include_str!("../templates/clangd");

#[derive(clap::Parser)]
enum CMakeProjCli {
    #[command(name = "new")]
    New(NewArgs),

    #[command(name = "init")]
    Init(InitArgs),

    #[command(name = "setup")]
    Setup(SetupArgs),

    #[command(name = "build")]
    Build(BuildArgs),

    #[command(name = "run")]
    Run(RunArgs),
}

fn main() {
    let result = match CMakeProjCli::parse() {
        CMakeProjCli::New(new_args) => new_project(new_args),
        CMakeProjCli::Init(init_args) => init_project(init_args),
        CMakeProjCli::Setup(setup_args) => return setup_project(setup_args),
        CMakeProjCli::Build(build_args) => return build_project(build_args),
        CMakeProjCli::Run(run_args) => return run_project(run_args),
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
