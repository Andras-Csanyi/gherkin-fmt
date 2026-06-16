Feature: The raw text remains as it is

  Rule: the raw text remains unchanged

  Scenario: step's raw block between ``` and ```
    When a step has content enclosed between ``` and ```
    Then the enclosed content remains unchanged

  Scenario: step's raw block between """ and """
    When a step has content enclosed between """ and """
    Then the enclosed content remains unchanged
