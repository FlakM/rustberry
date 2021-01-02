#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR=$(realpath "$SCRIPT_DIR/../..")

OUT_DIR="$ROOT_DIR/server/target/arm-unknown-linux-gnueabihf/release"
HTML_PACKAGE_DIR="$(realpath $ROOT_DIR/front-app/dist)"



ssh -qt pi@192.168.0.100 "
    mkdir -p /home/pi/server/html
"

scp "$OUT_DIR/watering_server" pi@192.168.0.100:~/server/
scp -rp "$HTML_PACKAGE_DIR/"* pi@192.168.0.100:~/server/html


scp $ROOT_DIR/server/systemd/* pi@192.168.0.100:~/.config/systemd/user/