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

🔐 Deterministic obfuscation – ID fields, usernames, and keys are hashed in a consistent format.
🏷️ Shape-preserving – Keeps field order, numeric values, and categories untouched.
🧩 Supports JSON and YAML – Auto-detects format or allow override with --format.
🛠️ Composable CLI – Works seamlessly in pipelines with jq, yq, and other Unix tools.

Planned features:

✅ Streaming-safe – Process large files through stdin/stdout with minimal memory usage.
🗓️ Date-like transformation – Preserves valid ISO 8601 date format with safe offsets.
