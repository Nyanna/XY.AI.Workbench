Erledige die offenen Punkte.

- nvcc liegt unter `/usr/local/cuda-13.3/bin/nvcc`

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)

# Übergabe: offene Punkte vor F1

Korrektur zu meiner letzten Antwort: Zum Zeitpunkt von F1 muss alles Vorherige erledigt sein, auch die Punkte, die ich als „bewusst nicht gelöst" geführt habe. Die Liste unten enthält deshalb alles, was vor F1 offen ist. Nicht geprüft habe ich E3 bis E8 gegen ihre eigenen Verifikationspunkte, weil ich ihre Result-Abschnitte nicht lesen durfte.

## A. Funktional offen (Code)

2. **E7: Segmentladen „direkt"**
   - Die Entries eines Segments werden einzeln über den Gather-Pfad geladen. Der Plan sieht einen direkten Bulk-Upload vor.
   - Ursache ist eine Lücke im B4-Vertrag: Das `Index`-Trait hat keinen Zugriff auf zusammenhängende Bytebereiche.
   - Zu tun: entweder eine Methode für Bereichs- oder Segmentzugriff im `Index`-Trait ergänzen (ändert B4, betrifft `InMemoryIndex` und alle Implementierungen) und den Bulk-Pfad bauen, oder die Abweichung entscheiden und festschreiben.

3. **E1: Hint-Anbindung an den Betrieb**
   - `read_set` mit `Sequential` und `Random` existiert und ist getestet, hat aber noch keinen Aufrufer außerhalb der Tests.
   - Zu klären: Wer ruft es auf, und woher kommen die Byte-Ranges der Entries im mmap? Das `Index`-Trait liefert keine Offsets. Das hängt mit Punkt 2 zusammen.

4. **E4/E5/E7: Verdrahtung des Betriebs**
   - `publish_loaded` und `complete_slot` sind `pub(crate)` und haben keinen Aufrufer. Das erzeugt Dead-Code-Warnungen.
   - Sie bekommen `swap` und `store` als Parameter, weil `SwapManager` die Arena leiht. Es fehlt der Besitzer, der Manager, Swap und Store zusammenhält und `cycle`, `publish_loaded` und `complete_slot` in der richtigen Reihenfolge treibt.

5. **E7: Evictor-Modell**
   - Der Evictor führt einen Schlüssel pro Index und überschreibt dessen Bytes bei jedem Laden mit der zuletzt geladenen Menge. Er zählt sie nicht auf.
   - Falls der Plan Eviction pro Segment oder pro Menge erwartet, ist das eine Abweichung. Ich habe das nicht gegen die DoD geprüft.

6. **E9: Lücken laut eigenem Result-Abschnitt**
   - Es gibt keinen Test, dass der Kern keine Parent-Child-Struktur hat.
   - Das Referenz-Addon gibt `Vec<TaskHandle>` statt `Vec<Task>` zurück (Abweichung vom Plan).
   - Es gibt keinen Hardware-Smoke-Test für Extensions auf der GTX 1660.
   - 22 ignorierte CUDA-Tests sind nicht untersucht.

## B. Verifikationen, die offen oder unbelegt sind

8. **E1: `madvise`-Semantik gegen die man-Seite:** Empirisch durch `mincore`-Tests belegt, aber nicht gegen die Dokumentation gelesen. Offen bleibt auch das Verhalten auf dem Zielkernel.
9. **E2:** Der Result-Abschnitt ist durch die Umsetzung überholt. Eine Prüfung der E2-DoD gegen den Code habe ich begonnen, aber nicht abgeschlossen: Der Leseversuch von `plan_e_02.md` brach ab. Umgesetzt sind Roaring, Resolver, Registry und Tracing, das ist aber nicht Punkt für Punkt gegen die DoD abgeglichen.
10. **Loom:** Die 7 Tests laufen grün. Ob die geänderte Arena (`IndexArena::shared`, `AllocRef`) ein eigenes loom-Modell braucht, habe ich nicht geprüft.

## D. Code-Hygiene

16. Dead-Code-Warnungen für `publish_loaded` und `complete_slot` (siehe Punkt 4).
17. Ungenutzter Import `cuda_port::CudaUploadPort` bei `--features cuda`.

## Empfohlene Reihenfolge
2. Entscheidung zum `Index`-Trait treffen (Punkt 2), weil davon auch Punkt 3 abhängt.
3. Betriebsverdrahtung bauen (Punkt 4).
4. E9-Lücken schließen (Punkt 6).