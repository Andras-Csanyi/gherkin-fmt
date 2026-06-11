Feature: The cli prints out the version

  There are cases when we just have to know the version of the cli.

  Scenario: Print the version when the `--version` parameter is provided
    Given I have the project compiled successfully
    When I execute the following command
    ```bash
    gherkinfmt --version
    ```
    Then the cli prints out the version number

