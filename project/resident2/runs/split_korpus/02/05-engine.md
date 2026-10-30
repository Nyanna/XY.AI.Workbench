## Engine

### Konfiguration und Start

- Die Engine wird über einen Builder erzeugt, der erst mit einem expliziten Start-Aufruf wirksam wird.
- Die Angabe eines Modells ist verpflichtend. Fehlt sie, scheitert der Start mit einem Konfigurationsfehler, der das fehlende Feld benennt.
- Die Engine konfiguriert den Planner mit Subengines, Modellen und Indizes.
- Modell- und Device-Ressourcen werden beim Start nicht erzeugt, sondern erst durch den ersten Task angelegt.
- Die Engine bietet Methoden zum Enqueuen der beiden Task-Arten Embedding und Similarity.

### Verwaltung von Indizes

- Beliebig viele durchsuchbare Indizes können registriert werden. Jeder erhält eine eindeutige ID.
- Das Registrieren lädt den Index nicht. Unmittelbar danach gilt er als nicht geladen. Das Laden löst erst ein Task aus.

### Shutdown

- Die Engine bietet ein geordnetes Shutdown, das auch beim automatischen Drop der Instanz abläuft.
- Zuerst werden die Worker angehalten und gejoint.
- Danach werden alle Tasks abgebrochen, die kein Worker übernommen hat. Wartende Aufrufer erhalten den Fehler „cancelled“.
- Das explizite Shutdown meldet bei Erfolg eine positive Rückmeldung.

