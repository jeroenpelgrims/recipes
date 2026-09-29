.PHONY: pdf manifest recipes-bin check site live-server dev bacon watch serve clean

out:
	mkdir out

pdf: manifest check out
	typst compile --root . pdf/main.typ out/recipes.pdf

manifest:
	ls -d recipes/*/ | xargs -n1 basename | jq -R . | jq -s . > pdf/recipes.json

recipes-bin:
	cargo build --release --manifest-path site/Cargo.toml

check: recipes-bin
	./site/target/release/recipes ./recipes check

site: recipes-bin out
	./site/target/release/recipes ./recipes build

miniserve:
	@command -v miniserve >/dev/null 2>&1 || cargo install miniserve --locked

bacon:
	@command -v bacon >/dev/null 2>&1 || cargo install bacon --locked

serve: build bacon miniserve 
	bacon site

# watch: bacon
# 	bacon site/

# Run the rebuild watcher and dev server together: bacon (headless)
# rebuilds out/ on source changes, live-server reloads the browser.
# dev: site live-server bacon
# 	@bacon --headless --job site site/ & \
# 	bacon_pid=$$!; \
# 	live-server -p 3000 out; \
# 	kill $$bacon_pid 2>/dev/null; true

clean:
	rm -rf out
	cargo clean --manifest-path site/Cargo.toml
