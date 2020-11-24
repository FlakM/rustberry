#!/usr/bin/env bash
set -e
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"


DOCKER_BUILDKIT=1 docker build "$ROOT_DIR/" -f "$ROOT_DIR/docker/Dockerfile" \
    --build-arg database_url="${CLEVER_PG}" \
    -t rust-water-builder \
    --progress=plain

OUT_DIR="$ROOT_DIR/target/build"
mkdir -p "$OUT_DIR"

docker run --rm \
    --user  "$(id -u)":"$(id -g)" \
    -v "$OUT_DIR":/out/ \
    rust-water-builder \
    cp /bins/rustberry /out/

cp config.json "$OUT_DIR"