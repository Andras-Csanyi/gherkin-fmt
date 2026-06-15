
Feature: test feature

  Background: test background
    Given this is a not indented step
    And another one
      | header1 | value1 |
      | header2 | value2 |
      | header3 | value3 |
    When another unindented step
      | header1 | value1 |
      | header2 | value2 |
      | header3 | value3 |
    And another one again
    Then once more
      | header1 | value1 |
      | header2 | value2 |
      | header3 | value3 |
    And why not another one again

  Scenario Outline: massive chaos comes here
    Given this is given
    And some shitty table
      | key               | value                   |
      | key2 adfa  afasdf | value2                  |
      | key3              | value3 adfa fasdf asdfa |
    And "input parameter" with <template>
    And more
      | key                 | value                   | val     |
      | key2 adfa  afasdf   | value2                  | adfas   |
      | key3                | value3 adfa fasdf asdfa | kkkkkkk |
      | key3lkjlkalkfasdlkj | value3 adfa fasdf asdfa | kkkkkkk |
    When the when is correct
    And the <template> with "input"
      | key                 | value                   | val     | fourth |
      | key2 adfa  afasdf   | value2                  | adfas   | 22     |
      | key3                | value3 adfa fasdf asdfa | kkkkkkk | lkadf  |
      | key3lkjlkalkfasdlkj | value3 adfa fasdf asdfa | kkkkkkk | adfa   |
    Then more template <t1> and <t2>

Examples:
  | template       | t1                   | t2       |
  | atemplate      | t1                   | t2       |
  | atemplate      | adfadt1              | t2       |
  | atemplate      | adfadt1              | tadfadf2 |
  | atadfadfasdfte | addafdadsfadsfafadt1 | tadfadf2 |