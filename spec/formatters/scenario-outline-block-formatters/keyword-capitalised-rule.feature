Feature: the `Scenario Outline` keyword is capitalized

  Rule: the `Scenario Outline` keyword is capitalized

  Scenario: The scenario in the `Scenario Outline` keyword is not capitalized
    Given the following input is provided
    ```gherkin
    scenario Outline: this is a test scenario outline
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Scenario Outline: this is a test scenario outline
    ```

  Scenario: The outline in the `Scenario Outline` keyword is not capitalized
    Given the following input is provided
    ```gherkin
    Scenario outline: this is a test scenario outline
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Scenario Outline: this is a test scenario outline
    ```

  Scenario: The `Scenario Outline` keyword is capitalized
    Given the following input is provided
    ```gherkin
    Scenario Outline: this is a test scenario outline
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Scenario Outline: this is a test scenario outline
    ```

