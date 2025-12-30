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
      | datetime            | 2025-10-02 17:41:16+00:00|
      | datetime            | 2022-05-16 22:39:20-05:00|
      | datetime            | 2022-05-16T22:39:20Z     |
      | datetime            | 2025-12-29 13:18:43.470684+00:00|
      | datetime            | 2025-12-29T13:18:43.470Z |
