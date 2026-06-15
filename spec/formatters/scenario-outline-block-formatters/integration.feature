Feature: `Scenario Outline` block formatting integration tests

  Scenario: Format the `Scenario Outline` block simple case
    Given the following input is provided
    ```gherkin
    Feature: test feature
    Scenario Outline: test scenario
    Given this is a not indented step
    When another unindented step
    Then once more
    ```
    When the formatter is executed
    Then the result is the following
    ```gherkin
    Feature: test feature

    Scenario Outline: test scenario
      Given this is a not indented step
      When another unindented step
      Then once more
    ```

  Scenario: Format the `Scenario Outline` block more complex case
    Given the following input is provided
    ```gherkin
    Feature: test feature


    Scenario Outline: test scenario
  Given this is a not indented step
       And another one
    When another unindented step
             And another one again
    Then once more
And why not another one again
    ```
    When the formatter is executed
    Then the result is the following
    ```gherkin
    Feature: test feature

      Scenario Outline: test scenario
        Given this is a not indented step
        And another one
        When another unindented step
        And another one again
        Then once more
        And why not another one again
    ```

  Scenario: Format the `Scenario Outline` block chaos
    Given the following input is provided
    ```gherkin
    Feature: test feature





Scenario Outline: test scenario
  Given this is a not indented step
       And another one
    When another unindented step
             And another one again
    Then once more
And why not another one again
    ```
    When the formatter is executed
    Then the result is the following
    ```gherkin
    Feature: test feature

      Scenario Outline: test scenario
        Given this is a not indented step
        And another one
        When another unindented step
        And another one again
        Then once more
        And why not another one again
    ```
