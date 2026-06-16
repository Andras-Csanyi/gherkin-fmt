Feature: Indentation of Scenario blocks in a file

  A feature file formatting starts with a birds-eye view where the
  "Feature" represents a single block
  And the "Scenario" blocks are indented right in accordance wiht the provided
  configuration values.
  Rule: `Scenario` blocks are indented by level 1 to `Feature` blocks

  Scenario: Shifting right a scenario block
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And the following input is provided
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

  Scenario: Shifting right two scenario blocks
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And the following input is provided
    ```gherkin
    Feature: This is a test input

    Scenario: This is the first scenario with the same indentation as the Feature
    Scenario: This is the second scenario with the same indentation as the Feature
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Feature: This is a test input

      Scenario: This is the first scenario with the same indentation as the Feature
      Scenario: This is the second scenario with the same indentation as the Feature
    ```

  Scenario: Aligning mixed indentation Scenario blocks 
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And the following input is provided
    ```gherkin
    Feature: This is a test input

        Scenario: This is the first scenario with the same indentation as the Feature
  Scenario: This is the second scenario with the same indentation as the Feature
     Scenario: This is the third scenario with the same indentation as the Feature
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Feature: This is a test input

      Scenario: This is the first scenario with the same indentation as the Feature
      Scenario: This is the second scenario with the same indentation as the Feature
      Scenario: This is the third scenario with the same indentation as the Feature
    ```


      
