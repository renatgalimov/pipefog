Feature: Obfuscation preserves classification

  Scenario Outline: obfuscate <detector>
    Given the input "<input>"
    When I obfuscate it as <detector>
    Then the result is a valid <detector> and equals "<expected>"

    Examples:
      | detector            | input                                                        | expected                                                   |
      | alpha_word          | lowercase                                                    | owokakown                                                  |
      | alpha_word          | abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz         | havadplachishadgutichavtwohisuexegbefgivaftthembouse       |
      | uppercase_word      | UPPERCASE                                                    | DEOLIADUP                                                  |
      | uppercase_word      | ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ         | ATAPUGHOTHODWHENETOLDECOMMEFVANVERHETMANTIMELFECTNOW       |
      | capitalized_word    | Capitalized                                                  | Commesmenag                                                |
      | capitalized_word    | Verylongcapitalizedwordtotestcyclinglogic                    | Beappunginstizansthiscontquifpartarlmanbu                  |
      | snake_case_word     | snake_case_word                                              | entsam_mustsho_ag                                          |
      | snake_case_word     | test_string_with_consonants                                  | owso_vanop_actin                                           |
      | snake_case_word     | __leading_underscores__                                      | __givect_acib_illig_what__                                 |
      | snake_case_word     | a_b_c_d_e_f_g                                                | ett_enc                                                    |
      | snake_case_word     | very_long_snake_case_identifier_name                         | ardcont_oen_acqu_illed_vanex_teil_untach                   |
      | base32_lowercase    | mfrggzdfmztwq2lknnwg23tp                                     | tzfksjm4zv4eojieh5lvbqcq                                   |
      | base32_lowercase    | mfrggzdfmztwq2lknnwg23tpmfrggzdfmztwq2lknnwg23tpmfrggzdfmztw | 3g7g35ftet2vnop3wndan44vqlcukrfhnqe6bzahgkiz5cppz2ylzmascuar |
      | base32_uppercase    | MFRGGZDFMZTWQ2LKNNWG23TP                                     | THF3P3YWUPIWVM6U7JDGG6LD                                   |
      | base32_uppercase    | MFRGGZDFMZTWQ2LKNNWG23TPMFRGGZDFMZTWQ2LKNNWG23TPMFRGGZDFMZTW | V3Z7RK33DEWK4CC56FTYTUBGRCLNZEXAFDTOZDIN34MHKGL252U32AJSGQRI |
      | datetime            | 2025-10-02 17:41:16+00:00                                    | 2000-01-01 00:00:00+00:00                                  |
      | datetime            | 2022-05-16 22:39:20-05:00                                    | 1999-12-31 19:00:00-05:00                                  |
      | datetime            | 2022-05-16T22:39:20Z                                         | 2000-01-01T00:00:00Z                                       |
      | datetime            | 2026-01-25 14:30:00-0500                                     | 1999-12-31 19:00:00-0500                                   |
      | datetime            | 2025-12-29 13:18:43.470684+00:00                             | 2000-01-01 00:00:00.000000+00:00                           |
      | datetime            | 2025-12-29T13:18:43.470Z                                     | 2000-01-01T00:00:00.000Z                                   |
      | datetime            | 2026-01-25T14:30:00.123456-0500                              | 1999-12-31T19:00:00.000000-0500                            |
      | email               | user@example.com                                             | ellar30@grethe4f.com                                       |
      | email               | john.doe@company.org                                         | allbe28@estsom7b.info                                      |
      | email               | test-user_123@sub.domain.io                                  | proje69@thatendc.gov                                       |
