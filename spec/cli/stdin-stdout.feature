Feature: reading from stdin and writing to stdout

  Scenario: Input is read from stdin and written to stdout
    When the following command is executed
    ```bash
    echo "feature: test feature" | gherkin-fmt -- -
    ```
    Then the result is
    ```bash
    Feature: test feature
    ```
