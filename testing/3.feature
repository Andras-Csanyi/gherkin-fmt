Feature: Step definition formatter integration tests

  Scenario:
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


    scenario outline: let's test the templates and parameters
    given not a capital start with a table
      |key|value|
      |key2|value2|
      |key3|value3|
    and another crazy table
      |header1|header2|header3|header4|
      |asdf|adfasdfasdfasdfadfasd|sdfadddfdasd|lkad|
      |asdfasdrfwafdafds|kad|adfasd|jlaksdfasdf|
    When this is a < tag > and <      another> <one    >
    and here are <more>
    Then this is an "input parameter"
    and a more "    funky    " and "   another"
