use itertools::{Either, Itertools};
use std::path::PathBuf;

use crate::recipe::Recipe;

#[derive(Debug, thiserror::Error)]
pub enum ParseRecipeError {
    #[error("cannot read {path}: {source}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("Invalid JSON in {path}: {source}")]
    Json {
        path: String,
        #[source]
        source: serde_json::Error,
    },
}

pub fn check(files: &[PathBuf]) -> Vec<(&PathBuf, Recipe)> {
    let recipes = files.iter().map(|path| (path, parse_recipe(path)));

    let (recipes, errors): (Vec<_>, Vec<_>) = recipes.partition_map(|r| match r {
        (path, Ok(recipe)) => Either::Left((path, recipe)),
        (_, Err(msg)) => Either::Right(msg),
    });

    if !errors.is_empty() {
        for err in errors {
            eprintln!("{err}")
        }
        std::process::exit(1);
    }

    recipes
}

fn parse_recipe(path: &PathBuf) -> Result<Recipe, ParseRecipeError> {
    let str_path = path.to_string_lossy().into_owned();
    let json = std::fs::read_to_string(path).map_err(|source| ParseRecipeError::Read {
        path: str_path.clone(),
        source,
    })?;
    serde_json::from_str::<Recipe>(json.as_str()).map_err(|source| ParseRecipeError::Json {
        path: str_path.clone(),
        source,
    })
}
