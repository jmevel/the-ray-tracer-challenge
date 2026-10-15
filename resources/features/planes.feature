Feature: Planes

Scenario: Intersect with a ray parallel to the plane
  Given p ← plane()
    And r ← ray(point(0, 10, 0), vector(0, 0, 1))
  When xs ← local_intersect(p, r)
  Then xs is empty

Scenario: Intersect with a coplanar ray
  Given p ← plane()
    And r ← ray(point(0, 0, 0), vector(0, 0, 1))
  When xs ← local_intersect(p, r)
  Then xs is empty

Scenario: A ray intersecting a plane from above
  Given p ← plane()
    And r ← ray(point(0, 1, 0), vector(0, -1, 0))
  When xs ← local_intersect(p, r)
  Then xs.count = 1
    And xs[0].t = 1
    And xs[0].object = p

Scenario: A ray intersecting a plane from below
  Given p ← plane()
    And r ← ray(point(0, -1, 0), vector(0, 1, 0))
  When xs ← local_intersect(p, r)
  Then xs.count = 1
    And xs[0].t = 1
    And xs[0].object = p
