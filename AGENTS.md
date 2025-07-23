# Development Rules for String Classifiers

This repository may include multiple string classifier functions. When adding a new classifier, follow these guidelines:

1. **Provide a detection function.** Every classifier must have a function like `is_<>(input: &str) -> bool`.Place the function in `src/classifiers.rs` in a suitable module.
2. **Write a unit test.** Each classifier requires a unit test showing that a sample string matches (or does not match) the classifier, the test should check that the obfuscated text also matches the same classifier. Tests can live in the same file under a `#[cfg(test)]` module or in another test module. 
3. **Run tests.** After implementing or modifying classifiers, run `cargo test` to ensure all tests pass.

### Well-known Examples

Maintain a list of sample inputs for every classifier under `tests/well_known_inputs.rs`.
Each entry should pair an input string with the classifiers that detect it.
When adding or modifying a classifier, update this list and use it in unit tests
to verify the expected results.

### Obfuscation Rules

The default obfuscation strategy is to hash the entire input string without any
modification. The resulting hash should then be converted into a human‑readable
form, which is used to reconstruct the obfuscated text. All classifiers should
follow this rule when providing obfuscation functions.

These guidelines apply to all future classifier-related changes.
