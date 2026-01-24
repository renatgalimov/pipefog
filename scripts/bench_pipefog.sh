#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <manifest-path> [iterations] [sample-path]" >&2
  exit 1
fi

manifest_path="$1"
iterations="${2:-200}"
sample_path="${3:-tests/perf_sample.json}"

workdir="$(cd "$(dirname "$manifest_path")" && pwd)"
sample_path="$(cd "$(dirname "$sample_path")" && pwd)/$(basename "$sample_path")"

input_file="$(mktemp)"
cleanup() {
  rm -f "$input_file"
}
trap cleanup EXIT

python3 - "$sample_path" "$iterations" "$input_file" <<'PY'
import sys

sample_path, iterations, output_path = sys.argv[1], int(sys.argv[2]), sys.argv[3]
with open(sample_path, "rb") as sample_file:
    sample_bytes = sample_file.read().strip()

with open(output_path, "wb") as output_file:
    for index in range(iterations):
        output_file.write(sample_bytes)
        if index + 1 < iterations:
            output_file.write(b"\n")
PY

(
  cd "$workdir"
  cargo build --release --manifest-path "$manifest_path"
)

binary_path="$workdir/target/release/pipefog"

python3 - "$binary_path" "$input_file" <<'PY'
import subprocess
import sys
import time

binary_path, input_path = sys.argv[1], sys.argv[2]
start = time.perf_counter()
with open(input_path, "rb") as input_file:
    subprocess.run([binary_path], stdin=input_file, stdout=subprocess.DEVNULL, check=True)
elapsed = time.perf_counter() - start
print(f"total_time_ms={elapsed * 1000:.2f}")
PY
