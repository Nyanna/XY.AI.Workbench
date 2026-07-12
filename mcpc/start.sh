#!/bin/bash

LT_COMPOSE_FILE="${HOME}/xyan/xy.ai.workbench/language-tool/docker-compose.yml"

lt_is_running() {
    docker compose -f "$LT_COMPOSE_FILE" ps --status running 2>/dev/null \
        | grep -q languagetool
}

LT_ALREADY_RUNNING=false
if lt_is_running; then
    LT_ALREADY_RUNNING=true
fi

if [[ "$LT_ALREADY_RUNNING" == "false" ]]; then
    echo "Starting LanguageTool..."
    LT_EXIT=0
    LT_ERROR="$(docker compose -f "$LT_COMPOSE_FILE" up -d 2>&1)" || LT_EXIT=$?
    if [[ $LT_EXIT -ne 0 ]]; then
        echo "Docker Error: $LT_ERROR" >&2
        if echo "$LT_ERROR" | grep -qiE "Cannot connect to the docker daemon|docker daemon is not running|is the docker daemon running|connect: no such file or directory"; then
            echo "Docker is not running. Starting Docker via systemctl --user start docker ..." >&2
            systemctl --user start docker
            echo "Retrying LanguageTool start..."
            docker compose -f "$LT_COMPOSE_FILE" up -d
        else
            echo "Error: LanguageTool failed to start." >&2
            exit $LT_EXIT
        fi
    fi
fi

source .env.sh
clear && PYTHONPATH=src python3 -m xy.ai.mcpc