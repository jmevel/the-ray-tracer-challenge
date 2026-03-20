Feature: Matrix Transformations

Scenario: Multiplying by a translation matrix
  Given transform ← translation(5, -3, 2)
  And p ← point(-3, 4, 5)
  Then transform * p = point(2, 1, 7)
