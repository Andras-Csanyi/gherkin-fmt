Feature: Vertical table is indented by 1 level in steps

  In `Given`,`When`, `Then` and `But` blocks the vertical table
  is indented by 1 level.

  Rule: Vertical table is indented by 1 level in steps

  Scenario: Table is not aligned
    Given the following input is provided
    ```gherkin
    Given something is given
    | header 1 | header 2 | header 3 |
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Given something is given
      | header 1 | header 2 | header 3 |
    ```

  Scenario: Table is not aligned, multiline
    Given the following input is provided
    ```gherkin
    Given something is given
    | header 1 | header 2 | header 3 |
    | what 1   | what 2   | what 3   |
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Given something is given
      | header 1 | header 2 | header 3 |
      | what 1   | what 2   | what 3   |
    ```

  Scenario: Table is aligned
    Given the following input is provided
    ```gherkin
    Given something is given
      | header 1 | header 2 | header 3 |
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Given something is given
      | header 1 | header 2 | header 3 |
    ```
  Scenario: Table is aligned, multiline
    Given the following input is provided
    ```gherkin
    Given something is given
      | header 1 | header 2 | header 3 |
      | value 1  | value 2  | value 3  |
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Given something is given
      | header 1 | header 2 | header 3 |
      | value 1  | value 2  | value 3  |
    ```

  Scenario: Table is over indented
    Given the following input is provided
    ```gherkin
    Given something is given
                | header 1 | header 2 | header 3 |
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Given something is given
      | header 1 | header 2 | header 3 |
    ```

  Scenario: Table is over indented, multiline
    Given the following input is provided
    ```gherkin
    Given something is given
                | header 1 | header 2 | header 3 |
                | value 1  | value 2  | value 3  |
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Given something is given
      | header 1 | header 2 | header 3 |
      | value 1  | value 2  | value 3  |
    ```

  Scenario: Table with mixed indentation
    Given the following input is provided
    ```gherkin
    Given something is given
                | header 1 | header 2 | header 3 |
                      | value 1  | value 2  | value 3  |
                |v 1       | v 2      | v 3      |
          |a 1       | a 2      | a 3      |
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Given something is given
      | header 1 | header 2 | header 3 |
      | value 1  | value 2  | value 3  |
      |v 1       | v 2      | v 3      |
      |a 1       | a 2      | a 3      |
    ```
