
Feature: test feature

  Background: test background
    Given this is a not indented step
    And another one
      | header1 | value1 |
      | header2 | value2 |
      | header3 | value3 |
    When another unindented step
      | header1 | value1 |
      | header2 | value2 |
      | header3 | value3 |
    And another one again
    Then once more
      | header1 | value1 |
      | header2 | value2 |
      | header3 | value3 |
    And why not another one again
