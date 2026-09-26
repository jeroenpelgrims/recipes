.PHONY: pdf manifest check-recipe check clean

pdf: manifest check
	typst compile --root . pdf/main.typ recipes.pdf

manifest:
	ls -d recipes/*/ | xargs -n1 basename | jq -R . | jq -s . > pdf/recipes.json

check-recipe:
	cargo build --release --manifest-path site/Cargo.toml --bin check-recipe

check: check-recipe
	ls recipes/*/recipe.json | xargs -n1 ./site/target/release/check-recipe

clean:
	cargo clean --manifest-path site/Cargo.toml
