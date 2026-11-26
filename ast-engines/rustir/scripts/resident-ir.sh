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
read -r -a RAG_CARGO_ARGS <<< "${RAG_CARGO_ARGS:---lib}"

EDGES_FILE="$OUT_DIR/edges.txt"
IR_FILE="$OUT_DIR/ir.txt"
DRIVER_BIN="$DRIVER_DIR/target/release/rustir-driver"

mkdir -p "$OUT_DIR"

rustir() {
    cargo run --quiet --manifest-path "$RUSTIR_DIR/Cargo.toml" -- "$@"
}

echo "[1/4] checking object names in $SOURCE_DIR"
rustir parse "$SOURCE_DIR" > /dev/null

echo "[2/4] building rustir-driver"
(cd "$DRIVER_DIR" && cargo build --release --quiet --target-dir "$DRIVER_DIR/target")

echo "[3/4] running the compiler over rag with toolchain $TOOLCHAIN"
rm -f "$EDGES_FILE"
(
    cd "$RAG_DIR"
    export CARGO_TARGET_DIR="$OUT_DIR/rag-target"
    cargo "+$TOOLCHAIN" clean --quiet -p rag
    RUSTC_WORKSPACE_WRAPPER="$DRIVER_BIN" \
        RUSTIR_OUT="$EDGES_FILE" \
        RUSTIR_PREFIX="$PREFIX" \
        cargo "+$TOOLCHAIN" check -p rag "${RAG_CARGO_ARGS[@]}"
)
if [ ! -s "$EDGES_FILE" ]; then
    echo "error: the driver wrote no edges to $EDGES_FILE (check RUSTIR_PREFIX=$PREFIX)" >&2
    exit 1
fi

echo "[4/4] merging sources and edges"
rustir parse "$SOURCE_DIR" --edges "$EDGES_FILE" > "$IR_FILE"

echo "edges:   $EDGES_FILE ($(wc -l < "$EDGES_FILE") lines)"
echo "objects: $IR_FILE ($(wc -l < "$IR_FILE") lines)"
