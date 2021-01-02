#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR=$(realpath "$SCRIPT_DIR/..")

OUT_DIR="$ROOT_DIR/target/arm-unknown-linux-gnueabihf/release"


scp "$OUT_DIR/config.json" pi@192.168.0.100:~/rustberry/config.json
scp "$OUT_DIR/rustberry" pi@192.168.0.100:~/rustberry/rustberry


scp $ROOT_DIR/systemd/* pi@192.168.0.100:~/.config/systemd/user/