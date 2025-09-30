@canvas
#noinspection CucumberUndefinedStep
Feature: Canvas

  Scenario: Creating a canvas
    Given c ← canvas(10, 20)
    Then c.width = 10
    And c.height = 20
    And every pixel of c is color(0, 0, 0)