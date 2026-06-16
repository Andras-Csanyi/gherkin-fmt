Feature: There is an empty line before every `Examples` block

  Rule: empty line before `Examples` block

  Scenario: Adding empty line before the `Examples` block
    Given the following input is provided
    ```gherkin
      Scenario Outline: This is a scenario with the same indentation as the Feature
        Examples:
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
      Scenario Outline: This is a scenario with the same indentation as the Feature

        Examples:
    ```

  Scenario: Removing extra lines before the `Examples` block
    Given the following input is provided
    ```gherkin
      Scenario Outline: This is a scenario with the same indentation as the Feature




        Examples:
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
      Scenario Outline: This is a scenario with the same indentation as the Feature

        Examples:
    ```
