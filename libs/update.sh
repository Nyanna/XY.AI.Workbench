#!/usr/bin/env bash
#
# Refreshes JARs in a libs/ directory via `dependency:copy-dependencies`
# against the single libs/pom.xml, filtered per-directory by
# allowed-artifacts.txt (whitelist). Not a build; no `mvn install`.
#
# New transitive deps not in the whitelist are silently skipped; deps that
# drop out of the graph disappear (old jars are deleted before copying).
#
# Usage:
#   ./update.sh <dirname|root|all>
#   ./update.sh latest   bump anthropic-java/google-genai/openai-java in
#                        libs/pom.xml to their newest release (versions-maven-
#                        plugin), then run this again with all/root/<dirname>
#
# Version bump workflow: edit libs/pom.xml (<dependencies>/<dependencyManagement>),
# add any newly-needed artifactId to the relevant allowed-artifacts.txt, then
# run ./update.sh all.

SDK_ARTIFACTS="com.anthropic:anthropic-java,com.google.genai:google-genai,com.openai:openai-java"

set -euo pipefail

LIBS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MVN="$LIBS_DIR/../tools/apache-maven-3.9.16/bin/mvn"
POM="$LIBS_DIR/pom.xml"

if [[ ! -x "$MVN" ]]; then
  echo "Fehler: Maven nicht gefunden unter $MVN" >&2
  exit 1
fi
if [[ ! -f "$POM" ]]; then
  echo "Fehler: Keine zentrale pom.xml unter $POM gefunden" >&2
  exit 1
fi

update_dir() {
  local dir="$1"
  local target="$LIBS_DIR/$dir"
  local whitelist="$target/allowed-artifacts.txt"

  if [[ ! -f "$whitelist" ]]; then
    echo "Fehler: Keine allowed-artifacts.txt in $target gefunden" >&2
    exit 1
  fi

  local include_ids
  include_ids="$(grep -v '^[[:space:]]*$' "$whitelist" | paste -sd, -)"

  echo "==> Aktualisiere $target"
  echo "    Whitelist: $include_ids"

  find "$target" -maxdepth 1 -type f -name '*.jar' -delete

  "$MVN" -f "$POM" -q dependency:copy-dependencies \
      -DoutputDirectory="$target" \
      -DincludeScope=runtime \
      -DincludeArtifactIds="$include_ids"

  echo "==> Fertig: $target"
}

usage() {
  echo "Verwendung: $0 <verzeichnisname|root|all>" >&2
  echo "Verfuegbare Verzeichnisse:" >&2
  for d in "$LIBS_DIR"/*/; do
    d="${d%/}"
    if [[ -f "$d/allowed-artifacts.txt" ]]; then
      echo "  - $(basename "$d")" >&2
    fi
  done
  exit 1
}

[[ $# -eq 1 ]] || usage

case "$1" in
  latest)
    "$MVN" -f "$POM" versions:use-latest-releases -Dincludes="$SDK_ARTIFACTS"
    rm -f "$LIBS_DIR/pom.xml.versionsBackup"
    exit 0
    ;;
  root)
    update_dir "."
    ;;
  all)
    update_dir "."
    for d in "$LIBS_DIR"/*/; do
      d="${d%/}"
      name="$(basename "$d")"
      if [[ -f "$d/allowed-artifacts.txt" ]]; then
        update_dir "$name"
      fi
    done
    ;;
  *)
    if [[ -d "$LIBS_DIR/$1" && -f "$LIBS_DIR/$1/allowed-artifacts.txt" ]]; then
      update_dir "$1"
    else
      usage
    fi
    ;;
esac
