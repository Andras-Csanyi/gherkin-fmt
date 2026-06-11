Feature: `Feature` block formatting integration tests

  Scenario: Format the `Feature` block
    Given the following input
    ```gherkin
    # this should be preserver
    Feature: this is a test feature
    unindented user story
    ```
    When the formatter is executed
    Then the result is the following
    ```gherkin
    # this should be preserver
    Feature: this is a test feature

      unindented user story
    ```

  Scenario: Format the `Feature` block
    Given the following input
    ```gherkin
    # this should be preserver
    # this should be preserver
    Feature: this is a test feature
    unindented user story
        badly indented user story
      again a badly indented line
    ```
    When the formatter is executed
    Then the result is the following
    ```gherkin
    # this should be preserver
    # this should be preserver
    Feature: this is a test feature

      unindented user story
      badly indented user story
      again a badly indented line
    ```

  Scenario: Format the `Feature` block
    Given the following input
    ```gherkin
    # this should be preserver
    # this should be preserver

        Feature: this is a badly indented feature




    unindented user story
        badly indented user story
      again a badly indented line
          again a badly indented line
    ```
    When the formatter is executed
    Then the result is the following
    ```gherkin
    # this should be preserver
    # this should be preserver

    Feature: this is a test feature

      unindented user story
      badly indented user story
      again a badly indented line
      again a badly indented line
    ```
