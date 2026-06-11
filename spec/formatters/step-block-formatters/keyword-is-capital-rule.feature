Feature: Step definition keywords are formatted to be capital letters

  The `Given`, `When`, `Then`, `And` and `But` step definitions keywords
  always must start with capital letters.
  It just looks better and leads the eye.

  Rule: Step definition keywords start with capital letters

  Scenario: The correct given flows through
    Given we have the following input
    ```gherkin
    Given this is a correct one
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this is a correct one
    ```

  Scenario: Given starts with capital letter
    Given we have the following input
    ```gherkin
    given this is a correct one
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    given this is a correct one
    ```

  Scenario: Not keyword `given` remains unchanged
    Given we have the following input
    ```gherkin
    Given this step has another given
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this step has another given
    ```

  Scenario: Not keyword `given` remains unchanged but the incorrect one is formatter
    Given we have the following input
    ```gherkin
    given this step has another given
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this step has another given
    ```

  Scenario: The correct when flows through
    Given we have the following input
    ```gherkin
    When this is a correct one
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    When this is a correct one
    ```

  Scenario: When starts with capital letter
    Given we have the following input
    ```gherkin
    when this is a correct one
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    When this is a correct one
    ```

  Scenario: Not keyword `when` remains unchanged
    Given we have the following input
    ```gherkin
    When and here is another when 
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    When and here is another when 
    ```

  Scenario: Not keyword `when` remains unchanged but the incorrect one is formatter
    Given we have the following input
    ```gherkin
    when this step has another when
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    When this step has another when
    ```

  Scenario: The correct then flows through
    Given we have the following input
    ```gherkin
    Then this is a correct one
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Then this is a correct one
    ```

  Scenario: Then starts with capital letter
    Given we have the following input
    ```gherkin
    then this is a correct one
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Then this is a correct one
    ```

  Scenario: Not keyword `then` remains unchanged
    Given we have the following input
    ```gherkin
    Then and here is another then 
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Then and here is another then 
    ```

  Scenario: Not keyword `then` remains unchanged but the incorrect one is formatter
    Given we have the following input
    ```gherkin
    then this step has another then
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Then this step has another then
    ```

  Scenario: The correct And flows through
    Given we have the following input
    ```gherkin
    And this is a correct one
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    And this is a correct one
    ```

  Scenario: And starts with capital letter
    Given we have the following input
    ```gherkin
    and this is a correct one
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    And this is a correct one
    ```

  Scenario: Not keyword `and` remains unchanged
    Given we have the following input
    ```gherkin
    And and here is another and
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    And and here is another and
    ```

  Scenario: Not keyword `and` remains unchanged but the incorrect one is formatter
    Given we have the following input
    ```gherkin
    and this step has another and
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    And this step has another and
    ```

  Scenario: The correct But flows through
    Given we have the following input
    ```gherkin
    But this is a correct one
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    But this is a correct one
    ```

  Scenario: But starts with capital letter
    Given we have the following input
    ```gherkin
    but this is a correct one
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    But this is a correct one
    ```

  Scenario: Not keyword `but` remains unchanged
    Given we have the following input
    ```gherkin
    But and here is another but
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    But and here is another but
    ```

  Scenario: Not keyword `but` remains unchanged but the incorrect one is formatter
    Given we have the following input
    ```gherkin
    but this step has another but
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    But this step has another but
    ```
