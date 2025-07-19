# Development Rules for String Classifiers

This repository may include multiple string classifier functions. When adding a new classifier, follow these guidelines:

1. **Provide a detection function.** Every classifier must have a function like `try_<>(input: &str) -> Option<whatever>`.Place the function in `src/classifiers.rs` in a suitable module.
2. **Write a unit test.** Each classifier requires a unit test showing that a sample string matches (or does not match) the classifier, the test should check that the obfuscated text also matches the same classifier. Tests can live in the same file under a `#[cfg(test)]` module or in another test module. 
3. **Run tests.** After implementing or modifying classifiers, run `cargo test` to ensure all tests pass.

These guidelines apply to all future classifier-related changes.
