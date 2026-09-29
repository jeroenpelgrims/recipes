mod build;
mod check;
mod discovery;
mod recipe;
mod templates;

use build::build;
use check::check;
use discovery::collect_json_files;
use std::path::Path;

fn main() {
    let folder = std::env::args().nth(1).unwrap_or_else(|| usage());
    let command = std::env::args().nth(2).unwrap_or_else(|| usage());

    let files = collect_json_files(Path::new(&folder));

    match command.as_str() {
        "check" => {
            check(&files);
        }
        "build" => {
            let recipes = check(&files);
            build(&files);
        }
        _ => usage(),
    }
}

fn usage() -> ! {
    eprintln!("usage: recipes <path-to-recipes-folder> <check|build>");
    std::process::exit(2);
}
