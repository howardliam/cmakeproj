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
    #[command(name = "new", about = "Creates a new project at targeted directory")]
    New(NewArgs),

    #[command(
        name = "init",
        about = "Creates a new project within an existing directory"
    )]
    Init(InitArgs),

    #[command(name = "setup", about = "Sets up CMake for building")]
    Setup(SetupArgs),

    #[command(name = "build", about = "Builds the project")]
    Build(BuildArgs),

    #[command(name = "run", about = "Builds and runs the project")]
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
    println!("{} setup", "cmakeproj".green());
    println!("{} run", "cmakeproj".green());
}
