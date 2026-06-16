Feature: Step formatter

  A step block starts with `Given`, `When`, `Then`, `And`, `But`
  They are simple sentences
  They can contain template parameters between `<` and `>` characters
  Rule: Template parameters in Step definitionsl

  Scenario: Correct template parameter in Given
    Given we have the following input
    ```gherkin
    Given this has the <parameter> parameter
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this has the <parameter> parameter
    ```

  Scenario: Left side spacing issue template parameter in Given
    Given we have the following input
    ```gherkin
    Given this has the < parameter> parameter
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this has the <parameter> parameter
    ```

  Scenario: Right side spacing issue template parameter in Given
    Given we have the following input
    ```gherkin
    Given this has the <parameter > parameter
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this has the <parameter> parameter
    ```

  Scenario: Chaos spacing issue template parameter in Given
    Given we have the following input
    ```gherkin
    Given this has the <       parameter       > parameter
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this has the <parameter> parameter
    ```

  Scenario: Correct template parameter in Given
    Given we have the following input
    ```gherkin
    Given this has the <parameter> parameter and another <one>
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this has the <parameter> parameter and another <one>
    ```

  Scenario: Left side spacing issue template parameter in Given
    Given we have the following input
    ```gherkin
    Given this has the < parameter> parameter and another <one >
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this has the <parameter> parameter and another <one>
    ```

  Scenario: Right side spacing issue template parameter in Given
    Given we have the following input
    ```gherkin
    Given this has the <parameter > parameter and another < one>
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this has the <parameter> parameter and another <one>
    ```

  Scenario: Chaos spacing issue template parameter in Given
    Given we have the following input
    ```gherkin
    Given this has the <       parameter       > parameter and another <      one     >
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Given this has the <parameter> parameter and another <one>
    ```

  Scenario: Correct template parameter in When
    Given we have the following input
    ```gherkin
    When this has the <parameter> parameter
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    When this has the <parameter> parameter
    ```

  Scenario: Left side spacing issue template parameter in When
    Given we have the following input
    ```gherkin
    When this has the < parameter> parameter
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    When this has the <parameter> parameter
    ```

  Scenario: Right side spacing issue template parameter in When
    Given we have the following input
    ```gherkin
    When this has the <parameter > parameter
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    When this has the <parameter> parameter
    ```

  Scenario: Chaos spacing issue template parameter in When
    Given we have the following input
    ```gherkin
    When this has the <       parameter       > parameter
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    When this has the <parameter> parameter
    ```

  Scenario: Correct template parameter in When
    Given we have the following input
    ```gherkin
    When this has the <parameter> parameter and another <one>
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    When this has the <parameter> parameter and another <one>
    ```

  Scenario: Left side spacing issue template parameter in When
    Given we have the following input
    ```gherkin
    When this has the < parameter> parameter and another <one >
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    When this has the <parameter> parameter and another <one>
    ```

  Scenario: Right side spacing issue template parameter in When
    Given we have the following input
    ```gherkin
    When this has the <parameter > parameter and another < one>
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    When this has the <parameter> parameter and another <one>
    ```

  Scenario: Chaos spacing issue template parameter in When
    Given we have the following input
    ```gherkin
    When this has the <       parameter       > parameter and another <      one     >
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    When this has the <parameter> parameter and another <one>
    ```

  Scenario: Correct template parameter in Then
    Given we have the following input
    ```gherkin
    Then this has the <parameter> parameter
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Then this has the <parameter> parameter
    ```

  Scenario: Left side spacing issue template parameter in Then
    Given we have the following input
    ```gherkin
    Then this has the < parameter> parameter
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Then this has the <parameter> parameter
    ```

  Scenario: Right side spacing issue template parameter in Then
    Given we have the following input
    ```gherkin
    Then this has the <parameter > parameter
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Then this has the <parameter> parameter
    ```

  Scenario: Chaos spacing issue template parameter in Then
    Given we have the following input
    ```gherkin
    Then this has the <       parameter       > parameter
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Then this has the <parameter> parameter
    ```

  Scenario: Correct template parameter in Then
    Given we have the following input
    ```gherkin
    Then this has the <parameter> parameter and another <one>
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Then this has the <parameter> parameter and another <one>
    ```

  Scenario: Left side spacing issue template parameter in Then
    Given we have the following input
    ```gherkin
    Then this has the < parameter> parameter and another <one >
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Then this has the <parameter> parameter and another <one>
    ```

  Scenario: Right side spacing issue template parameter in Then
    Given we have the following input
    ```gherkin
    Then this has the <parameter > parameter and another < one>
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Then this has the <parameter> parameter and another <one>
    ```

  Scenario: Chaos spacing issue template parameter in Then
    Given we have the following input
    ```gherkin
    Then this has the <       parameter       > parameter and another <      one     >
    ```
    When the formatter formats the file
    Then the result is the following
    ```gherkin
    Then this has the <parameter> parameter and another <one>
    ```
