Feature: the `Background` keyword is capitalized

  Rule: the `Background` keyword is capitalized

  Scenario: The `Background` keyword is not capitalized
    Given the following input is provided
    ```gherkin
    background: this is a test background
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Background: this is a test background
    ```

  Scenario: The `Background` keyword is capitalized
    Given the following input is provided
    ```gherkin
    Background: this is a test background
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Background: this is a test background
    ```

