# Tool Interceptor

Implementiere einen Tool-Interceptor der sowohl vor einem Toolaufruf passiert wird, als auch bevor das Ergebnis an den Agenten zurückgegeben wird.

## Kontext

Zur Steuerung des Agenten soll an zwei Punkten eingegriffen werden. Ein Human-in-the-loop soll:

1. Eine Toolausführung prüfen und genehmigen
2. In die Toolausführung eingreifen und Parameter anpassen, implizite Genehmigung
3. Eine Toolausführung ablehnen mit einem Hinweis zur weiteren Handlungsweise
3. Die Rückgabe des Tooloutputs verändern oder mit Anweisungen ersetzen

* Projektverzeichnis: `/home/user/xyan/xy.ai.workbench/mcpc/src`

## Controller

Erstelle einen weiteren REST-Endpunkt analog zu `/hooks/tool` mit dem Pfad `/control/tool`.
Ein Client wird sich mit diesem Endpunkt verbinden und zyklisch Nachrichten abrufen.

* Der Client erhält eine Liste von angefragten Tools oder ausgeführten Tools deren Rückgabe auf ein Approval wartet.
* Der Endpunkt erhält vom Client eine Liste genehmigter Toolausführungen oder Toolrückgaben.
* Zuerst werden die Genehmigungen des Clients verarbeitet und anschließend die Liste noch offener Anfragen zurückgegeben.

## Status

Eine globale Datenstruktur verwaltet die offenen Anfragen.

* Sie liefert die ausstehenden Anfragen an den Kontrollendpunkt
* Sie erhält genehmigte Anfragen des Kontrollendpunktes.
* Sie empfängt die Anfragen des Interceptors und blockiert

Hinweis: Prüfe ob hier ein Manager-Pattern sinnvoll ist.

## Interceptor

Soll ein Tool ausgeführt werden oder eine Rückgabe liefern blockiert der Interceptor den Aufruf oder die Rückgabe, leitet die Anfrage an die Datenstruktur weiter und wartet blockierend auf eine Antwort.

* Der Interceptor greift in die Anfrage oder Antwort ein.
* Der Interceptor kann den die Toolausgabe ersetzen oder die Anfrage mit Anweisungen ablehnen.

Hinweis: die Agenten mit sind mit einem entsprechend langem MCP-Timeout konfiguriert.

## Format

Der Kontrollendpunkt leitet das originale Ein- und Rückgabeformat weiter fügt diesem jedoch eine ID für eine Referenzierung hinzu.
Ein Approval kann so lediglich die ID referenzieren während Veränderungen von Ein- oder Ausgaben die komplette Datenstruktur verarbeiten können.
Die Antwort des Clients kennt daher 3 Fälle:

1. Einfaches Approval JSON referenziert die ID
2. Verweigerung mit ID und Verweigerungsgrund
3. Verändertes Ein- oder Ausgabe JSON

Hinweis: Ich denke, für die Rückgabe des Kontrollendpunktes reicht hier eine Liste die eine ID mit dem Tool Eingabe oder Ausgabe JSON verknüpft. 