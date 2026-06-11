Feature: One space left padding in the cell

  Every cell in the table has one space left padding for readability

  Rule: One space left padding in the cell

  Scenario: Simple case, one line and three columns
    Given the following input is provided
    ```gherkin
    Given this step includes a vertical table
    |header 1|header 2|header 3|
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Given this step includes a vertical table
      | header 1| header 2| header 3|
    ```

  Scenario: Two lines and three columns
    Given the following input is provided
    ```gherkin
    Given this step includes a vertical table
    |header 1|header 2|header 3|
    |value 1|value 2|value 3|
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Given this step includes a vertical table
    | header 1| header 2| header 3|
    | value 1| value 2| value 3|
    ```

  Scenario: Two lines and four columns
    Given the following input is provided
    ```gherkin
    Given this step includes a vertical table
    |header 1|header 2|header 3|header 4|
    |value 1|value 2|value 3|value 4|
    ```
    When the formatter formats the file
    Then the output is the following
    ```gherkin
    Given this step includes a vertical table
    | header 1| header 2| header 3| header 4|
    | value 1| value 2| value 3| value 4|
    ```

