# Build a PDF of all recipes using typst
#
# Usage:
#   make pdf        # build the PDF
#   make manifest   # regenerate pdf/recipes.json

.PHONY: pdf manifest

pdf: manifest
	typst compile --root . pdf/main.typ recipes.pdf

manifest:
	ls -d recipes/*/ | xargs -n1 basename | jq -R . | jq -s . > pdf/recipes.json
