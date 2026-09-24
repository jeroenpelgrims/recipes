#!/bin/sh
# Generates recipes.json with the names of all folders in recipes/
cd "$(dirname "$0")"
ls -d recipes/*/ | xargs -n1 basename | jq -R . | jq -s . > recipes.json
