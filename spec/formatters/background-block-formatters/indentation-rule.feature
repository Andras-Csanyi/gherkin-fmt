Feature: Indentation of Background block in a file

  A feature file formatting starts with a birds-eye view where the
  "Feature" represents a single block
  And the "Background" blocks are indented right in accordance wiht the provided
  configuration values.
  Rule: `Background` blocks are indented by level 1 to `Feature` blocks

  Scenario: Shifting right a scenario block
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And the following input is provided
    ```gherkin
    Feature: This is a test input

    Background: This is a background with the same indentation as the Feature
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Feature: This is a test input

      Background: This is a background with the same indentation as the Feature
    ```
