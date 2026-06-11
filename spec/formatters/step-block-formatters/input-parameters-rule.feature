Feature: Steps may contain input parameters between `"` and `"` characters

  A step block starts with `Given`, `When`, `Then`, `And`, `But`
  They are simple sentences
  They may contain input parameters between `"` and `"` characters

  Rule: In Step definitions the content between `"` and `"` remains as it is

  Scenario: Given step with a single input parameter and it remains as it is
    Given we have the following input
    ```gherkin
    Given we have this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Given we have this "input parameter"
    ```

  Scenario: Given step with a single input parameter and space issue
    Given we have the following input
    ```gherkin
    Given we        have this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Given we have this "input parameter"
    ```

  Scenario: Given step with a single input parameter and space issues
    Given we have the following input
    ```gherkin
    Given we        have           this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Given we have this "input parameter"
    ```

  Scenario: Given step with a single funky input parameter and space issues
    Given we have the following input
    ```gherkin
    Given we        have           this "input              parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Given we have this "input              parameter"
    ```

  Scenario: When step with a single input parameter and it remains as it is
    Given we have the following input
    ```gherkin
    When we have this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    When we have this "input parameter"
    ```

  Scenario: When step with a single input parameter and space issue
    Given we have the following input
    ```gherkin
    When we        have this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    When we have this "input parameter"
    ```

  Scenario: When step with a single input parameter and space issues
    Given we have the following input
    ```gherkin
    When we        have           this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    When we have this "input parameter"
    ```

  Scenario: When step with a single funky input parameter and space issues
    Given we have the following input
    ```gherkin
    When we        have           this "input              parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    When we have this "input              parameter"
    ```

  Scenario: Then step with a single input parameter and it remains as it is
    Given we have the following input
    ```gherkin
    Then we have this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Then we have this "input parameter"
    ```

  Scenario: Then step with a single input parameter and space issue
    Given we have the following input
    ```gherkin
    Then we        have this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Then we have this "input parameter"
    ```

  Scenario: Then step with a single input parameter and space issues
    Given we have the following input
    ```gherkin
    Then we        have           this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Then we have this "input parameter"
    ```

  Scenario: Then step with a single funky input parameter and space issues
    Given we have the following input
    ```gherkin
    Then we        have           this "input              parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Then we have this "input              parameter"
    ```

  Scenario: And step with a single input parameter and it remains as it is
    Given we have the following input
    ```gherkin
    And we have this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    And we have this "input parameter"
    ```

  Scenario: And step with a single input parameter and space issue
    Given we have the following input
    ```gherkin
    And we        have this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    And we have this "input parameter"
    ```

  Scenario: And step with a single input parameter and space issues
    Given we have the following input
    ```gherkin
    And we        have           this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    And we have this "input parameter"
    ```

  Scenario: And step with a single funky input parameter and space issues
    Given we have the following input
    ```gherkin
    And we        have           this "input              parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    And we have this "input              parameter"
    ```

  Scenario: But step with a single input parameter and it remains as it is
    Given we have the following input
    ```gherkin
    But we have this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    But we have this "input parameter"
    ```

  Scenario: But step with a single input parameter and space issue
    Given we have the following input
    ```gherkin
    But we        have this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    But we have this "input parameter"
    ```

  Scenario: But step with a single input parameter and space issues
    Given we have the following input
    ```gherkin
    But we        have           this "input parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    But we have this "input parameter"
    ```

  Scenario: But step with a single funky input parameter and space issues
    Given we have the following input
    ```gherkin
    But we        have           this "input              parameter"
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    But we have this "input              parameter"
    ```

