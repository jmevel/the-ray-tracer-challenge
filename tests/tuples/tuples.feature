Feature: Tuples, Vectors, and Points

  Scenario: A tuple with w=1.0 is a point
    Given a ← tuple(4.3, -4.2, 3.1, 1.0)
    Then a.x = 4.3
    And a.y = -4.2
    And a.z = 3.1
    And a.w = 1.0
    And a is a point
    And a is not a vector

  Scenario: A tuple with w=0 is a vector
    Given a ← tuple(4.3, -4.2, 3.1, 0.0)
    Then a.x = 4.3
    And a.y = -4.2
    And a.z = 3.1
    And a.w = 0.0
    And a is not a point
    And a is a vector

  Scenario: point() creates tuples with w=1
    Given p ← point(4, -4, 3)
    Then p = tuple(4, -4, 3, 1)

  Scenario: vector() creates tuples with w=0
    Given v ← vector(4, -4, 3)
    Then v = tuple(4, -4, 3, 0)

  Scenario: two tuples with different values are different
    Given p1 ← point(4, -4, 3)
    Given p2 ← point(4.00001, -4, 3)
    Then p1 does not equal p2

  Scenario: two tuples with very close values are equal
    Given p1 ← point(4, -4, 3)
    Given p2 ← point(4.000009, -4, 3)
    Then p1 equals p2

