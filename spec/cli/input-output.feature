Feature: Input and output files

  Scenario: Input file is read from the specified path and wrote to a specific path
    Given there is the `test.feature` file
    And it has the following content
    ```gherkin
    Feature: test feature
    Misaligned user story part
    ```
    When the following command is executed
    ```bash
    gherkinfmt --output test-result.feature test.feature
    ```
    Then the `test-result.feature` has been created or overwritten
    And its content is the following
    ```gherkin
    Feature: test feature
  
      Misaligned user story part
    ```

