#!/usr/bin/env bash
# Prepend SPDX license line to .rs files under crates/, src/, tests/, examples/, benchmarks/.
# Skips target/, .git/, archives/; skips files whose first line already contains SPDX-License-Identifier.

set -euo pipefail

ROOT="${ROOT:-/home/eastgate/Development/ecoPrimals/phase1/beardog}"
HEADER='// SPDX-License-Identifier: AGPL-3.0-only'

dirs=()
for d in crates src tests examples benchmarks; do
  [[ -d "$ROOT/$d" ]] && dirs+=("$ROOT/$d")
done

if [[ ${#dirs[@]} -eq 0 ]]; then
  echo "No target directories under $ROOT" >&2
  echo "Modified: 0"
  exit 0
fi

modified=0
while IFS= read -r -d '' f; do
  # Rule 2: line 1 already declares SPDX
  if head -n 1 "$f" | grep -q 'SPDX-License-Identifier'; then
    continue
  fi
  # Prepend header as new line 1 (before //!, #![], use, etc.)
  tmp=$(mktemp)
  awk -v h="$HEADER" 'BEGIN{print h} {print}' "$f" > "$tmp" && mv "$tmp" "$f"
  modified=$((modified + 1))
done < <(
  find "${dirs[@]}" \
    \( -type d \( -name target -o -name .git -o -name archives \) -prune \) -o \
    \( -name '*.rs' -type f -print0 \)
)

echo "Modified: $modified"
