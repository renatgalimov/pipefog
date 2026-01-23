Feature: Obfuscation preserves classification

  Scenario Outline: obfuscate <detector>
    Given the input "<input>"
    When I obfuscate it as <detector>
    Then the result is a valid <detector>

    Examples:
      | detector            | input                                                        |
      | alpha_word          | lowercase                                                    |
      | alpha_word          | abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz         |
      | uppercase_word      | UPPERCASE                                                    |
      | uppercase_word      | ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ         |
      | capitalized_word    | Capitalized                                                  |
      | capitalized_word    | Verylongcapitalizedwordtotestcyclinglogic                    |
      | snake_case_word     | snake_case_word                                              |
      | snake_case_word     | test_string_with_consonants                                  |
      | snake_case_word     | __leading_underscores__                                      |
      | snake_case_word     | a_b_c_d_e_f_g                                                |
      | snake_case_word     | very_long_snake_case_identifier_name                         |
      | base32_lowercase    | mfrggzdfmztwq2lknnwg23tp                                     |
      | base32_lowercase    | mfrggzdfmztwq2lknnwg23tpmfrggzdfmztwq2lknnwg23tpmfrggzdfmztw |
      | base32_uppercase    | MFRGGZDFMZTWQ2LKNNWG23TP                                     |
      | base32_uppercase    | MFRGGZDFMZTWQ2LKNNWG23TPMFRGGZDFMZTWQ2LKNNWG23TPMFRGGZDFMZTW |
      | datetime            | 2025-10-02 17:41:16+00:00                                    |
      | datetime            | 2022-05-16 22:39:20-05:00                                    |
      | datetime            | 2022-05-16T22:39:20Z                                         |
      | datetime            | 2025-12-29 13:18:43.470684+00:00                             |
      | datetime            | 2025-12-29T13:18:43.470Z                                     |
      | email               | user@example.com                                             |
      | email               | john.doe@company.org                                         |
      | email               | test-user_123@sub.domain.io                                  |
