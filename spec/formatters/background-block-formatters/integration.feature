Feature: `Background` block formatting integration tests

  Scenario: Format the `Background` block simple case
    Given the following input is provided
    ```gherkin
    Feature: test feature
    Background: test background
    Given this is a not indented step
    When another unindented step
    Then once more
    ```
    When the formatter is executed
    Then the result is the following
    ```gherkin
    Feature: test feature

      Background: test background
        Given this is a not indented step
        When another unindented step
        Then once more
    ```

  Scenario: Format the `Background` block more complex case
    Given the following input is provided
    ```gherkin
    Feature: test feature


    Background: test background
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

      Background: test background
        Given this is a not indented step
        And another one
        When another unindented step
        And another one again
        Then once more
        And why not another one again
    ```

  Scenario: Format the `Background` block chaos
    Given the following input is provided
    ```gherkin
    Feature: test feature





Background: test background
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

      Background: test background
        Given this is a not indented step
        And another one
        When another unindented step
        And another one again
        Then once more
        And why not another one again
    ```
