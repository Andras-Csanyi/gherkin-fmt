Feature: There is an empty line before every `Background` block

  Rule: empty line before `Background` block

  Scenario: Adding empty line before the `Background` block
    Given the following input is provided
    ```gherkin
    Feature: This is a test input

    Background: This is a background with the same indentation as the Feature
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Feature: This is a test input

      Background: This is a Background with the same indentation as the Feature
    ```

  Scenario: Removing extra lines before the `Background` block
    Given the following input is provided
    ```gherkin
    Feature: This is a test input


    Background: This is a Background with the same indentation as the Feature
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Feature: This is a test input

    Background: This is a Background with the same indentation as the Feature
    ```
