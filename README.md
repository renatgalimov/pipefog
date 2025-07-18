# pipefog

This tool reads JSON values from standard input and writes them back with every string value replaced by its SHA3-256 hash. Input may consist of multiple JSON documents concatenated together. Each document is printed on a separate line using pretty formatting.

## Usage

```bash
cat input.json | cargo run --release
```
