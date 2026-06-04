Feature: Spheres

  Scenario: A sphere has a default material
    Given s ← sphere()
    When m ← s.material
    Then m = material()

  Scenario: A sphere may be assigned a material
    Given s ← sphere()
    And m ← material()
    And m.ambient ← 1
    When s.material ← m
    Then s.material = m

  Scenario: A helper for producing a sphere with a glassy material
    Given s ← glass_sphere()
    Then s.transform = identity_matrix
    And s.material.transparency = 1.0
    And s.material.refractive_index = 1.5
