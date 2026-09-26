mod recipe;
mod templates;

use askama::Template;
use recipe::Recipe;

/// View model for the recipe page. Template fields are looked up on this
/// struct by name, so `{{ recipe.recipe_name }}` in the template reads
/// `self.recipe.recipe_name` here. Askama checks all of this at compile
/// time — a typo in the template is a build error, not a runtime one.
#[derive(Template)]
#[template(path = "recipe.html")]
struct RecipeTemplate {
    recipe: Recipe,
}

fn main() {
    let json = r#"{
        "recipe_name": "Example Pancakes",
        "yields": [{"amount": 4, "unit": "pieces"}],
        "ingredients": [
            {"ingredient": "Flour", "amounts": [{"amount": 250, "unit": "gr"}]},
            {"ingredient": "Milk", "amounts": [{"amount": 300, "unit": "ml"}]},
            {"ingredient": "Egg", "amounts": [{"amount": 2, "unit": "pieces"}]},
            {"ingredient": "Butter", "processing": ["melted"]}
        ],
        "steps": [
            {"step": "Whisk everything together into a smooth batter."},
            {"step": "Fry small ladles of batter in butter over medium heat."}
        ]
    }"#;

    let recipe = Recipe::from_json(json).expect("example JSON should parse");
    let page = RecipeTemplate { recipe };
    println!("{}", page.render().expect("template should render"));
}
