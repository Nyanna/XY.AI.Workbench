#!/usr/bin/env bash
# Generiert einen Java-Client (openapi-generator, library=native, Jackson 3)
# aus einer OpenAPI-Spec unter libs/openapi/<name>.yaml.
# Existiert filters/<name>.yaml, wird die Spec vorher damit gefiltert.
#
# Usage: ./generate-client.sh <name>
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ $# -ne 1 ]; then
    echo "Usage: $0 <spec-name>" >&2
    exit 1
fi
NAME="$1"

BASE_SPEC="$SCRIPT_DIR/${NAME}.yaml"
FILTER_CONF="$SCRIPT_DIR/filters/${NAME}.yaml"

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

SPEC_FILE="$BASE_SPEC"
if [ -f "$FILTER_CONF" ]; then
    SPEC_FILE="$WORK_DIR/${NAME}.filtered.yaml"
    echo "Filtere Spec mit $FILTER_CONF ..."
    python3 "$SCRIPT_DIR/filter_spec.py" "$FILTER_CONF" "$SPEC_FILE"
    cp "$SPEC_FILE" "$SCRIPT_DIR/filters/"
fi

if [ ! -f "$SPEC_FILE" ]; then
    echo "Spec nicht gefunden: $SPEC_FILE" >&2
    exit 1
fi

# we use our own generator now
echo "Done with filtering"
exit 0

REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
GENERATOR_JAR="$REPO_ROOT/tools/openapi-generator-cli-7.25.0.jar"
SRC_ROOT="$REPO_ROOT/src/xy/ai/workbench/connector/openapi"
PACKAGE_BASE="xy.ai.workbench.connector.openapi.${NAME}"
OUT_DIR="$WORK_DIR/gen"

# Generator-Optionen als Dict, damit weitere Optionen leicht ergaenzt werden koennen.
declare -A ADDITIONAL_PROPERTIES=(
    [useJackson3]=false
    [dateLibrary]=java8
    [openApiNullable]=false
    [hideGenerationTimestamp]=true
    [generateGeneratedAnnotation]=false
    [useJspecify]=false
    [useOneOfInterfaces]=false
)
additional_properties_arg=""
for key in "${!ADDITIONAL_PROPERTIES[@]}"; do
    additional_properties_arg+="${key}=${ADDITIONAL_PROPERTIES[$key]},"
done
additional_properties_arg="${additional_properties_arg%,}"

java -jar "$GENERATOR_JAR" generate \
    -i "$SPEC_FILE" \
    -g java \
    --library native \
    --additional-properties="$additional_properties_arg" \
    --template-dir "$SCRIPT_DIR/templates" \
    --api-package "${PACKAGE_BASE}.api" \
    --model-package "${PACKAGE_BASE}.model" \
    --invoker-package "${PACKAGE_BASE}" \
    --skip-validate-spec \
    -o "$OUT_DIR"

GEN_JAVA_DIR="$OUT_DIR/src/main/java/xy/ai/workbench/connector/openapi/${NAME}"
if [ ! -d "$GEN_JAVA_DIR" ]; then
    echo "Generierte Sourcen nicht gefunden: $GEN_JAVA_DIR" >&2
    exit 1
fi

TARGET_DIR="$SRC_ROOT/${NAME}"
mkdir -p "$TARGET_DIR"
rsync -a --delete "$GEN_JAVA_DIR/" "$TARGET_DIR/"

echo "Client '${NAME}' generiert nach: $TARGET_DIR"
