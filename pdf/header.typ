#import "details.typ": render-details

#let render-header(recipe, folder) = [
  = #recipe.recipe_name

  #let has-image = "X-image" in recipe

  #grid(
    columns: if has-image { (2fr, 1fr) } else { (1fr,) },
    gutter: if has-image { 1.5em } else { 0em },
    if has-image [
      #block(width: 100%, height: 6cm, clip: true)[
        #image("../recipes/" + folder + "/" + recipe.at("X-image"), width: 100%)
      ]
    ],
    align(horizon)[#render-details(recipe)],
  )
]
