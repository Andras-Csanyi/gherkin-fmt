#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

cargo build

GHERKINFMT="$ROOT/target/debug/gherkinfmt"

for formatter_dir in "$ROOT"/testing/; do
  for input in "$formatter_dir"*.feature; do
    filename="$(basename "$input")"

    if [[ "$filename" == *-result.feature ]]; then
      continue
    fi

    result="${input%.feature}-result.feature"
    "$GHERKINFMT" --debug --output "$result" "$input"
  done
done

