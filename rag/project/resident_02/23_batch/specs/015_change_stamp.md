# Spezifikation: Change-Stamp (Build-Reihenfolge 15)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe:** Rust-Modul (Library), das je Schreib- und Löschoperation eines Index einen eindeutigen, opaken Stamp vergibt. Es führt den Max-Stamp und setzt ihn beim Wiederaufnehmen eines Index fort.

**Grenzen:**
- Es gibt keine Abhängigkeiten, auch nicht zu anderen Modulen des Systems. Nur `std` ist erlaubt.
- Nicht Teil des Moduls: Persistenz, Notification, Locking und Publication, das Erkennen des exklusiven Write-Abschnitts und das Ermitteln des höchsten vorkommenden Stamps. Der Aufrufer liefert Letzteres beim Wiederaufnehmen als Wert.
- Das Modul enthält keine Ordnungs-API für Stamps.

## 2. Kontext

Jeder Index hat genau einen `StampCounter`. Der Write-Pfad ruft `next()` im exklusiven Write-Abschnitt unmittelbar vor der Publication auf. Das gilt für jedes Schreiben und jedes Entfernen. Dokumente und Einträge tragen den Stamp als Veränderungsmarker.

Consumer vergleichen Stamps nur auf Gleichheit. Persistenz und Notification nutzen die Rohdarstellung (`u64`). Beim Wiederaufnehmen übergibt der Aufrufer den gespeicherten Max-Stamp und den höchsten tatsächlich vorkommenden Stamp.

## 3. Interface

```rust
/// Opaker Veränderungsmarker. Nur Gleichheit ist semantisch erlaubt.
/// Implementiert NICHT PartialOrd/Ord.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChangeStamp(/* u64, privat */);

impl ChangeStamp {
    /// Rohdarstellung für Persistenz/Notification. Immer >= 1 bei vergebenen Stamps.
    pub fn as_raw(&self) -> u64;
    /// Rekonstruiert aus Rohwert (z. B. aus Persistenz). Kein Prüfen auf Vergabe.
    pub fn from_raw(raw: u64) -> ChangeStamp;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StampError {
    /// Zähler erschöpft (u64::MAX bereits vergeben).
    Exhausted,
}
impl std::fmt::Display for StampError {}
impl std::error::Error for StampError {}

/// Zähler pro Index. Send + Sync.
#[derive(Debug)]
pub struct StampCounter { /* AtomicU64: zuletzt vergebener Rohwert, 0 = leer */ }

impl StampCounter {
    /// Neuer Index: leerer Zähler, erster vergebener Stamp hat Rohwert 1.
    pub fn new() -> StampCounter;

    /// Wiederaufnehmen. Beide Werte sind Rohwerte; None = nicht vorhanden.
    /// Fortsetzung oberhalb von max(stored_max, highest_present).
    pub fn resume(stored_max: Option<u64>, highest_present: Option<u64>) -> StampCounter;

    /// Vergibt einen neuen Stamp (&self, thread-sicher).
    pub fn next(&self) -> Result<ChangeStamp, StampError>;

    /// Größter bisher vergebene/fortgesetzte Stamp; None bei leerem Zähler.
    pub fn max(&self) -> Option<ChangeStamp>;

    /// Rohdarstellung des Max-Stamps; None bei leerem Zähler (entspricht null).
    pub fn max_raw(&self) -> Option<u64>;
}

impl Default for StampCounter {} // wie new()
```

Es gibt `static`-Assertions bzw. Tests, dass `StampCounter: Send + Sync` und `ChangeStamp: Send + Sync + Copy` gelten.

## 4. Verhalten und Invarianten

- `new()`: `max()` und `max_raw()` sind `None`. Das erste `next()` liefert Rohwert 1, das zweite 2, und so weiter. Der Zähler wächst um genau 1.
- `next()`: Jeder erfolgreiche Aufruf liefert einen Stamp, der sich von allen zuvor von diesem Zähler vergebenen Stamps unterscheidet. Das gilt auch bei gleichzeitigen Aufrufen aus mehreren Threads, die Vergabe ist atomar (z. B. `fetch_update` mit `checked_add`). Es entstehen weder Duplikate noch Lücken durch Races.
- Stamps werden für Schreiben und Löschen gleich vergeben. Eine später neu angelegte gleiche ID erhält daher einen anderen Stamp als vor ihrem Entfernen, weil der Zähler nie zurückgesetzt wird und nie Werte wiederverwendet.
- Der Aufrufer ruft `next()` im exklusiven Write-Abschnitt unmittelbar vor der Publication auf. Das Modul erzwingt das nicht, bleibt aber auch ohne diese Disziplin eindeutig.
- `max()` liefert den zuletzt vergebenen Stamp, der zugleich der größte ist. Nach `next()` gilt `max() == Some(zurückgegebener Stamp)`. `max_raw() == max().map(|s| s.as_raw())`.
- `resume(stored, highest)`:
  - Sei `base = max(stored.unwrap_or(0), highest.unwrap_or(0))`.
  - Gilt `base == 0`, verhält sich der Zähler wie `new()`.
  - Sonst liefert `max_raw()` den Wert `Some(base)`, und das nächste `next()` liefert Rohwert `base + 1`. Der nächste Stamp liegt also strikt oberhalb beider Eingaben.
  - Ein gespeicherter Max-Stamp, der kleiner als der höchste vorkommende ist (z. B. nach Absturz), wird dadurch korrekt übersteuert. Umgekehrt gilt das ebenso.
- Erschöpfung: Ist der letzte vergebene Rohwert `u64::MAX`, liefert `next()` `Err(StampError::Exhausted)`. Der Zustand bleibt unverändert, es gibt keinen Überlauf und keinen Panic. Auch weitere Aufrufe liefern `Exhausted`.
- Ein fehlgeschlagenes `next()` verbraucht keinen Stamp.
- `from_raw(x).as_raw() == x`. Gleichheit und Hash beruhen allein auf dem Rohwert.
- Das Modul panict nie. Es gibt keine Ordnungs-Traits (`PartialOrd`, `Ord`) auf `ChangeStamp`, damit Consumer nicht auf Ordnung vergleichen.
- Lese-Zugriffe (`max`, `max_raw`) sind lock-frei und thread-sicher. Sie liefern einen konsistenten Wert, der zu einem Zeitpunkt zwischen zwei `next()`-Aufrufen gehört.

## 5. Relied-on elements

Keine. Dieses Modul hat keine Abhängigkeiten.