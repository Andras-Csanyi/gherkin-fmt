Feature: The `Steps` in the `Background` block are aligned and indented

  Rule: Background block steps alignment

  Scenario: The `Background` block has a not indented step
    Given there is the following input
    ```gherkin
    Background: this is a test background
    Given this is a not indented step
    ```
    When the formatter formats the file
    Then the reseult is the following
    ```gherkin
    Background: this is a test background
      Given this is a not indented step
    ```

  Scenario: The `Background` block has two not indented steps
    Given there is the following input
    ```gherkin
    Background: this is a test background
    Given this is a not indented step
    And this is a not indented step
    ```
    When the formatter formats the file
    Then the reseult is the following
    ```gherkin
    Background: this is a test background
      Given this is a not indented step
      And this is a not indented step
    ```

  Scenario: The `Background` block has three not indented steps
    Given there is the following input
    ```gherkin
    Background: this is a test background
    Given this is a not indented step
    When this is a not indented step
    Then this is a not indented step
    ```
    When the formatter formats the file
    Then the reseult is the following
    ```gherkin
    Background: this is a test background
      Given this is a not indented step
      When this is a not indented step
      Then this is a not indented step
    ```

  Scenario: The `Background` block has multiple chaotically indented steps
    Given there is the following input
    ```gherkin
    Background: this is a test background
        Given this is a not indented step
                  And here is some chaos
        When this is a not indented step
            And you want more
    Then this is a not indented step
       And here is more
    ```
    When the formatter formats the file
    Then the reseult is the following
    ```gherkin
    Background: this is a test background
      Given this is a not indented step
      And here is some chaos
      When this is a not indented step
      And you want more
      Then this is a not indented step
      And here is more
    ```


