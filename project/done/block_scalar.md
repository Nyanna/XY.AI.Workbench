Prüfe was hier die Scalar Block Darstellung in YAML verhindert:

```yaml
  content: "package ch.swissmedic.swissdamed.domain;\n\nimport ch.swissmedic.swissdamed.domain.exception.DomainFieldValidationException;\nimport java.util.regex.Pattern;\n\npublic final class RegExpValidator {\n\n    public static final String NUMERIC_PATTERN_MIN_3_CHARS = \"\\\\d{3,}\";\n    \n    private static final String ERROR_CODE_PREFIX = \"validation.\";\n    private static final String PATTERN_POSTFIX = \".pattern\";\n\n    private RegExpValidator() {\n    }\n\n    /**\n     * Validates that the {@code fieldValue} matches the provided {@code regExp} pattern\n     *\n     * @param fieldValue the value to be checked\n     * @param fieldName  name of the field, to be used in thrown exception if validation fails\n     * @param regExp     the regular expression pattern that the field value must match\n     * @throws DomainFieldValidationException if the validation fails\n     */\n    public static void validate(String fieldValue, String fieldName, String regExp) {\n        Validate.notNull(fieldValue, fieldName);\n        Validate.notNull(regExp, \"regExp\");\n\n        if (!Pattern.matches(regExp, fieldValue)) {\n            String message = \"%s must match pattern: %s\".formatted(fieldName, regExp);\n            throw new DomainFieldValidationException(message,\n                    ERROR_CODE_PREFIX + fieldName + PATTERN_POSTFIX);\n        }\n    }\n\n    /**\n     * Validates that the {@code fieldValue} matches the provided {@code regExp} pattern or is {@code null}\n     *\n     * @param fieldValue the value to be checked, can be {@code null}\n     * @param fieldName  name of the field, to be used in thrown exception if validation fails\n     * @param regExp     the regular expression pattern that the field value must match\n     * @throws DomainFieldValidationException if the validation fails\n     */\n    public static void validateNullable(String fieldValue, String fieldName, String regExp) {\n        if (fieldValue != null) {\n            validate(fieldValue, fieldName, regExp);\n        }\n    }\n\n    /**\n     * Validates that the {@code fieldValue} is numeric with at least 3 characters\n     *\n     * @param fieldValue the value to be checked\n     * @param fieldName  name of the field, to be used in thrown exception if validation fails\n     * @throws DomainFieldValidationException if the validation fails\n     */\n    public static void validateNumericMinLength(String fieldValue, String fieldName) {\n        validate(fieldValue, fieldName, NUMERIC_PATTERN_MIN_3_CHARS);\n    }\n\n    /**\n     * Validates that the {@code fieldValue} is numeric with at least 3 characters or is {@code null}\n     *\n     * @param fieldValue the value to be checked, can be {@code null}\n     * @param fieldName  name of the field, to be used in thrown exception if validation fails\n     * @throws DomainFieldValidationException if the validation fails\n     */\n    public static void validateNumericMinLengthNullable(String fieldValue, String fieldName) {\n        validateNullable(fieldValue, fieldName, NUMERIC_PATTERN_MIN_3_CHARS);\n    }\n}\n"
```

SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=9f9a4be8-ca71-4b37-91d7-ca0f560c5fad | model=claude-sonnet-5 | plugins=
# Warum hier kein Scalar Block (`|` oder `>`) verwendet werden kann

Der Inhalt liegt als **doppelt gequotete YAML-Zeichenkette** vor (`"..."` mit `\n`, `\"`, `\\` als Escape-Sequenzen). Das ist der Grund, warum kein Block-Scalar (Literal `|` oder Folded `>`) zum Einsatz kommt – vermutlich weil der Content **programmatisch generiert** wurde (z.B. via `json.dumps()` oder einem Serializer, der automatisch Double-Quoted-Style wählt), statt bewusst als YAML-Block formatiert zu werden.

## Konkrete Hindernisse für Block-Scalar-Darstellung

1. **Eingebettete doppelte Anführungszeichen (`\"`)**
   Der Code enthält viele String-Literale in Java (z.B. `"validation."`, `"%s must match pattern: %s"`). Diese werden im YAML-String escaped als `\"`. In einem Block-Scalar bräuchten diese **kein Escaping** – dort wären `"` einfach literale Zeichen. Das allein wäre also kein technisches Hindernis, sondern eher ein Hinweis, dass der Generator naiv escaped hat.

2. **Backslashes in Regex-Mustern (`\\\\d{3,}`)**
   Das ist der eigentliche Knackpunkt:
   - Im Java-Code steht `"\\d{3,}"` (ein escapter Backslash für Regex `\d`).
   - In der Double-Quoted-YAML-Darstellung wird daraus `\\\\d{3,}` (jeder `\` wird zu `\\`), weil YAML in Double-Quoted-Strings Backslash-Escaping selbst durchführt.
   - **Im Block-Scalar (`|`) gibt es kein Backslash-Escaping** – Backslashes sind dort literale Zeichen. Das würde bedeuten, der Generator müsste zwischen zwei völlig unterschiedlichen Escape-Regeln unterscheiden (Double-Quoted vs. keine Escapes), was ein einfacher naiver Dumper i.d.R. nicht automatisch löst – er bleibt beim „sicheren“ Default: Double-Quoted-Style.

3. **Mixed Line-Endings/Encoding-Vorsicht**
   Viele YAML-Emitter (z.B. PyYAML mit `default_style` oder Ruby's Psych) wählen automatisch Double-Quoted, sobald der String:
   - Backslashes enthält
   - Sonderzeichen wie Tabs enthält
   - Mit einem für Block-Scalare "gefährlichen" Zeichen beginnt/endet (führende/nachfolgende Leerzeichen, Sonderzeichen am Zeilenanfang wie `-`, `#`, `%`)

   Hier ist entscheidend: Der String enthält **`%s`** (Format-Strings) – das `%` am Zeilenanfang wäre in YAML problematisch, da `%` reserviert ist für YAML-Direktiven. Allerdings steht es hier nicht am Zeilenanfang, sondern mitten im String, was für Block-Scalare unproblematisch wäre. Trotzdem könnte ein simpler Serializer defensiv Double-Quoted wählen, sobald er auf ein `%`-Zeichen trifft.

4. **Der eigentliche Hauptgrund: Automatisierte Generierung ohne Format-Wahl-Logik**
   Die meisten Bibliotheken (JSON-to-YAML-Konverter, generische Serializer wie Jackson-YAML, SnakeYAML mit Default-Flow-Style) entscheiden sich **standardmäßig für Double-Quoted-Style**, sobald ein String:
   - Newlines enthält
   - Sonderzeichen enthält
   
   ...und implementieren **keine Heuristik**, um zu prüfen, ob ein Literal-Block-Style (`|`) sauberer/lesbarer wäre. Das ist reine Tool-Entscheidung, kein YAML-Zwang.

## Fazit

**Kein zwingendes YAML-Sprachhindernis** verbietet hier einen Block-Scalar – der Inhalt (mehrzeiliger Java-Code mit Anführungszeichen und Backslashes) wäre technisch problemlos als `|`-Block darstellbar, sogar **lesbarer**. Der Grund, warum stattdessen Double-Quoted verwendet wird, liegt an:

- der **Serialisierungs-Engine**, die pauschal Double-Quoted für Strings mit Sonderzeichen wählt, ohne Block-Scalar-Fallback zu prüfen
- vermutlich einer **automatisierten Pipeline** (z.B. Export aus einer JSON-Quelle oder direkte String-Serialisierung ohne YAML-optimiertes Escaping)

**Empfehlung:** Falls Lesbarkeit gewünscht ist, manuell in Literal-Block-Style (`|`) umwandeln – dabei entfallen alle `\"`- und `\\`-Escapes, und der Code bleibt 1:1 lesbar (nur die Einrückung muss konsistent gehalten werden).

Es wird Java verwendet
```java

	private final YAMLMapper yaml = YAMLMapper.builder().disable(YAMLGenerator.Feature.WRITE_DOC_START_MARKER)
			.disable(YAMLGenerator.Feature.SPLIT_LINES).enable(YAMLGenerator.Feature.LITERAL_BLOCK_STYLE)
			.enable(YAMLGenerator.Feature.MINIMIZE_QUOTES).build();
```

Die Ausgabe funktioniert in anderen Fällen korrekt. Nur in diesem Fall nicht daher muss es am String liegen.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=9f9a4be8-ca71-4b37-91d7-ca0f560c5fad | model=claude-sonnet-5 | plugins=
ReasoningToken: 2454
Sehr guter Hinweis – das grenzt die Ursache stark ein. Da es sich um Jackson `YAMLMapper` mit `LITERAL_BLOCK_STYLE` handelt, wird die Stildarstellung **nicht von Jackson selbst**, sondern von der darunterliegenden **SnakeYAML-Engine** entschieden (`Emitter.analyzeScalar()`). Jackson setzt zwar `ScalarStyle.LITERAL`, aber SnakeYAML kann das **überstimmen**, wenn der String laut Analyse „nicht blockfähig" ist.

## Der konkrete Auslöser im String

Schau dir diese Stelle im JSON-escapten Content genau an:

```
...NUMERIC_PATTERN_MIN_3_CHARS = \"\\\\d{3,}\";\n    \n    private static final String ERROR_CODE_PREFIX...
```

Dekodiert bedeutet das:

```java
public static final String NUMERIC_PATTERN_MIN_3_CHARS = "\\d{3,}";
    ␣␣␣␣            ← diese Zeile enthält NUR 4 Leerzeichen, dann \n
private static final String ERROR_CODE_PREFIX = "validation.";
```

**Es gibt eine „leere" Zeile, die in Wirklichkeit nicht leer ist, sondern aus 4 Leerzeichen besteht** (`"    \n"`), statt eines echten Leerstrings (`"\n"`).

## Warum das den Block-Style verhindert

SnakeYAMLs Scalar-Analyse (`ScalarAnalysis`) prüft beim Emittieren u. a.:

- Enthält der String eine Zeile, die **nur aus Leerzeichen besteht** oder **trailing whitespace** vor einem Zeilenumbruch hat?
- Falls ja → `allowBlock = false`, weil Block-Scalare (Literal `|`) **Trailing-Whitespace nicht zuverlässig/eindeutig repräsentieren können** (YAML-Parser könnten das beim Re-Parsen nicht sicher von "kein Trailing-Whitespace" unterscheiden bzw. es widerspricht der YAML-Spec-Behandlung von Blank Lines in Block-Scalaren).

Als Konsequenz fällt SnakeYAML (und damit Jackson via `LITERAL_BLOCK_STYLE`) automatisch auf **Double-Quoted-Style** zurück – unabhängig von der Jackson-Konfiguration.

## Warum es „bei anderen Strings" funktioniert

Vermutlich haben andere Java-Files/Content-Strings, die du bisher gedumpt hast, **keine Leerzeilen mit „unsichtbaren“ Leerzeichen** (z. B. weil der Editor/Formatter dort automatisch Trailing Whitespace entfernt hat). Dieser eine String stammt offenbar aus einer Quelle, die **Trailing Whitespace auf sonst leeren Zeilen nicht bereinigt hat** – klassisches Editor-/IDE-Artefakt (z. B. Auto-Indent beim Einfügen einer Leerzeile).

## Lösung

**Vor dem Dumpen den String normalisieren**, z. B.:

```java
String cleaned = content.replaceAll("(?m)[ \\t]+$", ""); 
// entfernt trailing whitespace auf jeder Zeile (auch bei sonst leeren Zeilen)
```

Oder allgemeiner als Utility:

```java
public static String stripTrailingWhitespacePerLine(String input) {
    return Arrays.stream(input.split("\n", -1))
                 .map(line -> line.stripTrailing())
                 .collect(Collectors.joining("\n"));
}
```

Wende das **vor** `yaml.writeValueAsString(...)` auf den `content`-Wert an – dann sollte SnakeYAML den Literal-Block-Style (`|`) korrekt anwenden können, da keine "unsichtbaren" Leerzeichen mehr die Scalar-Analyse blockieren.

**Kurzfassung:** Der String enthält eine augenscheinlich leere Zeile, die tatsächlich 4 Leerzeichen (Trailing Whitespace) enthält. Das macht ihn laut SnakeYAML-Regeln „block-unsicher“, weshalb trotz aktiviertem `LITERAL_BLOCK_STYLE` auf Double-Quoted zurückgefallen wird.

Das geht nicht direkt weil:
```java

	public String toYaml(JsonNode node) {
		if (node == null || node.isMissingNode() || node.isNull())
			return "";
		try {
			return yaml.writeValueAsString(node).stripTrailing();
		} catch (JsonProcessingException e) {
			// Should not happen for a tree that Jackson itself produced; fall back to
			// plain JSON rather than losing the payload.
			LOG.error("Failed to render control item as YAML", e);
			return JsonUtil.pretty(node);
		}
	}
```

SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=9f9a4be8-ca71-4b37-91d7-ca0f560c5fad | model=claude-sonnet-5 | plugins=
ReasoningToken: 822
Du hast recht – der `content`-Wert steckt tief in einem `JsonNode`-Baum, du kannst also nicht einfach einen einzelnen String vorbehandeln. Die Lösung: **den gesamten `JsonNode`-Baum rekursiv durchlaufen** und alle Textknoten (`TextNode`) normalisieren, bevor du sie an SnakeYAML/Jackson übergibst.

## Lösung: Baum rekursiv bereinigen

```java
package ch.swissmedic.swissdamed.yaml;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.node.TextNode;

import java.util.ArrayList;
import java.util.List;
import java.util.regex.Pattern;

public class YamlRenderer {

    private static final Pattern TRAILING_WS_PER_LINE = Pattern.compile("[ \\t]+(?=\\n|$)");

    private final YAMLMapper yaml = /* ... wie gehabt ... */;

    public String toYaml(JsonNode node) {
        if (node == null || node.isMissingNode() || node.isNull())
            return "";
        try {
            JsonNode sanitized = sanitizeWhitespace(node.deepCopy());
            return yaml.writeValueAsString(sanitized).stripTrailing();
        } catch (JsonProcessingException e) {
            LOG.error("Failed to render control item as YAML", e);
            return JsonUtil.pretty(node);
        }
    }

    /**
     * Entfernt zeilenweise Trailing Whitespace in allen TextNode-Werten
     * des Baumes, damit SnakeYAML den Literal-Block-Style (|) korrekt
     * anwenden kann. Ohne diese Bereinigung fällt SnakeYAML bei Strings
     * mit "leeren" Zeilen, die tatsächlich Leerzeichen enthalten,
     * automatisch auf Double-Quoted-Style zurück.
     */
    private JsonNode sanitizeWhitespace(JsonNode node) {
        if (node.isTextual()) {
            return TextNode.valueOf(stripTrailingPerLine(node.textValue()));
        }
        if (node.isObject()) {
            ObjectNode obj = (ObjectNode) node;
            // Feldnamen vorher kopieren, um ConcurrentModification zu vermeiden
            List<String> fieldNames = new ArrayList<>();
            obj.fieldNames().forEachRemaining(fieldNames::add);
            for (String name : fieldNames) {
                obj.set(name, sanitizeWhitespace(obj.get(name)));
            }
            return obj;
        }
        if (node.isArray()) {
            ArrayNode arr = (ArrayNode) node;
            for (int i = 0; i < arr.size(); i++) {
                arr.set(i, sanitizeWhitespace(arr.get(i)));
            }
            return arr;
        }
        // Zahlen, Booleans, null, etc. unverändert lassen
        return node;
    }

    private String stripTrailingPerLine(String input) {
        return TRAILING_WS_PER_LINE.matcher(input).replaceAll("");
    }
}
```

## Wichtige Details

1. **`node.deepCopy()`**: Da `ObjectNode`/`ArrayNode` mutable sind, solltest du den Original-Baum nicht verändern (Seiteneffekte vermeiden), falls er anderweitig noch genutzt wird. Deshalb erst kopieren.

2. **Regex statt `stripTrailing()` pro Zeile**: `stripTrailing()` würde nur am Ende des *gesamten* Strings wirken, nicht pro Zeile. Der Regex `[ \t]+(?=\n|$)` matched Leerzeichen/Tabs, die direkt vor einem `\n` oder am Stringende stehen – das erfasst genau die problematischen „leeren, aber nicht leeren" Zeilen wie `"    \n"`.

3. **`ObjectNode.set()` beim Iterieren**: Da `set()` nur den *Value* eines bestehenden Keys ersetzt (nicht die Key-Menge ändert), wäre direktes Iterieren über `fieldNames()` mit gleichzeitigem `set()` bei Jacksons `LinkedHashMap`-Implementierung meist unproblematisch – trotzdem ist das Kopieren der Feldnamen defensiver und sicherer gegen zukünftige Jackson-Versionsänderungen.

4. **Nur `TextNode` betroffen**: Zahlen, Booleans, `null` bleiben unverändert – nur String-Werte werden normalisiert.

## Ergebnis

Damit wird **vor** jedem YAML-Dump sichergestellt, dass keine „unsichtbaren" Trailing-Whitespace-Zeilen mehr im Baum vorkommen, und SnakeYAML kann `LITERAL_BLOCK_STYLE` zuverlässig für alle mehrzeiligen Strings (wie deinen Java-Code) anwenden – unabhängig davon, wie tief sie im JSON-Baum verschachtelt sind.
