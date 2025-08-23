[![Rust Tests](https://github.com/renatgalimov/pipefog/actions/workflows/tests.yml/badge.svg)](https://github.com/renatgalimov/pipefog/actions/workflows/tests.yml)
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
flowchart TD
    A[Input stream (JSON/YAML)] --> B[pipefog CLI]
    B --> C[Mode]
    C -->|default| D[Parse & obfuscate strings]
    D --> E[Output stream]
    C -->|humanise| F[Convert bytes to syllables]
    C -->|syllable-frequency| G[Report syllable stats]
```

Planned features:

- ✅ Streaming-safe – Process large files through stdin/stdout with minimal memory usage.
