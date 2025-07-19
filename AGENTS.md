# Development Rules for String Classifiers

This repository may include multiple string classifier functions. When adding a new classifier, follow these guidelines:

1. **Provide a detection function.** Every classifier must have a function whose name starts with `is_` and returns an `Option<T>`. The argument is the value to examine. For string classifiers this will be `Option<&str>`, while other classifiers may return a different type (for example `Option<DateTime<Utc>>`). Place the function in `src/` in a suitable module.
2. **Write a unit test.** Each classifier requires a unit test showing that a sample string matches (or does not match) the classifier. Tests can live in the same file under a `#[cfg(test)]` module or in another test module.
3. **Run tests.** After implementing or modifying classifiers, run `cargo test` to ensure all tests pass.

These guidelines apply to all future classifier-related changes.
