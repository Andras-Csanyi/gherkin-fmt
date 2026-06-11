Feature: Column width padding

  A column is always as wide as the widest cell.
  A column content has a 1 space right padding.

  Rule: column width padding

  Scenario: header, a line and two columns
    Given the following input is provided
    ```gherkin
    Given this has a table
      | header 1| header 2|
      |value and value 1| values and values and values|
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this has a table
      | header 1          | header 2                     |
      | value and value 1 | values and values and values |
    ```

  Scenario: header, three lines and three columns 
    Given the following input is provided
    ```gherkin
    Given this has a table
      | h 1| h 2| h 3|
      |value 1| values and| more values|
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this has a table
      | h 1     | h 2        | h 3         |
      | value 1 | values and | more values |
    ```

  Scenario: header, multiple lines and three columns
    Given the following input is provided
    ```gherkin
    Given this step has a table
      | h 1| h 2| h 3|
      |value 1| values and| more values|
      |a value| a value ssssssssss| a value|
      |bcddddddd value| bc value| bc value|
      |dcccc value| dcccc value| dcccc value|
      |gggg value| gggg value| ggggdddddddddddd value|
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this step has a table
      | h 1             | h 2                | h 3                    |
      | value 1         | values and         | more values            |
      | a value         | a value ssssssssss | a value                |
      | bcddddddd value | bc value           | bc value               |
      | dcccc value     | dcccc value        | dcccc value            |
      | gggg value      | gggg value         | ggggdddddddddddd value |
    ```

