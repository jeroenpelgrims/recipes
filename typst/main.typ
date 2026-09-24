#set block(spacing: 1.5em)
#import "recipe.typ": render-recipe

#let recipe-folders = json("recipes.json")

#for (i, folder) in recipe-folders.enumerate() {
  let recipe = json("../recipes/" + folder + "/recipe.json")
  render-recipe(recipe)
  if i < recipe-folders.len() - 1 {
    pagebreak()
  }
}
