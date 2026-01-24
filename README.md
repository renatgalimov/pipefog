[![Rust Tests](https://github.com/renatgalimov/pipefog/actions/workflows/tests.yml/badge.svg)](https://github.com/renatgalimov/pipefog/actions/workflows/tests.yml)
[![Lint](https://github.com/renatgalimov/pipefog/actions/workflows/lint.yml/badge.svg)](https://github.com/renatgalimov/pipefog/actions/workflows/lint.yml)
[![Coverage](https://github.com/renatgalimov/pipefog/actions/workflows/coverage.yml/badge.svg)](https://github.com/renatgalimov/pipefog/actions/workflows/coverage.yml)
----

<img width="1280" height="640" alt="pipefog-social" src="https://github.com/user-attachments/assets/a210fa94-fe76-49c3-9b14-7d21bd2c4281" />

# pipefog

**pipefog** – Stream‑structured data obfuscator for JSON/YAML.

🔒 Mask your sensitive data while preserving format and structure. Designed for CLI pipelines and integration with tools like `jq` and `yq`.

---

## 🚀 Example Usage

```bash
cat secrets.json | jq . | pipefog | jq .
```

## ✨ Features

- 🔐 Deterministic obfuscation – ID fields, usernames, and keys are hashed in a consistent format.
- 🏷️ Shape-preserving – Keeps field order, numeric values, and categories untouched.
- 📏 Length-preserving – Output matches input length by default, at the cost of reduced entropy.
- 🧩 Supports JSON and YAML – Auto-detects format or allow override with --format.
- 🛠️ Composable CLI – Works seamlessly in pipelines with jq, yq, and other Unix tools.
- 🗓️ ISO 8601 datetime obfuscation – Shifts dates relative to runtime baselines while preserving format.

`pipefog` detects datetimes in the `YYYY-MM-DDTHH:MM:SSZ` form. A random baseline between
1970-01-01 and the current date is chosen at startup. The first encountered datetime sets an
original baseline. Every subsequent datetime is shifted relative to these baselines so the output
remains a valid ISO 8601 `Z` datetime while preserving relative differences.

## 🏗️ Architecture

This project documents its system architecture using Mermaid diagrams, which GitHub renders natively.

```mermaid
graph TD
    A["Input stream (JSON/YAML)"] --> B["pipefog CLI"]
    B --> C["Mode"]
    C -->|default| D1["Parse tokens"]
    D1 --> H0["sha256 hash"]
    D1 --> D2{String type?}
    H0 --> D2
    D2 -->|alpha_word| H1["hash→syllables"]
    D2 -->|uppercase_word| H2["hash→syllables→upper"]
    D2 -->|capitalized_word| H3["hash→syllables→capitalized"]
    D2 -->|snake_case_word| H4["hash→snake_case"]
    D2 -->|iso8601_z_datetime| H5["obfuscate_iso8601_z_datetime"]
    D2 -->|base32_lowercase| H6["hash→base32_lower"]
    D2 -->|base32_uppercase| H7["hash→base32_upper"]
    D2 -->|no match| H8["hex encode hash"]
    H1 --> E["Output stream"]
    H2 --> E
    H3 --> E
    H4 --> E
    H5 --> E
    H6 --> E
    H7 --> E
    H8 --> E
    C -->|humanise| F["Convert bytes to syllables"]
    C -->|syllable-frequency| G["Report syllable stats"]
```

Planned features:

- ✅ Streaming-safe – Process large files through stdin/stdout with minimal memory usage.
