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

value="$value" python3 - "$COOKIE_PATH" <<'PYEOF'
import json
import os
import sys
from datetime import datetime, timedelta, timezone

value = os.environ["value"]
expires = (datetime.now(timezone.utc) + timedelta(days=180)).strftime("%Y-%m-%dT%H:%M:%SZ")
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
