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

  Scenario outline: Case 1 scenario outline
    given this is not capitalised
    and this one neither
    when neither
    And here is a table
    |header1|header2|header3|
    |value1|value2|value3|
  Then the whole must be formatted correctly

  scenario Outline: Case 2 scenario outline
    given this is not capitalised
    |header1|header2|header3|
    |value1|value2|value3|
    and this one neither
      |key|value|
      |key2|value2|
      |key3|value3|
      |key4|value4|
    when neither
    And here is a table
    |header1|header2|header3|
    |value1|value2|value3|
  Then the whole must be formatted correctly

  Scenario: raw content
    given this step has raw content
    ```gherkin
    Feature: whatever feature
      Scenario: whatever scenario
        Given whatever step
        And the whole must remain unchanged
    ```
    but this but must be formatted

  Scenario: raw content
    given this step has raw content
    and this and that
    ```gherkin
    feature: whatever feature
    scenario: whatever scenario
    given whatever step
    and the whole must remain unchanged
    ```
    when whatever
    And this whatever
    but this but must be formatted
    Then whateverium
