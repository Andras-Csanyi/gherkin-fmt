Feature: test feature

  Background: test background
    Given this is a not indented step
      | header1 | value1 |
      | header2 | value2 |

  Scenario Outline: malformed
    Given <template comes here
    When this is formatted 
    Then this one is interesting

    Examples:
      | asd    | aasd        |
      | aaaaaa | ddddddddddd |

  Scenario Outline: malformed second
    Given <template comes here
    When this is> formatted 
    Then this <one is> interesting

    Examples:
      | asd    | aasd        |
      | aaaaaa | ddddddddddd |