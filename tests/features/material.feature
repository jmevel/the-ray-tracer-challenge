@material
Feature: Material

  Background:
    Given m ← material()
    And position ← point(0, 0, 0)

  Scenario: The default material
    Then m.color = color(1, 1, 1)
    And m.ambient = 0.1
    And m.diffuse = 0.9
    And m.specular = 0.9
    And m.shininess = 200.0

  Scenario: Lighting with the eye between the light and the surface
    Given eyev ← vector(0, 0, -1)
    And normalv ← vector(0, 0, -1)
    And light ← point_light(point(0, 0, -10), color(1, 1, 1))
    When result ← lighting(m, light, position, eyev, normalv)
    Then result = color(1.9, 1.9, 1.9)

  Scenario: Lighting with the eye between light and surface, eye offset 45°
    Given eyev ← vector(0, √2/2, -√2/2)
    And normalv ← vector(0, 0, -1)
    And light ← point_light(point(0, 0, -10), color(1, 1, 1))
    When result ← lighting(m, light, position, eyev, normalv)
    Then result = color(1.0, 1.0, 1.0)

  Scenario: Lighting with eye opposite surface, light offset 45°
    Given eyev ← vector(0, 0, -1)
    And normalv ← vector(0, 0, -1)
    And light ← point_light(point(0, 10, -10), color(1, 1, 1))
    When result ← lighting(m, light, position, eyev, normalv)
    Then result = color(0.7364, 0.7364, 0.7364)

  Scenario: Lighting with eye in the path of the reflection vector
    Given eyev ← vector(0, -√2/2, -√2/2)
    And normalv ← vector(0, 0, -1)
    And light ← point_light(point(0, 10, -10), color(1, 1, 1))
    When result ← lighting(m, light, position, eyev, normalv)
    # Values from the book aren't precise enough
    # In scenario 'two tuples with different values are different' we make the test that 4.00001 does NOT equal 4
    # But here the books wants to test that two values with even less fraction digits ARE EQUAL (1.6364 and 1.6363853)
    # Point and Color are both Tuple so they share the same EPSILON value
    Then result = color(1.6363853, 1.6363853, 1.6363853)

  Scenario: Lighting with the light behind the surface
    Given eyev ← vector(0, 0, -1)
    And normalv ← vector(0, 0, -1)
    And light ← point_light(point(0, 0, 10), color(1, 1, 1))
    When result ← lighting(m, light, position, eyev, normalv)
    Then result = color(0.1, 0.1, 0.1)
