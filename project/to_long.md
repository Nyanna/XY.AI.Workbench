Die Knoten ID's der AST Markdown Engine ist zu lang. Es empfiehlt sich die Abkürzung auf ein Hash Basis und das potenziell nur auf eine bestimmte Länge um Eindeutigkeit zu erreichen und Stabilität zu erhöhen. Eindeutig muss nur pro Datei sein.

`/home/user/xyan/xy.ai.workbench/ast-engines/markdown`

Beispiel:
```yaml
id: 866d-e1-e0
result:
  structuredContent:
    edit_block:
      results:
      - path: /home/user/xyan/xy.ai.workbench/rag/project/resident_02/23_batch/specs/119d_offene_punkte.md
        id: Spezifikation_Offene_Punkte_zum_Abgleich_bis_119_Build_Order_119d.4_Behavior_and_invariants.2d3sDf|iqsdHl
        new_id: Spezifikation_Offene_Punkte_zum_Abgleich_bis_119_Build_Order_119d.4_Behavior_and_invariants.2d3sDf|6kXMse
      - path: /home/user/xyan/xy.ai.workbench/rag/project/resident_02/23_batch/specs/119d_offene_punkte.md
        id: Spezifikation_Offene_Punkte_zum_Abgleich_bis_119_Build_Order_119d.4_Behavior_and_invariants.2d3sDf|iqsdHl
        new_id: Spezifikation_Offene_Punkte_zum_Abgleich_bis_119_Build_Order_119d.4_Behavior_and_invariants.2d3sDf|D1a8QO
      errors:
      - path: /home/user/xyan/xy.ai.workbench/rag/project/resident_02/23_batch/specs/119d_offene_punkte.md
        id: Spezifikation_Offene_Punkte_zum_Abgleich_bis_119_Build_Order_119d.4_Behavior_and_invariants.2d3sDf|iqsdHl
        error: Text not found in node.
        reason: not_found
        next_step: reread_node
```