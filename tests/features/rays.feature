@rays
Feature: Rays

  Scenario: Creating and querying a ray
    Given origin ← point(1, 2, 3)
    And direction ← vector(4, 5, 6)
    When r ← ray(origin, direction)
    Then r.origin = origin
    And r.direction = direction
