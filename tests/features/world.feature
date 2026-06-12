@world
Feature: World

  Scenario: Creating a world
    Given w ← world()
    Then w contains no objects
    And w has no light source
