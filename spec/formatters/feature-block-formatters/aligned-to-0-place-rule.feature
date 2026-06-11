Feature: the "Feature" block is aligned to the 0 place in its line

  The "Feature" block is aligned to the 0 place in its line

  Rule: `Feature` block is aligned to 0 place

  Scenario: Feature block that is already at 0 place remains unchanged
    Given we have the following input
    ```gherkin
    Feature: this is already at the 0 place
    ```
    When the content is formatted
    Then the result is the following
    ```gherkin
    Feature: this is already at the 0 place
    ```
  Scenario: A Feature block that is at place 2 is aligned to 0
    Given we have the following input
    ```gherkin
      Feature: this block is at place 2
    ```
    When the content is formatted
    Then the result is the following
    ```gherkin
    Feature: this block is at place 2
    ```
  Scenario: A Feature block that is at place 3 is aligned to 0
    Given we have the following input
    ```gherkin
       Feature: this block is at place 3
    ```
    When the content is formatted
    Then the result is the following
    ```gherkin
    Feature: this block is at place 3
    ```
  Scenario: A Feature block that is at place 4 is aligned to 0
    Given we have the following input
    ```gherkin
        Feature: this block is at place 4
    ```
    When the content is formatted
    Then the result is the following
    ```gherkin
    Feature: this block is at place 4
    ```
