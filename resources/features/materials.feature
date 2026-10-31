Feature: Materials

  Scenario: Reflectivity for the default material
    Given m ← material()
    Then m.reflective = 0.0

  Scenario: Transparency and Refractive Index for the default material
    Given m ← material()
    Then m.transparency = 0.0
    And m.refractive_index = 1.0
