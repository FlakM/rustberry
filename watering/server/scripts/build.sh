#!/usr/bin/env bash
set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR=$(realpath "$SCRIPT_DIR/../..")


# build front end package
( cd "$ROOT_DIR/front-app" && trunk build --release)
HTML_PACKAGE_DIR="$(realpath $ROOT_DIR/front-app/dist)"


# build actix web server
cross build --target arm-unknown-linux-gnueabihf --release
