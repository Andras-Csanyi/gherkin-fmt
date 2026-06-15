Feature: The `Steps` in the `Scenario Outline` block are aligned and indented

  Rule: Scenario Outline block steps alignment

  Scenario: The `Scenario Outline` block has a not indented step
    Given there is the following input
    ```gherkin
    Scenario Outline: this is a test scenario outline
    Given this is a not indented step
    ```
    When the formatter formats the file
    Then the reseult is the following
    ```gherkin
    Scenario Outline: this is a test scenario outline
      Given this is a not indented step
    ```

  Scenario: The `Scenario Outline` block has two not indented steps
    Given there is the following input
    ```gherkin
    Scenario Outline: this is a test scenario
    Given this is a not indented step
    And this is a not indented step
    ```
    When the formatter formats the file
    Then the reseult is the following
    ```gherkin
    Scenario Outline: this is a test scenario
      Given this is a not indented step
      And this is a not indented step
    ```

  Scenario: The `Scenario Outline` block has three not indented steps
    Given there is the following input
    ```gherkin
    Scenario Outline: this is a test scenario
    Given this is a not indented step
    When this is a not indented step
    Then this is a not indented step
    ```
    When the formatter formats the file
    Then the reseult is the following
    ```gherkin
    Scenario Outline: this is a test scenario
      Given this is a not indented step
      When this is a not indented step
      Then this is a not indented step
    ```

  Scenario: The `Scenario Outline` block has multiple chaotically indented steps
    Given there is the following input
    ```gherkin
    Scenario Outline: this is a test scenario
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
    Scenario Outline: this is a test scenario
      Given this is a not indented step
      And here is some chaos
      When this is a not indented step
      And you want more
      Then this is a not indented step
      And here is more
    ```


