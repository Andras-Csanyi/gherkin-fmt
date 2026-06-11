Feature: Step formatter

  A step block starts with `Given`, `When`, `Then`, `And`, `But`
  They are simple sentences
  Between the words there is only one space

  Rule: one space between words in steps

  Scenario: Simple `Given` step
    Given we have the following input
    ```gherkin
    Given everything is just fine with this step
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Given everything is just fine with this step
    ```

  Scenario: Multi space between a word in a `Given` step
    Given we have the following input
    ```gherkin
    Given this step           is not fine
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Given this step is not fine
    ```

  Scenario: Multi multi space between a word in a `Given` step
    Given we have the following input
    ```gherkin
    Given this            step           is not           fine
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Given this step is not fine
    ```

  Scenario: Simple `When` step
    Given we have the following input
    ```gherkin
    When everything is just fine with this step
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    When everything is just fine with this step
    ```

  Scenario: Multi space between a word in a `Given` step
    Given we have the following input
    ```gherkin
    When this step           is not fine
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    When this step is not fine
    ```

  Scenario: Multi space between a word in a `Given` step
    Given we have the following input
    ```gherkin
    When this          step           is not           fine
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    When this step is not fine
    ```

  Scenario: Simple `Then` step
    Given we have the following input
    ```gherkin
    Then everything is just fine with this step
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Then everything is just fine with this step
    ```

  Scenario: Multi space between a word in a `Then` step
    Given we have the following input
    ```gherkin
    Then this step           is not fine
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Then this step is not fine
    ```

  Scenario: Multi space between a word in a `Then` step
    Given we have the following input
    ```gherkin
    Then this          step           is not              fine
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Then this step is not fine
    ```

  Scenario: Simple `And` step
    Given we have the following input
    ```gherkin
    And everything is just fine with this step
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    And everything is just fine with this step
    ```

  Scenario: Multi space between a word in a `And` step
    Given we have the following input
    ```gherkin
    And this step           is not fine
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    And this step is not fine
    ```

  Scenario: Multi multi space between a word in a `And` step
    Given we have the following input
    ```gherkin
    And this             step           is not            fine
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    And this step is not fine
    ```

  Scenario: Simple `But` step
    Given we have the following input
    ```gherkin
    But everything is just fine with this step
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    But everything is just fine with this step
    ```

  Scenario: Multi multi space between a word in a `But` step
    Given we have the following input
    ```gherkin
    But this                step           is not              fine
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    But this step is not fine
    ```

  Scenario: Multi multi multi space between a word in a `But` step
    Given we have the following input
    ```gherkin
    Given this is a proper given statement
    And this        is       not really
    And this                step           is not              fine either
    But this one is fine again
    When I try to read               these
    And I       really       try 
    Then I wish I had a good formatter
    And I would read these lines easily
    But not        these        ones
    ```
    When the formatter formats the file
    Then the result is
    ```gherkin
    Given this is a proper given statement
    And this is not really
    And this step is not fine either
    But this one is fine again
    When I try to read these
    And I really try 
    Then I wish I had a good formatter
    And I would read these lines easily
    But not these ones


