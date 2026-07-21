#!/usr/bin/env bash
set -euo pipefail

COOKIE_DIR="$HOME/Library/Application Support/cargo-compete"
COOKIE_PATH="$COOKIE_DIR/cookies.jsonl"

echo "Chrome DevTools (Application > Cookies > https://atcoder.jp) の REVEL_SESSION の Value を貼り付けてください。"
read -r -s -p "REVEL_SESSION value: " value
echo

if [ -z "$value" ]; then
  echo "value が空です" >&2
  exit 1
fi

mkdir -p "$COOKIE_DIR"

expires=$(date -u -v+180d +"%Y-%m-%dT%H:%M:%SZ")

value="$value" expires="$expires" python3 - "$COOKIE_PATH" <<'PYEOF'
import json
import os
import sys

value = os.environ["value"]
expires = os.environ["expires"]
path = sys.argv[1]

entry = {
    "raw_cookie": f"REVEL_SESSION={value}; HttpOnly; Secure",
    "path": ["/", True],
    "domain": {"HostOnly": "atcoder.jp"},
    "expires": {"AtUtc": expires},
}

with open(path, "w") as f:
    f.write(json.dumps(entry) + "\n")
PYEOF

chmod 600 "$COOKIE_PATH"
echo "wrote $COOKIE_PATH"
