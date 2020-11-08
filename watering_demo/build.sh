#!/usr/bin/env bash

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"


DOCKER_BUILDKIT=1 docker build "$ROOT_DIR/" -f "$ROOT_DIR/docker/Dockerfile" \
    --build-arg database_url="${CLEVER_PG}" \
    -t rust-water-builder \
    --progress=plain
# docker run --rm --user  "$(id -u)":"$(id -g)" \
#     -e DATABASE_URL="$CLEVER_PG" \
#     -v "$PWD":/app \
#     rust-water-builder