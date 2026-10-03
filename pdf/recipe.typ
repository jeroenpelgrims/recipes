#import "header.typ": render-header
#import "ingredients.typ": render-ingredients
#import "steps.typ": render-steps

#let render-recipe(recipe, folder) = [
  #render-header(recipe, folder)

  #render-ingredients(recipe)

  #render-steps(recipe)
]
