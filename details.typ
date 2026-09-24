#let render-details(recipe) = [
  #if "yields" in recipe [
    *Serves:* #recipe.yields.first().amount #recipe.yields.first().unit \
  ]
  #if "oven_temp" in recipe [
    *Oven:* #recipe.oven_temp.first().amount °#recipe.oven_temp.first().unit
    #if "oven_fan" in recipe [ (fan #recipe.oven_fan)] \
  ]
  #if "oven_time" in recipe [
    *Oven time:* #recipe.oven_time \
  ]
]
