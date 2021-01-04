#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR=$(realpath "$SCRIPT_DIR/../..")

OUT_DIR="$ROOT_DIR/server/target/arm-unknown-linux-gnueabihf/release"


scp "$OUT_DIR/watering_server" pi@192.168.0.100:~/server/
scp $ROOT_DIR/server/systemd/* pi@192.168.0.100:~/.config/systemd/user/