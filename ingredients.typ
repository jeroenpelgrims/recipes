#let render-ingredients(recipe) = [
  == Ingredients
  #layout(size => {
    let min-col-width = 7cm
    let cols = calc.max(1, calc.floor(size.width / min-col-width))
    grid(
      columns: (1fr,) * cols,
      gutter: 1em,

      ..recipe.ingredients.map(ingredient => {
        let name = ingredient.ingredient
        if "amounts" in ingredient {
          let (amount, unit) = ingredient.amounts.first()
          [#amount #unit *#name*]
        } else {
          [*#name*]
        }
        if "processing" in ingredient [
          #text(size: 0.85em, fill: luma(30%))[(#ingredient.processing.join(", "))]
        ]
      })
    )
  })
]
