#!/usr/bin/env bash
set -euo pipefail

PORT="${1:-8080}"
IP="$(ipconfig getifaddr en0 2>/dev/null || ipconfig getifaddr en1 2>/dev/null || echo '127.0.0.1')"

if ! command -v trunk >/dev/null 2>&1; then
  echo "ERROR: trunk is not installed. Install with: cargo install trunk" >&2
  exit 1
fi

echo "LAN URL: http://${IP}:${PORT}"
echo "If another device cannot connect, check macOS firewall and that both devices are on the same LAN."

cargo check --lib --target wasm32-unknown-unknown
trunk serve --address 0.0.0.0 --port "${PORT}"
