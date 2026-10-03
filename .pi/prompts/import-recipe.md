---
description: Import a recipe from a URL into the recipes/ collection — parses it, converts units to metric via the specialist agents, and writes recipes/<slug>/recipe.json
argument-hint: "<recipe-url>"
---

Import the recipe at $1 into this collection. (If no URL was given, ask the user for one and stop.)

Do NOT parse or convert the recipe yourself — that work belongs to the specialist agents. Your job is orchestration:

1. **Convert via the recipe-importer agent.** Call the Agent tool with `subagent_type: "recipe-importer"` and a prompt like: "Convert the recipe at $1 to Open Recipe Format JSON. Follow your definition exactly: fetch the URL yourself, delegate any US-unit conversions to the unit-converter agent, and run your mandatory verification loop."
   - If the agent reports that `check` did NOT pass, stop. Do not write anything; relay the agent's failure report to the user.
   - If the Agent tool rejects the `recipe-importer` subagent type (agent registry may require a fresh session), stop and tell the user to `/reload` or restart pi — do not fall back to doing the conversion inline.

2. **Pick the slug.** From the verified JSON's `recipe_name`, derive a folder slug: lowercase, ASCII (transliterate diacritics, e.g. `crêpes` → `crepes`), words joined with hyphens, short but recognizable (e.g. "Rosemary Garlic Focaccia Bread" → `focaccia`). Look at the existing `recipes/` folders first and match their style. If `recipes/<slug>/` already exists, DO NOT overwrite it — stop and propose an alternative slug to the user.

3. **Write the file.** Create `recipes/<slug>/` and write the agent's JSON **verbatim** — the exact bytes of its ```json code fence, no re-typing, reformatting, or re-ordering — to `recipes/<slug>/recipe.json`. The file inside the folder is ALWAYS named `recipe.json`.

4. **Final gate.** From the repo root run `cargo run -q -- check recipes`. It must exit 0. If it fails, remove the folder you just created, and report the error.

5. **Report back:** the slug and path written, which quantities the agents converted to metric, anything flagged `NOT CONVERTED` or `⚠ web-sourced`, the agent's other assumptions, and the final check result.
