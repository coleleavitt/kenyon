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
fdeps=$(cargo tree -p flyrag -e normal --depth 1 --prefix none | tail -n +2 | awk '{print $1}' | sort -u | tr '\n' ' ')
if [ "$fdeps" != "kenyon thiserror " ]; then
  echo "flyrag runtime dependencies must be exactly 'kenyon thiserror'; found: $fdeps" >&2
  exit 1
fi
echo "flyrag deps ok: $fdeps"
