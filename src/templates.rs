use askama::Template;
use std::fs;
use std::io;
use std::path::Path;

use crate::recipe::Recipe;

/// A template that can write its rendered output to a given location.
pub trait WritableTemplate: Template {
    /// Render the template and write the output to `path`,
    /// creating parent directories as needed.
    fn write_to(&self, path: &Path) -> io::Result<()> {
        let output = self.render().map_err(io::Error::other)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, output)
    }
}

impl<T: Template> WritableTemplate for T {}

#[derive(Template)]
#[template(path = "recipe.html")]
pub struct RecipeTemplate {
    pub recipe: Recipe,
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate;

pub struct RecipeEntry {
    pub slug: String,
    pub recipe: Recipe,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub recipes: Vec<RecipeEntry>,
}
