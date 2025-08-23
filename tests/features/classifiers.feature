Feature: Obfuscation preserves classification

  Scenario Outline: obfuscate <detector>
    Given the input "<input>"
    When I obfuscate it as <detector>
    Then the result is a valid <detector>

    Examples:
      | detector            | input                    |
      | alpha_word          | lowercase                |
      | uppercase_word      | UPPERCASE                |
      | capitalized_word    | Capitalized              |
      | snake_case_word     | snake_case_word          |
      | base32_lowercase    | mfrggzdfmztwq2lknnwg23tp |
      | base32_uppercase    | MFRGGZDFMZTWQ2LKNNWG23TP |
