#!/usr/bin/env bash
set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR=$(realpath "$SCRIPT_DIR/..")


DATABASE_URL="${CLEVER_PG}" cross build --target arm-unknown-linux-gnueabihf --release
OUT_DIR="$ROOT_DIR/target/arm-unknown-linux-gnueabihf/release"

mkdir -p "$OUT_DIR"
cp config.json "$OUT_DIR"
