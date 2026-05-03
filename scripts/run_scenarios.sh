#!/usr/bin/env bash
    # SPDX-License-Identifier: Apache-2.0
    #
    # Copyright (c) 2025 RTVLAS contributors

    set -euo pipefail

    . "${HOME}/.cargo/env"

    while IFS= read -r scenario; do
      id="$(python3 - <<'PY' "$scenario"
import json, sys
item = json.loads(sys.argv[1])
print(item["id"])
PY
)"
      input="$(python3 - <<'PY' "$scenario"
import json, sys
item = json.loads(sys.argv[1])
print(item["input"])
PY
)"
      output="$(python3 - <<'PY' "$scenario"
import json, sys
item = json.loads(sys.argv[1])
print(item["output"])
PY
)"
      cargo run --release -p eval_tool -- "$id" "$input" "$output"
    done < <(python3 - <<'PY'
import json
from pathlib import Path
manifest = json.loads(Path("scenarios/manifest.json").read_text())
for item in manifest:
    print(json.dumps(item))
PY
)
