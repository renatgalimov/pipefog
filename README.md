<img width="1280" height="640" alt="pipefog-social" src="https://github.com/user-attachments/assets/a210fa94-fe76-49c3-9b14-7d21bd2c4281" />

# pipefog

**pipefog** – Stream‑structured data obfuscator for JSON/YAML.

🔒 Mask your sensitive data while preserving format and structure. Designed for CLI pipelines and integration with tools like `jq` and `yq`.

---

## 🚀 Example Usage

```bash
cat secrets.json | jq . | pipefog | jq .
```
