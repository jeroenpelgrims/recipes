mod build;
mod check;
mod discovery;
mod recipe;
mod templates;

use build::build;
use check::check;
use clap::{Parser, Subcommand};
use discovery::collect_json_files;
use std::{io, path::PathBuf};

#[derive(Parser)]
#[command(
    name = "recipes",
    version,
    about = "Validate and build the recipe collection"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate all recipe JSON files in a folder
    Check {
        /// Path to the recipes folder
        dir: PathBuf,
    },
    /// Build the static site from the recipes
    Build {
        /// Path to the recipes folder
        dir: PathBuf,
        /// Output folder for the generated HTML files
        #[arg(long, default_value = "out")]
        out_dir: PathBuf,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let (dir, out_dir) = match cli.command {
        Command::Check { dir } => (dir, None),
        Command::Build { dir, out_dir } => (dir, Some(out_dir)),
    };

    if !dir.is_dir() {
        eprintln!("error: {} is not a directory", dir.display());
        std::process::exit(1);
    }

    let files = collect_json_files(&dir);
    let recipes = check(&files);

    if let Some(out_dir) = out_dir {
        build(recipes, &out_dir)?;
    }

    Ok(())
}
