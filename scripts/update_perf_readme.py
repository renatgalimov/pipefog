#!/usr/bin/env python3
import argparse
from datetime import datetime, timezone


def build_block(args):
    change_percent = ((args.latest_ms - args.previous_ms) / args.previous_ms) * 100.0
    change_text = f"{change_percent:+.2f}%"
    updated_at = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")

    lines = [
        "<!-- perf-begin -->",
        f"Benchmark input: `tests/perf_sample.json` ({args.iterations} iterations).",
        "",
        "| Version | Total time (ms) |",
        "| --- | ---: |",
        f"| {args.latest_tag} | {args.latest_ms:.2f} |",
        f"| {args.previous_tag} | {args.previous_ms:.2f} |",
        f"| Change | {change_text} |",
        "",
        f"Last updated: {updated_at}",
        "<!-- perf-end -->",
    ]
    return "\n".join(lines)


def update_readme(readme_path, block):
    with open(readme_path, "r", encoding="utf-8") as readme_file:
        contents = readme_file.read()

    begin = "<!-- perf-begin -->"
    end = "<!-- perf-end -->"

    if begin in contents and end in contents:
        prefix = contents.split(begin)[0]
        suffix = contents.split(end)[1]
        updated = prefix + block + suffix
    else:
        updated = contents.rstrip() + "\n\n## Performance\n\n" + block + "\n"

    with open(readme_path, "w", encoding="utf-8") as readme_file:
        readme_file.write(updated)


def main():
    parser = argparse.ArgumentParser(description="Update README performance section.")
    parser.add_argument("--readme", required=True)
    parser.add_argument("--latest-tag", required=True)
    parser.add_argument("--latest-ms", type=float, required=True)
    parser.add_argument("--previous-tag", required=True)
    parser.add_argument("--previous-ms", type=float, required=True)
    parser.add_argument("--iterations", type=int, required=True)
    args = parser.parse_args()

    block = build_block(args)
    update_readme(args.readme, block)


if __name__ == "__main__":
    main()
