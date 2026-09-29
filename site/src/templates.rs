use askama::Template;

use crate::recipe::Recipe;

#[derive(Template)]
#[template(path = "recipe.html")]
pub struct RecipeTemplate {
    pub recipe: Recipe,
}
