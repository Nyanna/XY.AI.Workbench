# Openalex Tools

Implementiere ein Set von Openalex Tools in einem eigenen Package und Subackages.

* Paging wird nur auf die erste Seite festgesetzt.
* Fields werden begrenzt unterstützt und in semantische sinnvolle Presets aggregiert `/home/user/xyan/xy.ai.workbench/project/oa_fields.md`

## Authentication

* add key to API URL "https://api.openalex.org/works?api_key=YOUR_KEY"
* envar for API-key will be "MCPC_OPENALEX_KEY"

## Architektur

Implementiere die Schnittstellen vollständig und darauf aufbauend ein Set von Tools das die Schnittstellen verwendet mittels Presets.

* Eine Package rein für Schnittstellen mit vollständiger API.
* Eine Reihe von Tools die mit Standardannahmen eine einfache Verwendung durch KI-Agenten ermöglichen.

## Kontext

Es ist zu erwarten, das die hiermit Implementierten grundlegenden Tools mit der weiteren Nutzung verändert werden auf die tatsächlichen Anwendungsfälle.

* Python Projekt: `/home/user/xyan/xy.ai.workbench/mcpc/src`
* Unittests sind nicht notwendig.

## Tools & Endpunkt

Implementiere folgende Schnittstellen:

* Suche: `/home/user/xyan/xy.ai.workbench/project/oa_search.md`
* KI gestützte Suche: `/home/user/xyan/xy.ai.workbench/project/oa_ssearch.md`
* Einzelarbeiten: `/home/user/xyan/xy.ai.workbench/project/oa_single.md`
