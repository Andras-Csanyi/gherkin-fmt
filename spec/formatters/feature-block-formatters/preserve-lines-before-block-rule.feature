Feature: Feature alignment preserves lines before it

  There are cases when the feature file may contain additional information
  before the Feature block, for example: comments.
  The formatter must preserve these lines.
  The formatter must not be confused if these lines contain Gherkin specific keywords.
  Rule: preserve lines before `Feature` block

  Scenario: Preserving a single line content before the "Feature:"  block
    Given we have the following input
    ```gherkin
    # this is some comment
    Feature: this is some testing input
    ```
    When the content is formatted 
    Then result is the following
    ```gherkin
    # this is some comment
    Feature: this is some testing input
    ```

  Scenario: Preserving multiple lines content before the "Feature:"  block
    Given we have the following input
    ```gherkin
    # this is the first line
    # this is the second line
    Feature: this is some testing input
    ```
    When the content is formatted 
    Then result is the following
    ```gherkin
    # this is the first line
    # this is the second line
    Feature: this is some testing input
    ```
