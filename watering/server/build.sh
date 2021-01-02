#!/usr/bin/env bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

set -e

"$SCRIPT_DIR/scripts/build.sh"
"$SCRIPT_DIR/scripts/uninstall.sh"
"$SCRIPT_DIR/scripts/upload.sh"
"$SCRIPT_DIR/scripts/install.sh"