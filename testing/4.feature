
    Feature: test feature





Background: test background
  Given this is a not indented step
       And another one
         |header1|value1|
         |header2|value2|
         |header3|value3|
    When another unindented step
         |header1|value1|
         |header2|value2|
         |header3|value3|
             And another one again
    Then once more
         |header1|value1|
         |header2|value2|
         |header3|value3|
And why not another one again

scenario outline: massive chaos comes here
given this is given
and some shitty table
  |key|value|
  |key2 adfa  afasdf|value2|
  |key3|value3 adfa fasdf asdfa|
and "input parameter" with <template >
and more
  |key|value| val|
  |key2 adfa  afasdf|value2| adfas |
  |key3|value3 adfa fasdf asdfa|kkkkkkk|
  |key3lkjlkalkfasdlkj|value3 adfa fasdf asdfa|kkkkkkk|
When the when is correct
and the <  template > with "input"
  |key|value| val|fourth|
  |key2 adfa  afasdf|value2| adfas |22|
  |key3|value3 adfa fasdf asdfa|kkkkkkk| lkadf|
  |key3lkjlkalkfasdlkj|value3 adfa fasdf asdfa|kkkkkkk| adfa|
Then more template <t1> and <t2>
examples:
|template|t1|t2|
|atemplate|t1|t2|
|atemplate|adfadt1|t2|
|atemplate|adfadt1|tadfadf2|
|atadfadfasdfte|addafdadsfadsfafadt1|tadfadf2|

