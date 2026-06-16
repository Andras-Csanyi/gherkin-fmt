Feature: There is an empty line before every `Scenario Outline` block

  Rule: empty line before `Scenario Outline` block

  Scenario: Adding empty line before the `Scenario Outline` block
    Given the following input is provided
    ```gherkin
    Feature: This is a test input
      Scenario Outline: This is a scenario with the same indentation as the Feature
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Feature: This is a test input

      Scenario Outline: This is a scenario with the same indentation as the Feature
    ```

  Scenario: Removing extra lines before the `Scenario Outline` block
    Given the following input is provided
    ```gherkin
    Feature: This is a test input



      Scenario Outline: This is a scenario with the same indentation as the Feature
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Feature: This is a test input

      Scenario Outline: This is a scenario with the same indentation as the Feature
    ```
