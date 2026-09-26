mod recipe;
use recipe::Recipe;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: check-recipe <path-to-recipe.json>");
        std::process::exit(2);
    });

    let json = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("error: cannot read {path}: {e}");
        std::process::exit(2);
    });

    match Recipe::from_json(&json) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("FAIL: {path}: {e}");
            std::process::exit(1);
        }
    }
}
