@shape
Feature: Shape

Scenario: The default transformation
  Given s ← test_shape()
  Then s.transform = identity_matrix

Scenario: Assigning a transformation
  Given s ← test_shape()
  When set_transform(s, translation(2, 3, 4))
  Then s.transform = translation(2, 3, 4)

# These two tests are commented because the author of the book wants us to save a local variable in our test_shape in order to check its value
# However this way of doing this is forcing me to make my Shape's intersect function declared on a mutable reference of Shape
# for the sole purpose of testing which is a trade-off I'm not ready to accept
# Thus, I'm sad to say I have to disable these two tests, unless one day I find a more elegant way

#Scenario: Intersecting a scaled shape with a ray
#  Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
#    And s ← test_shape()
#  When set_transform(s, scaling(2, 2, 2))
#    And xs ← intersect(s, r)
#  Then s.saved_ray.origin = point(0, 0, -2.5)
#    And s.saved_ray.direction = vector(0, 0, 0.5)

#Scenario: Intersecting a translated shape with a ray
#  Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
#    And s ← test_shape()
#  When set_transform(s, translation(5, 0, 0))
#    And xs ← intersect(s, r)
#  Then s.saved_ray.origin = point(-5, 0, -5)
#    And s.saved_ray.direction = vector(0, 0, 1)

Scenario: Computing the normal on a translated shape
  Given s ← test_shape()
  When set_transform(s, translation(0, 1, 0))
    And n ← normal_at(s, point(0, 1.70711, -0.70711))
  Then n = vector(0, 0.70711, -0.70711)

Scenario: Computing the normal on a transformed shape
  Given s ← test_shape()
    And m ← scaling(1, 0.5, 1) * rotation_z(π/5)
  When set_transform(s, m)
    And n ← normal_at(s, point(0, √2/2, -√2/2))
  Then n = vector(0, 0.97014, -0.24254)
