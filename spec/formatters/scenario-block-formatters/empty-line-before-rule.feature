Feature: There is an empty line before every `Scenario` block

  Rule: empty line before `Scenario` block

  Scenario: Adding empty line before the `Scenario` block
    Given the following input is provided
    ```gherkin
    Feature: This is a test input
      Scenario: This is a scenario with the same indentation as the Feature
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Feature: This is a test input

      Scenario: This is a scenario with the same indentation as the Feature
    ```

  Scenario: Removing extra lines before the `Scenario` block
    Given the following input is provided
    ```gherkin
    Feature: This is a test input



      Scenario: This is a scenario with the same indentation as the Feature
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Feature: This is a test input

      Scenario: This is a scenario with the same indentation as the Feature
    ```
