Feature: Table formatters integration

  The below scenarios show cases when the standalone formatters
  are used together

  Scenario: case 1
    Given the following input is provided
    ```gherkin
    Given this step doesn't have a table
    When this step does have a table
    |t 1|t 2|
    |asdffffff|ga|
    |gagggg|aasdfafasdfsasdfasd|
    Then we are at the end
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this step doesn't have a table
    When this step does have a table
      | t 1       | t 2                 |
      | asdffffff | ga                  |
      | gagggg    | aasdfafasdfsasdfasd |
    Then we are at the end
    ```

  Scenario: case 2
    Given the following input is provided
    ```gherkin
    Given this step doesn't have a table
      |a 1|a 2|a 3|
      |as|fassss|fasgaadfasdfawer|
      |lalalal|dfa|hlahsdhfa|
      |akld|fasd|hwoqurakdfja;sdl|
    When this step does have a table
    |t 1|t 2|
    |asdffffff|ga|
    |gagggg|aasdfafasdfsasdfasd|
    Then we are at the end
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this step doesn't have a table
      | a 1     | a 2    | a 3              |
      | as      | fassss | fasgaadfasdfawer |
      | lalalal | dfa    | hlahsdhfa        |
      | akld    | fasd   | hwoqurakdfja;sdl |
    When this step does have a table
      | t 1       | t 2                 |
      | asdffffff | ga                  |
      | gagggg    | aasdfafasdfsasdfasd |
    Then we are at the end
    ```
