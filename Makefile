.PHONY: pdf manifest recipes-bin check site live-server dev bacon watch serve clean

out:
	mkdir out

pdf: manifest check out
	typst compile --root . pdf/main.typ out/recipes.pdf

manifest:
	ls -d recipes/*/ | xargs -n1 basename | jq -R . | jq -s . > pdf/recipes.json

recipes-bin:
	cargo build --release

check: recipes-bin
	./target/release/recipes check ./recipes

build: recipes-bin out
	./target/release/recipes build ./recipes --out-dir out

miniserve:
	@command -v miniserve >/dev/null 2>&1 || cargo install miniserve --locked

bacon:
	@command -v bacon >/dev/null 2>&1 || cargo install bacon --locked

watch: bacon miniserve 
	bacon site

serve: miniserve 
	miniserve -p 3000 --index index.html out

clean:
	rm -rf out
	cargo clean --manifest-path site/Cargo.toml
