.PHONY: pdf manifest check-recipe build-site check site miniserve dev clean

out:
	mkdir out

pdf: manifest check out
	typst compile --root . pdf/main.typ out/recipes.pdf

manifest:
	ls -d recipes/*/ | xargs -n1 basename | jq -R . | jq -s . > pdf/recipes.json

check-recipe:
	cargo build --release --manifest-path site/Cargo.toml --bin check-recipe

check: check-recipe
	ls recipes/*/recipe.json | xargs -n1 ./site/target/release/check-recipe

build-site:
	cargo build --release --manifest-path site/Cargo.toml --bin build-site

site: build-site out
	./site/target/release/build-site

miniserve:
	@command -v miniserve >/dev/null 2>&1 || cargo install miniserve

dev: site miniserve
	miniserve -p 3000 --index index.html out

clean:
	rm -rf out
	cargo clean --manifest-path site/Cargo.toml
