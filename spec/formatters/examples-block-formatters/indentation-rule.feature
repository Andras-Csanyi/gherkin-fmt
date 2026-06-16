Feature: Indentation of `Examples` blocks in a file

  The "Feature" block is positioned to the 0 place.
  The "Scenario Outline" blocks are indented right by 1 level in accordance wiht the provided
  configuration values.
  The "Examples" block is indented by 1 level to the "Scenario Outline" block.
  Rule: `Examples` blocks are indented by level 1 to `Scenario Outline` block

  Scenario: Shifting right an `Examples` block
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And the following input is provided
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

  Scenario: Shifting right two `Examples` blocks
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And the following input is provided
    ```gherkin
    Scenario Outline: This is the first scenario with the same indentation as the Feature
    Examples:
    Examples:
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Scenario Outline: This is the first scenario with the same indentation as the Feature

    Examples:

    Examples:
    ```

  Scenario: Aligning mixed indentation `Examples` blocks 
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And the following input is provided
    ```gherkin
    Scenario Outline: This is the first scenario with the same indentation as the Feature
  Examples:
                Examples:
        Examples:
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Scenario Outline: This is the first scenario with the same indentation as the Feature

      Examples:

      Examples:

      Examples:
    ```


      
