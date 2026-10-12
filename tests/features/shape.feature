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
# However this way of doing this is forcing me to make my Shape's intersect function declared for an &mut Shape for the sole purpose of testing which is IMHO not acceptable
# Thus, I'm sad to say I have to disable these two tests, unless I find a more elegant way one day

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
