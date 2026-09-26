#let render-steps(recipe) = [
  == Steps
  #enum(
    ..recipe.steps.map(step => [#step.step]),
  )
]
