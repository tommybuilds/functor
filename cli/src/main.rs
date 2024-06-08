use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::{env, io, process};

use tokio::macros::*;

mod commands;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to override the current working directory
    #[arg(short, long)]
    dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(aliases = ["i"])]
    Init {
        #[arg()]
        template: String,
    },
    #[command(aliases = ["b"])]
    Build,
    #[command(aliases = ["d", "dev"])]
    Develop,
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let args = Args::parse();

    let working_directory = get_working_directory(&args);
    let functor_json_path = validate_metadata_path(&working_directory);

    let working_directory_os_str = working_directory.into_os_string();
    let working_directory_str = working_directory_os_str.into_string().unwrap();

    match &args.command {
        Command::Init { template } => {
            // TODO: Handle init
            println!(
                "TODO: Initialize with template '{}' in directory: {}",
                template, &working_directory_str,
            )
        }
        Command::Build => {
            println!("TODO: Build in directory: {}", &working_directory_str);
        }
        Command::Develop => {
            println!("Running develop... {}", &working_directory_str);
            let res = commands::develop::execute(&working_directory_str).await;
            println!("Done!");
        }
    }

    println!("Running command: {:?}", args.command);
    Ok(())
}

fn validate_metadata_path(working_directory: &PathBuf) -> PathBuf {
    let functor_path = working_directory.join("functor.json");

    if functor_path.exists() {
        println!("Found functor.json at {}", functor_path.display());
        // Optional: Read and parse the JSON file
        // let content = fs::read_to_string(&functor_path).expect("Failed to read functor.json");
        // let json: serde_json::Value = serde_json::from_str(&content).expect("Failed to parse functor.json");
        // println!("Content of functor.json: {}", json);
    } else {
        eprintln!("functor.json not found in {}", working_directory.display());
        process::exit(1);
    }

    functor_path
}

fn get_working_directory(args: &Args) -> PathBuf {
    let dir = args
        .dir
        .clone()
        .unwrap_or_else(|| env::current_dir().expect("Failed to get current directory"));
    println!("Hello from directory: {}", dir.display());
    dir
}