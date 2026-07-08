# Bash Control

Mit diesen Änderungen `/home/user/xyan/xy.ai.workbench/mcpc/project/done/tools_intercept.diff` wurde eine Kontrollschnittstelle implementiert.
Implementiere nun ein passendes Bashscript, das grundlegende Operationen kann.

* Das Script läuft in einer ständigen Schleife über den Kontrollendpunkt und zeigt die Anfragen nacheinander an. Bis es abgebrochen wird.
* Das Script fordert den Nutzer auf die Anfrage zu erlauben (default), oder mit einer Begründung abzulehnen.
* Das Bashskript zeigt jede Anfrage an, erlaubt als Antwort, jedoch nur Allow und Deny mit Begründung.
* Es erlaubt keine Veränderung von Ein- oder Rückgabe.
* Zielskript Pfad: `/home/user/xyan/xy.ai.workbench/mcpc/control.sh`