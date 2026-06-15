Feature: Case 1 test

  Misaligned story part line 1
  Misaligned story part line 2
  Misaligned story part line 3

  Scenario: Case 1 scenario 1
    Given this step is misaligned
    And this one too
    When we read this
    Then it hurts
    And must be formatted

  Scenario Outline: Case 1 scenario outline
    Given this is not capitalised
    And this one neither
    When neither
    And here is a table
      | header1 | header2 | header3 |
      | value1  | value2  | value3  |
    Then the whole must be formatted correctly

  Scenario Outline: Case 2 scenario outline
    Given this is not capitalised
      | header1 | header2 | header3 |
      | value1  | value2  | value3  |
    And this one neither
      | key  | value  |
      | key2 | value2 |
      | key3 | value3 |
      | key4 | value4 |
    When neither
    And here is a table
      | header1 | header2 | header3 |
      | value1  | value2  | value3  |
    Then the whole must be formatted correctly