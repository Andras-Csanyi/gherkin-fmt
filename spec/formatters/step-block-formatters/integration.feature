Feature: Step block formatters integration test

  Scenario: Integration test
    Given the following input
    ```gherkin
    given this must be formatted
    But this given should remain as it is
    but this but also should be formatter
    And here are the "input      parameters" things
    And here are they again         " input     params"
    and this and too but not the second one
    And this remains as it is
    When this is just fine
    and this is template <parameter>
    but this one is malformed < para>
    and this one more <      para        >
    and this one is not
    But this is a good but
    then this then should be formatter
    But not the second one
    And this line has multiple <   input >         <     parameters     >
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this must be formatted
    But this given should remain as it is
    But this but also should be formatter
    And here are the "input      parameters" things
    And here are they again " input     params"
    And this and too but not the second one
    And this remains as it is
    When this is just fine
    And this is template <parameter>
    But this one is malformed <para>
    And this one more <para>
    And this one is not
    But this is a good but
    Then this then should be formatter
    But not the second one
    And this line has multiple <input> <parameters>
    ```
