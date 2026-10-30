#!/usr/bin/env sh
set -eu
cd "$(dirname "$0")"
COBALT_BINDINGS_OUT=1 cargo build --features generate_bindings,build_sdk
