use std::{
    io,
    path::{Path, PathBuf},
};

use crate::{
    recipe::Recipe,
    templates::{AboutTemplate, IndexTemplate, RecipeEntry, WritableTemplate},
};

pub fn build(recipes: Vec<(&PathBuf, Recipe)>, out_dir: &Path) -> io::Result<()> {
    std::fs::copy("static/style.css", out_dir.join("style.css"))?;
    std::fs::copy("static/script.js", out_dir.join("script.js"))?;
    IndexTemplate {
        recipes: recipes
            .into_iter()
            .map(|(path, recipe)| RecipeEntry {
                slug: slug(path),
                recipe: recipe,
            })
            .collect(),
    }
    .write_to(out_dir.to_path_buf().join("index.html").as_path())?;
    let _ = AboutTemplate {}.write_to(out_dir.to_path_buf().join("about/index.html").as_path())?;

    Ok(())
    // create about page
    //
    // todo!()
    // std::fs::create_dir_all(out_dir).unwrap_or_else(|e| {
    //     eprintln!("error: cannot create {}: {e}", out_dir.display());
    //     std::process::exit(2);
    // });

    // for (path, recipe) in recipes {
    //     let page = RecipeTemplate {
    //         recipe: recipe.clone(),
    //     };
    //     let html = page.render().expect("template should render");

    //     let out_path = out_dir.join(format!("{}.html", slug(path)));
    //     std::fs::write(&out_path, html).unwrap_or_else(|e| {
    //         eprintln!("error: cannot write {}: {e}", out_path.display());
    //         std::process::exit(2);
    //     });
    //     println!("wrote {}", out_path.display());
    // }
}

fn slug(path: &Path) -> String {
    path.parent()
        .and_then(|parent| parent.file_name())
        .or_else(|| path.file_stem())
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "recipe".to_string())
}

fn copy_static(from: &Path, to: &Path) {}
