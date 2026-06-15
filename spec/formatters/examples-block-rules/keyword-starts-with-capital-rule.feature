Feature: the `Examples` keyword is capitalized

  Rule: the `Examples` keyword is capitalized

  Scenario: The `Examples` keyword is not capitalized
    Given the following input is provided
    ```gherkin
    examples: this is a test examples
    ```
    When the formatter formats the file
    Then the output is the followingn
    ```gherkin
    Examples: this is a test examples
    ```

  Scenario: The `Examples` keyword is capitalized
    Given the following input is provided
    ```gherkin
    Examples: this is a test examples
    ```
    When the formatter formats the file
    Then the output is the followingn
    ```gherkin
    Examples: this is a test examples
    ```


