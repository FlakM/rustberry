#!/usr/bin/env bash
set -e

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

OUT_DIR="$ROOT_DIR/target/build"


scp "$OUT_DIR/config.json" pi@192.168.0.100:~/rustberry/config.json
scp "$OUT_DIR/rustberry" pi@192.168.0.100:~/rustberry/rustberry