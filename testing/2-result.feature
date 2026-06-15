Feature: Case 2 test

  Misaligned story part line 1
  Misaligned story part line 2
  Misaligned story part line 3

  Scenario: Case 1 scenario 1
    Given this step is misaligned
      | header 1 | header 2 | header 3 |
      | value1   | value2   | value3   |
    And this one too
      | key 1 | value 1 |
      | key 2 | value 2 |
      | key 3 | value 3 |
    When we read this
    Then it hurts
    And must be formatted

  Scenario: Case 2 scenario 2
    Given this step is misaligned
      | header 1           | header 2                  | header 3 |
      | value1             | value2 asdfa  afasdfasdfa | value3   |
      | value1 adfasdfasdf | value2 asdfa  afasdfasdfa | value3   |
    And this one too
      | key 1 | value 1 |
      | key 2 | value 2 |
      | key 3 | value 3 |
    When we read this
    Then it hurts
    And must be formatted