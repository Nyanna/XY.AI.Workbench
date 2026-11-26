#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUSTIR_DIR="$(dirname "$SCRIPT_DIR")"
WORKBENCH_DIR="$(cd "$RUSTIR_DIR/../.." && pwd)"
DRIVER_DIR="$WORKBENCH_DIR/ast-engines/rustir-driver"
RAG_DIR="${RAG_DIR:-$WORKBENCH_DIR/rag}"
SOURCE_DIR="${SOURCE_DIR:-$RAG_DIR/src/resident}"
PREFIX="${RUSTIR_PREFIX:-resident}"
TOOLCHAIN="${RAG_TOOLCHAIN:-nightly-2026-11-25}"
OUT_DIR="${OUT_DIR:-${TMPDIR:-/tmp}/rustir-resident}"
RENDER_DIR="${RENDER_DIR:-$RUSTIR_DIR/testrag}"
REUSE_EDGES="${REUSE_EDGES:-0}"
RENDER_CHECK="${RENDER_CHECK:-0}"
read -r -a RAG_CARGO_ARGS <<< "${RAG_CARGO_ARGS:---lib}"
EDGES_FILE="$OUT_DIR/edges.txt"
DRIVER_BIN="$DRIVER_DIR/target/release/rustir-driver"

mkdir -p "$OUT_DIR"

rustir() {
    cargo run --quiet --manifest-path "$RUSTIR_DIR/Cargo.toml" -- "$@"
}

echo "[1/5] checking object names in $SOURCE_DIR"
rustir parse "$SOURCE_DIR" > /dev/null

if [ "$REUSE_EDGES" = "1" ] && [ -s "$EDGES_FILE" ]; then
    echo "[2/5] reusing $EDGES_FILE"
    echo "[3/5] skipped"
else
    echo "[2/5] building rustir-driver"
    (cd "$DRIVER_DIR" && cargo build --release --quiet --target-dir "$DRIVER_DIR/target")

    echo "[3/5] running the compiler over rag with toolchain $TOOLCHAIN"
    rm -f "$EDGES_FILE"
    (
        cd "$RAG_DIR"
        export CARGO_TARGET_DIR="$OUT_DIR/rag-target"
        cargo "+$TOOLCHAIN" clean --quiet -p xy-ai-rag
        RUSTC_WORKSPACE_WRAPPER="$DRIVER_BIN" \
            RUSTIR_OUT="$EDGES_FILE" \
            RUSTIR_PREFIX="$PREFIX" \
            cargo "+$TOOLCHAIN" check -p xy-ai-rag "${RAG_CARGO_ARGS[@]}"
    )
    if [ ! -s "$EDGES_FILE" ]; then
        echo "error: the driver wrote no edges to $EDGES_FILE (check RUSTIR_PREFIX=$PREFIX)" >&2
        exit 1
    fi
fi

echo "[4/5] rendering sources into $RENDER_DIR/src"
rustir render "$SOURCE_DIR" --edges "$EDGES_FILE" --out "$RENDER_DIR" --sources-only

if [ "$RENDER_CHECK" = "1" ]; then
    echo "[5/5] compiling $RENDER_DIR with toolchain $TOOLCHAIN"
    (cd "$RENDER_DIR" && cargo "+$TOOLCHAIN" check --lib)
else
    echo "[5/5] compile check skipped (set RENDER_CHECK=1 to enable)"
fi

echo "edges:  $EDGES_FILE ($(wc -l < "$EDGES_FILE") lines)"
echo "output: $RENDER_DIR/src ($(find "$RENDER_DIR/src" -name '*.rs' | wc -l) modules)"
