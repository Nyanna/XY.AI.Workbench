# Funktionale Anforderungen Queue

## Zweck des Systems

Das System stellt eine Warteschlange für Arbeitseinheiten bereit, die von mehreren Erzeugern gleichzeitig befüllt und von mehreren Verbrauchern gleichzeitig abgearbeitet wird. Jede Arbeitseinheit hat einen eigenen Zustand mit Ergebnis und Benachrichtigung. Die Warteschlange wird mehrfach eingesetzt: einmal für Aufgaben und dreimal für Teiloperationen (Rechenprozessor, Grafikprozessor und eine gemeinsame Warteschlange).

## Einreihen von Einträgen

- Die Warteschlange muss beliebig viele Einträge aufnehmen. Jeder Eintrag besteht aus unveränderlichen Metadaten, einer Nutzlast und einer gemeinsamen Zustandseinheit.
- Die Metadaten (zum Beispiel Priorität, Ressource, Modell oder Betriebsart) muss die Warteschlange als undurchsichtige Information behandeln. Sie darf diese nicht interpretieren, sondern nur an Verbraucher zum Filtern bereitstellen.
- Der Typ der Metadaten, der Nutzlast, des Ergebnisses und des Fehlers wird vom Aufrufer festgelegt.
- Das Einreihen muss dem Aufrufer einen Verweis auf den neu angelegten Eintrag zurückgeben.
- Das Einreihen darf niemals blockieren und keine Sperre auf der Gesamtstruktur verwenden.
- Mehrere Erzeuger müssen gleichzeitig einreihen können. Kein Eintrag darf dabei verloren gehen oder überschrieben werden.
- Die Reihenfolge der Einträge entspricht ihrem Alter, also der Reihenfolge des erfolgreichen Anhängens. Die Reihenfolge der Einträge eines einzelnen Erzeugers muss dabei erhalten bleiben.
- Das Anhängen muss auch dann korrekt arbeiten, wenn der interne Hinweis auf das Ende der Warteschlange veraltet oder nicht aktualisiert ist. In diesem Fall muss das tatsächliche Ende gefunden und die Reihenfolge gewahrt werden.
- Die Warteschlange muss beim Anlegen eine beliebige, auch leere, Menge von Benachrichtigungskanälen entgegennehmen, je Teilengine einen.
- Bei jedem Einreihen müssen alle registrierten Benachrichtigungskanäle ein Signal erhalten, sodass wartende Teilengines aufwachen.
- Zu Testzwecken muss es möglich sein, das Aktualisieren des Endehinweises gezielt zu unterdrücken. So lässt sich nachweisen, dass die Reihenfolge und Vollständigkeit auch bei veraltetem Hinweis erhalten bleiben.

## Durchlaufen der Warteschlange

- Die Warteschlange muss einen Durchlauf aller Einträge in Altersreihenfolge anbieten, vom ältesten zum jüngsten Eintrag.
- Zusätzlich muss sie einen Durchlauf anbieten, der nur aktive Einträge liefert, also solche, die weder abgeschlossen noch abgebrochen sind.
- Der interne Ankereintrag am Anfang darf in keinem Durchlauf erscheinen.
- Beide Durchläufe dürfen niemals blockieren. Sie müssen auch dann gültige Ergebnisse liefern, wenn gleichzeitig eingereiht, beansprucht oder abgeschlossen wird.
- Ein Durchlauf muss jeden bereits gelieferten Eintrag gültig halten, auch wenn dieser danach logisch aus der Warteschlange entfernt wird.
- Eine leere Warteschlange muss einen leeren Durchlauf ergeben.
- Der Durchlauf muss exakt die Einträge in der Einreihungsreihenfolge liefern, ohne Auslassung und ohne Wiederholung.

## Aktivität und logisches Entfernen

- Ein Eintrag gilt als aktiv, solange seine Aufgabe wartend oder in Bearbeitung ist.
- Ein Eintrag gilt als nicht aktiv, sobald seine Aufgabe abgeschlossen oder abgebrochen ist. Der interne Ankereintrag gilt nie als aktiv.
- Abgeschlossene oder abgebrochene Einträge werden nicht physisch entfernt, sondern nur im aktiven Durchlauf übersprungen. Der vollständige Durchlauf liefert sie weiterhin.
- Der Speicher aller Einträge wird erst freigegeben, wenn die gesamte Warteschlange verworfen wird.

## Beanspruchen von Einträgen

- Ein Verbraucher muss einen Eintrag beanspruchen können. Dabei geht der Zustand von wartend nach in Bearbeitung über, und der Verbraucher erhält die Nutzlast.
- Die Nutzlast wird dem Beanspruchenden übergeben und nicht kopiert. Sie darf nur genau einmal ausgegeben werden.
- Ein erneutes Beanspruchen desselben Eintrags muss ohne Nutzlast scheitern.
- Das Beanspruchen eines abgebrochenen Eintrags muss scheitern und ohne Nutzlast enden.
- Das Beanspruchen des internen Ankereintrags darf nie eine Nutzlast liefern.
- Bei gleichzeitigem Beanspruchen desselben Eintrags durch mehrere Verbraucher darf genau einer gewinnen.
- Das Beanspruchen darf nie blockieren und nie auf andere Abläufe warten, während die Nutzlast gesperrt ist.
- Die Warteschlange muss das Beanspruchen eines Eintrags über eine eigene Funktion anbieten. Diese verhält sich identisch zum Beanspruchen am Eintrag selbst.
- Jeder Beanspruchungsversuch soll mit seinem Ausgang protokolliert werden.

## Zustandsmodell einer Aufgabe

- Eine Aufgabe hat genau vier mögliche Zustände: wartend, in Bearbeitung, abgeschlossen und abgebrochen. Neu angelegte Aufgaben sind wartend.
- Erlaubte Übergänge sind ausschließlich:
  - von wartend nach in Bearbeitung durch Beanspruchen,
  - von in Bearbeitung nach abgeschlossen durch Abschließen,
  - von wartend nach abgebrochen durch Abbrechen.
- Jeder Übergang muss atomar erfolgen. Bei gleichzeitigen Versuchen darf höchstens einer gelingen.
- Beanspruchen und Abbrechen schließen sich gegenseitig aus. Bei gleichzeitigem Auftreten gewinnt genau eines von beiden, und der Endzustand muss dazu passen.
- Der aktuelle Zustand muss jederzeit abfragbar sein. Die Abfrage liefert eine konsistente Momentaufnahme.
- Ein ungültiger interner Zustandswert gilt als nicht behebbarer interner Fehler.

## Abbrechen

- Das Abbrechen muss nur bei einer wartenden Aufgabe möglich sein. Es meldet dem Aufrufer Erfolg oder Misserfolg.
- Eine abgebrochene Aufgabe kann danach nicht mehr beansprucht werden.
- Das Abbrechen einer bereits beanspruchten, abgeschlossenen oder abgebrochenen Aufgabe darf nichts verändern und muss Misserfolg melden.
- Ein erfolgreiches Abbrechen muss alle Wartenden auf diese Aufgabe aufwecken.
- Jeder Abbruchversuch soll mit seinem Ausgang protokolliert werden.

## Abschließen und Ergebnis

- Das Abschließen muss ein Ergebnis speichern, das entweder ein Erfolgswert oder ein Fehlerwert ist. Danach geht die Aufgabe in den Zustand abgeschlossen über.
- Das Abschließen ist ausschließlich dem Verbraucher erlaubt, der die Aufgabe erfolgreich beansprucht hat, und darf nur einmal erfolgen.
- Das Ergebnis muss gespeichert und sichtbar sein, bevor der Zustand abgeschlossen für andere erkennbar wird. Wer den Zustand abgeschlossen beobachtet, muss das Ergebnis immer lesen können.
- Ein zweiter Abschlussversuch darf das ursprüngliche Ergebnis nicht verändern. Er wird mit einer Warnung protokolliert und ansonsten ignoriert. In Entwicklungsumgebungen soll er zusätzlich als Programmierfehler auffallen.
- Das Abschließen muss alle Wartenden auf diese Aufgabe aufwecken.
- Das Ergebnis muss jederzeit abfragbar sein. Solange nichts gesetzt ist, liefert die Abfrage kein Ergebnis. Bei abgebrochenen Aufgaben gibt es nie ein Ergebnis.
- Das Abschließen soll protokolliert werden.
- Für die Teiloperationen der drei Operationswarteschlangen gilt abweichend: Deren Fertigstellung wird nicht über das Ergebnis der Zustandseinheit verfolgt, sondern über einen externen Zähler der zugehörigen Aufgabe.

## Warten auf Abschluss

- Ein Aufrufer muss asynchron darauf warten können, dass eine Aufgabe abgeschlossen oder abgebrochen ist.
- Ist die Aufgabe bereits abgeschlossen oder abgebrochen, muss das Warten sofort enden.
- Es darf keine Benachrichtigung verloren gehen. Das gilt auch, wenn der Abschluss zwischen Zustandsprüfung und Beginn des Wartens eintritt.
- Mehrere Wartende auf dieselbe Aufgabe müssen gleichzeitig unterstützt werden. Sie werden bei Abschluss oder Abbruch alle aufgeweckt.
- Ein Aufwecken ohne erreichten Endzustand darf das Warten nicht beenden, sondern führt zu erneuter Prüfung und weiterem Warten.
- Das Warten blockiert keinen Ausführungsstrang.

## Einsatz als Aufgaben- und Operationswarteschlangen

- Es muss eine Aufgabenwarteschlange geben, deren Metadaten- und Nutzlasttypen sowie Ergebnis- und Fehlertypen vom Aufrufer bestimmt werden. Die Zustandseinheit jeder Aufgabe trägt genau ein Ergebnis, das nur vom Gewinner des Beanspruchens gesetzt wird.
- Es muss zwei Operationswarteschlangen geben: je eine für Rechenprozessor und Grafikprozessor sowie eine gemeinsame. Sie verwenden denselben Aufbau wie die Aufgabenwarteschlange, aber eigene Metadaten und Werte als Nutzlast.
- Eine Aubengine muss zuerst die eigene Operationswarteschlange prüfen und erst danach auf die gemeinsame zurückgreifen.
- Jede Warteschlange muss unabhängig von den anderen arbeiten und ihre eigene Menge von Benachrichtigungskanälen haben.

## Speicherfreigabe und Robustheit

- Beim Verwerfen einer Warteschlange müssen alle Einträge freigegeben werden.
- Das Verwerfen muss auch bei sehr langen Ketten mit mehr als hunderttausend Einträgen ohne Überlauf des Aufrufspeichers gelingen.
- Wird ein Teil der Kette noch von einem anderen Besitzer gehalten, bleibt dieser Teil gültig und wird erst von dessen letztem Besitzer freigegeben.
- Keine Operation der Warteschlange darf eine Sperre auf der Kette selbst benötigen. Es gibt keine Versionen und keine Momentaufnahmen der Kette. Lediglich die Nutzlast eines einzelnen Eintrags ist kurzzeitig gegen gleichzeitigen Zugriff geschützt.
