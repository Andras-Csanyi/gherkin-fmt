Feature: When no parameter provided the help page is printed out

  Scenario: Print the help info when no parameters provided
    Given I have the project compiled successfully
    When I execute the following command
    ```bash
    gherkinfmt
    ```
    Then the result is identical with the output of the following command
    ```bash
    gherkinfmt --help
    ```

  Scenario: Print the help info when the `--help` parameter is provided
    Given I have the project compiled successfully
    When I execute the following command
    ```bash
    gherkinfmt --help
    ```
    Then the help page of the cli is printed out

