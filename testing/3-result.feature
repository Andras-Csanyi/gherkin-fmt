Feature: Step definition formatter integration tests

  Scenario:
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

  Scenario Outline: let's test the templates and parameters
    Given not a capital start with a table
      | key  | value  |
      | key2 | value2 |
      | key3 | value3 |
    And another crazy table
      | header1           | header2               | header3      | header4     |
      | asdf              | adfasdfasdfasdfadfasd | sdfadddfdasd | lkad        |
      | asdfasdrfwafdafds | kad                   | adfasd       | jlaksdfasdf |
    When this is a <tag> and <another> <one>
    And here are <more>
    Then this is an "input parameter"
    And a more "    funky    " and "   another"
