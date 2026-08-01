#!/usr/bin/env bash
#
# Builds XY.AI.Workbench
# It also computes the compile classpath itself.
#
# Usage:
#   ./build.sh                 -> mvn clean package
#   ./build.sh clean verify    -> any other Maven goals/args are passed through
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MAVEN_HOME="${SCRIPT_DIR}/tools/apache-maven-3.9.16"

# Override with: ECLIPSE_INSTALL_DIR=/path/to/eclipse ./build.sh
ECLIPSE_INSTALL_DIR="${ECLIPSE_INSTALL_DIR:-/home/user/Downloads/eclipse-committers-2025-06-R-linux-gtk-x86_64/eclipse}"
ECLIPSE_PLUGINS_DIR="${ECLIPSE_INSTALL_DIR}/plugins"

if [ ! -d "${ECLIPSE_PLUGINS_DIR}" ]; then
	echo "ERROR: Eclipse plugins directory not found: ${ECLIPSE_PLUGINS_DIR}" >&2
	echo "  Set ECLIPSE_INSTALL_DIR to point at your local Eclipse installation." >&2
	exit 1
fi

LIBS_DIR="${SCRIPT_DIR}/libs"

if [ -z "${JAVA_HOME:-}" ]; then
	for candidate in \
		/usr/lib/jvm/java-21-openjdk-amd64 \
		/usr/lib/jvm/java-1.21.0-openjdk-amd64 \
		/usr/lib/jvm/default-java; do
		if [ -x "${candidate}/bin/javac" ]; then
			JAVA_HOME="${candidate}"
			break
		fi
	done
fi

if [ -z "${JAVA_HOME:-}" ] || [ ! -x "${JAVA_HOME}/bin/javac" ]; then
	echo "ERROR: no JDK 21 found. Set JAVA_HOME explicitly, e.g.:" >&2
	echo "  JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64 ./build.sh" >&2
	exit 1
fi

export JAVA_HOME
export PATH="${JAVA_HOME}/bin:${MAVEN_HOME}/bin:${PATH}"

# Build an explicit, ':'-separated classpath
shopt -s nullglob
CP_ENTRIES=()
for jar in "${ECLIPSE_PLUGINS_DIR}"/*.jar "${LIBS_DIR}"/*.jar "${LIBS_DIR}"/*/*.jar; do
	CP_ENTRIES+=("${jar}")
done
shopt -u nullglob

if [ "${#CP_ENTRIES[@]}" -eq 0 ]; then
	echo "ERROR: no jars found on classpath (checked ${ECLIPSE_PLUGINS_DIR} and ${LIBS_DIR})" >&2
	exit 1
fi

COMPILE_CLASSPATH="$(IFS=:; echo "${CP_ENTRIES[*]}")"

GOALS=("$@")
if [ "${#GOALS[@]}" -eq 0 ]; then
	GOALS=(clean package)
fi

"${MAVEN_HOME}/bin/mvn" -q -ntp -B \
	-Declipse.install.dir="${ECLIPSE_INSTALL_DIR}" \
	-Dcompile.classpath="${COMPILE_CLASSPATH}" \
	"${GOALS[@]}" && echo "Successfull"