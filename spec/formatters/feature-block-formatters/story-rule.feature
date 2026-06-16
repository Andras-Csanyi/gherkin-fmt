Feature: The story content of a feature block is indented by 1 level

  A feature block may contain the user story
  Right here where this text is located
  This content also needs to be indented by 1 level
  Rule: `Feature` block's story is indented by 1 level

  Scenario: one line user story is aligned
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And we have the following input
    ```gherkin
    Feature: this is a feature block

    This is the unaligned single line user story
    ```
    When the content is formatted
    Then the result is the following
    ```gherkin
    Feature: this is a feature block

      This is the unaligned single line user story
    ```

  Scenario: two lines user story is aligned
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And we have the following input
    ```gherkin
    Feature: this is a feature block

    This is the first unaligned line
    This is the second unaligned line
    ```
    When the content is formatted
    Then the result is the following
    ```gherkin
    Feature: this is a feature block

      This is the first unaligned line
      This is the second unaligned line
    ```

  Scenario: multi line user story is aligned
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And we have the following input
    ```gherkin
    Feature: this is a feature block

    This is the first unaligned line
    This is the second unaligned line
    This is the third unaligned line
    This is the fourth unaligned line
    ```
    When the content is formatted
    Then the result is the following
    ```gherkin
    Feature: this is a feature block

      This is the first unaligned line
      This is the second unaligned line
      This is the third unaligned line
      This is the fourth unaligned line
    ```

  Scenario: overindented one line user story is aligned
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And we have the following input
    ```gherkin
    Feature: this is a feature block

          This is the unaligned single line user story
    ```
    When the content is formatted
    Then the result is the following
    ```gherkin
    Feature: this is a feature block

      This is the unaligned single line user story
    ```

  Scenario: overindented two lines user story is aligned
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And we have the following input
    ```gherkin
    Feature: this is a feature block

        This is the first unaligned line
        This is the second unaligned line
    ```
    When the content is formatted
    Then the result is the following
    ```gherkin
    Feature: this is a feature block

      This is the first unaligned line
      This is the second unaligned line
    ```

  Scenario: overindented multi line user story is aligned
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And we have the following input
    ```gherkin
    Feature: this is a feature block

            This is the first unaligned line
            This is the second unaligned line
            This is the third unaligned line
            This is the fourth unaligned line
    ```
    When the content is formatted
    Then the result is the following
    ```gherkin
    Feature: this is a feature block

      This is the first unaligned line
      This is the second unaligned line
      This is the third unaligned line
      This is the fourth unaligned line
    ```

  Scenario: mix indented multi line user story is aligned
    Given the following configuration values are provided
      | indent-size | 2 spaces |
    And we have the following input
    ```gherkin
    Feature: this is a feature block

    This is the first unaligned line
            This is the second unaligned line
          This is the third unaligned line
                    This is the fourth unaligned line
    ```
    When the content is formatted
    Then the result is the following
    ```gherkin
    Feature: this is a feature block

      This is the first unaligned line
      This is the second unaligned line
      This is the third unaligned line
      This is the fourth unaligned line
    ```
