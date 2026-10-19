#!/usr/bin/env bash
# kenyon keeps exactly one runtime dependency (thiserror). Fails if any other
# normal (non-dev, non-build) dependency appears at depth 1.
set -euo pipefail
cd "$(dirname "$0")/.."
deps=$(cargo tree -p kenyon -e normal --depth 1 --prefix none | tail -n +2 | awk '{print $1}' | sort -u)
if [ "$deps" != "thiserror" ]; then
  echo "kenyon runtime dependencies must be exactly 'thiserror'; found:" >&2
  echo "$deps" >&2
  exit 1
fi
echo "deps ok: $deps"
