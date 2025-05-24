mod create_dir;
mod create_file;

use std::io::Error;

use clap::Parser;
use create_dir::{add_basic_dependencies, create_dirs, init_cargo_project};
use create_file::{change_main, create_env_example_file};

const APP_NAME: &'static str = "create-axum-app";
const APP_VERSION: &'static str = "0.0.1";
const DEFAULT_SERVER_URL: &'static str = "0.0.0.0:3000";

#[derive(Parser, Debug)]
#[command(name=APP_NAME,author, version, about, long_about = None)]
struct CreateAxumApp {
    name: String,
    #[arg(short, long, default_value_t = String::from(DEFAULT_SERVER_URL) )]
    server_url: String,
}
fn main() -> Result<(), Error> {
    let args = CreateAxumApp::parse();

    if args.name.is_empty() {
        println!("Please provide a name for the project");
        std::process::exit(1)
    }

    if let Err(err) = init_cargo_project(&args.name) {
        eprintln!("Error: {}", err);
        std::process::exit(1)
    }

    if let Err(err) = create_dirs(&args.name) {
        eprintln!("Error: {}", err);
        std::process::exit(1)
    }
    if let Err(err) = add_basic_dependencies(&args.name) {
        eprintln!("Error: {}", err);
        std::process::exit(1)
    }

    if let Err(err) = change_main(&args.name) {
        eprintln!("Error: {}", err);
        std::process::exit(1)
    }

    if let Err(err) = create_env_example_file(&args.name, &args.server_url) {
        eprintln!("Error: {}", err);
        std::process::exit(1)
    }

    println!("--------------------------------------------------------------");
    println!(
        "✅ Project {} created successfully by create-axum-app v-{}",
        args.name, APP_VERSION
    );
    println!();
    println!();
    println!("Run the following commands to start the project");
    println!("cd {}", args.name);
    println!("cargo run");
    println!("--------------------------------------------------------------");

    Ok(())
}
