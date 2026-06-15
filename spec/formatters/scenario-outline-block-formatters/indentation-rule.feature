Feature: Indentation of `Scenario Outline` blocks in a file

  A feature file formatting starts with a birds-eye view where the 
  "Feature" represents a single block
  and the "Scenario Outline" blocks are indented right in accordance wiht the provided
  configuration values.

  Rule: `Scenario Outline` blocks are indented by level 1 to `Feature` blocks

  Scenario: Shifting right a `Scenario Outline` block
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And the following input is provided
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
  Scenario: Shifting right two `Scenario Outline` blocks
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And the following input is provided
    ```gherkin
    Feature: This is a test input

    Scenario Outline: This is the first scenario with the same indentation as the Feature
    Scenario Outline: This is the second scenario with the same indentation as the Feature
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Feature: This is a test input

      Scenario Outline: This is the first scenario with the same indentation as the Feature
      Scenario Outline: This is the second scenario with the same indentation as the Feature
    ```
  Scenario: Aligning mixed indentation `Scenario Outline` blocks 
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And the following input is provided
    ```gherkin
    Feature: This is a test input

        Scenario Outline: This is the first scenario with the same indentation as the Feature
  Scenario Outline: This is the second scenario with the same indentation as the Feature
     Scenario Outline: This is the third scenario with the same indentation as the Feature
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Feature: This is a test input

      Scenario Outline: This is the first scenario with the same indentation as the Feature
      Scenario Outline: This is the second scenario with the same indentation as the Feature
      Scenario Outline: This is the third scenario with the same indentation as the Feature
    ```


      
