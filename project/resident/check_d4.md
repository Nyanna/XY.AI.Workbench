D1 bis D4 wurden um gesetzt das folgende sind abweichungen aus D4. Prüfe und korrigiere wo möglich. Erkläre mir die Sache mit den python referenzwerten, ist das einmalig notwendig oder dauerhaft?

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben
- nvcc fehlt nicht und ist im Projekt konfiguriert

Noch offen:
- Es gibt weiterhin keine Python-Referenzwerte, weil `sentence-transformers` fehlt.
- Der Zero-Copy-Pfad über den Device-Pointer ist nicht umgesetzt.

- **f16 im Candle-BERT:** Der Candle-`BertModel` liefert für e5-small in f16 durchgehend NaN. Das passiert auf CPU und GPU, mit und ohne Maske und auch mit größerem `layer_norm_eps`. Die Ursache habe ich nicht gefunden. Der Adapter rechnet das Backbone deshalb in f32. Die Embeddings werden trotzdem im dtype des Slots gespeichert, auf der GPU also f16. Das weicht von der Vorgabe "GPU rechnet f16" ab. Die Konstante `BACKBONE_DTYPE` steht mit Begründung im Code, das Backbone kann später wieder auf f16 umgestellt werden.
SystemInit:  id=f01755b4-ad74-45e3-b3cb-083cbb1ae906, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst den aktuellen Stand des Encoder-Codes an.
Control Request:
```yaml
id: 6bcd-ac
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    - /home/user/xyan/xy.ai.workbench/rag/tests
    - /home/user/xyan/xy.ai.workbench/rag/benches
    pattern: BACKBONE_DTYPE|sentence.transformers|reference|device_ptr|layer_norm_eps
    include:
    - '*.rs'
    - '*.toml'
    - '*.json'
    limit: 40
  with_lineno: true
  reason: "Locate f16 backbone constant, reference-value handling and pointer path in code"
```
/answer 6bcd-ac allow