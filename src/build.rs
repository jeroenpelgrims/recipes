use std::{
    io,
    path::{Path, PathBuf},
};

use crate::{
    recipe::Recipe,
    templates::{AboutTemplate, IndexTemplate, RecipeEntry, RecipeTemplate, WritableTemplate},
};

pub fn build(recipes: Vec<(&PathBuf, Recipe)>, out_dir: &Path) -> io::Result<()> {
    copy_static_files(Path::new("static"), out_dir)?;

    IndexTemplate {
        recipes: recipes
            .iter()
            .map(|(path, recipe)| RecipeEntry {
                slug: slug(path),
                recipe: recipe.clone(),
            })
            .collect(),
    }
    .write_to(out_dir.to_path_buf().join("index.html").as_path())?;
    AboutTemplate {}.write_to(out_dir.to_path_buf().join("about/index.html").as_path())?;

    for (path, recipe) in recipes {
        let recipe_dir = out_dir.join(slug(path));
        RecipeTemplate {
            recipe: recipe.clone(),
        }
        .write_to(&recipe_dir.join("index.html"))?;
        copy_recipe_image(path, &recipe, &recipe_dir)?;
    }

    Ok(())
}

fn copy_static_files(static_dir: &Path, out_dir: &Path) -> io::Result<()> {
    for entry in std::fs::read_dir(static_dir)? {
        let path = entry?.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                std::fs::copy(&path, out_dir.join(file_name))?;
            }
        }
    }
    Ok(())
}

fn slug(path: &Path) -> String {
    path.parent()
        .and_then(|parent| parent.file_name())
        .or_else(|| path.file_stem())
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "recipe".to_string())
}

/// If the recipe has an `X-image` extension field, copy the referenced
/// image file from the recipe's source directory into its output directory.
fn copy_recipe_image(recipe_path: &Path, recipe: &Recipe, out_dir: &Path) -> io::Result<()> {
    let (Some(image), Some(src_dir)) = (recipe.image(), recipe_path.parent()) else {
        return Ok(());
    };
    let src = src_dir.join(image);
    if src.is_file() {
        std::fs::create_dir_all(out_dir)?;
        std::fs::copy(&src, out_dir.join(image))?;
    } else {
        eprintln!(
            "warning: {} references missing image {}",
            recipe_path.display(),
            src.display()
        );
    }
    Ok(())
}
