use std::path::PathBuf;

use askama::Template;

use crate::recipe::Recipe;
use crate::templates::RecipeTemplate;

pub fn build(files: &[PathBuf]) {
    for path in files {
        let json = std::fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("error: cannot read {}: {e}", path.display());
            std::process::exit(2);
        });

        let recipe = Recipe::from_json(&json).unwrap_or_else(|e| {
            eprintln!("FAIL: {}: {e}", path.display());
            std::process::exit(1);
        });

        let page = RecipeTemplate { recipe };
        println!("{}", page.render().expect("template should render"));
    }
}
