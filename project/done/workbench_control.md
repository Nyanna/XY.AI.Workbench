# Workbench Control

Implementiere diese Änderungen.
Alternativ zu dieser Bash-Volage: `/home/user/xyan/xy.ai.workbench/mcpc/control.sh` soll eine Control-Client-Implementierung direkt in den retrieval Loop des Connector `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/ClaudeCodeConnector.java` implementiert werden.

* Die Methode `xy.ai.workbench.connectors.claudecode.ClaudeCodeConnector.preprocessInput(String, boolean[], String[])` soll angepasst werden. Die bisherige Implementierung von "/allow <id>" und "/deny <id> <reason>" wird ersetzt.
* Der Retrieval Loop wird erweitert und lauscht zuerst auf dem Kontrollendpunkt und erst wenn dieser Leer ist auf "session.readLine()". Das garantiert das zuerst Anfragen bearbeitet werden bevor neue ausgelöst werden können.
* Liegen Anfragen am Kontrollendpunkt vor, nimmt der Loop die erste und gibt sie als Result zurück.
* Das Result umfasst die Ausgabe der unveränderten JSON Datenstruktur.
* Zeilen des Result wird ein "#: " vorangestellt was sie als Markdown-Kommentare Markiert.
* Möchte der Nutzer Parameter verändern so wird er die "#: " selbst aus der Prompteingabe selbst entfernen. 


## Ablauf

1. Ein "/allow <id>", "/deny <id> <reason>", veränderte Ein- und Rückgaben, wird zuerst an den Kontrollendpunkt übermittelt
2. Dann werden verbleibende Anfragen zurückgegeben und erst wenn keine Anfragen mehr vorliegen
3. Wird mit der weiteren CLI-Kommunikation vortgesetzt.

## Operationen

Unterstützt werden:

* Das Genehmigen einer Anfrage mittels: "/allow <id>"
* Das Ablehnen einer Anfrage mit Grund: "/deny <id> <reason>"
* Das Verändern der Parameter für Tool-Eingaben
* Das Verändern der Rückgabe von Tool-Ausgaben

## Veränderungen

Veränderungen werden daran erkannt das der Prompt die originale JSON-Datenstruktur enthält die eine passende ID zu den offenen Anfragen hat.

* Wird eine Veränderung der Ein- oder Rückgabe erkannt wird diese entsprechedn der Verarbeitung in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py` übermittelt.
