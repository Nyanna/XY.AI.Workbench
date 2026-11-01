- Umformulieren aller Anforderungen in Stichpunkte

- textuelle Anforderungen (01_requirements)
- reverse Engineering von Anforderungen aus code (02_reverse_engineered)
- Mergen und vereinheitlichen aller textuellen Anforderungen zu einem Dokument (03_merge_requirements.md, 03_requirements/function_all.md)
- Technische und textuelle Zusammenführen (04_add_technical.md)
- Formulierungen korrigieren und Optimieren (05_optimize_requirements.md -> 05_corpus_reformulated_terms.md)

- beide verschmelzen zu Korpus (04_corpus_combined.md)
- Korpus optimieren und Umformulieren (05_corpus_reformulated_terms.md)

- Modulbaum Ableiten (YAML, 12_create_modules.md -> 12_modules_raw.yml)
- Modulbaum Optimieren (13_optimize_modules.md -> 13_modules_optimized.yml)
- Modulbaum Review (13_tree_visual.py)

- Korpus Anforderungen Segmentieren (14_split_corpus.py -> 14_split_corpus)
- Segmente in Verzeichnissen Gruppieren (Manuell oder Script nach Dateigröße)
- Korpus Anforderungen Segmentweise Modulen zuordnen (15_korpus2modul.md)
- Anforderungen aller Module mergen (15_all_reqs.yml -> 15_merge_req.py -> 15_all_reqs_merged.yml)
- Nach dem mergen wieder aufteilen in ~300 Zeilen (15_corpus2modules)

- Anforderungsmenge Deduplizieren und Optimieren (16_req_dedup.md -> 16_modules_optimized)
- Für Übersicht wieder in gemeinsame liste kopieren (16_all_reqs_merged_dedup.yml)

- Segmentweise mit Komponentenbaum Modulen Abhängigkeiten zuordnen (17_add_dependencies.md)
- Fertige Modulliste mit Abhängigkeiten zusammenkopieren (18_modules_deps.yml)
- Zyklen prüfen (19_graph.py -> 19_cyles.yml)
- Zyklen manuell und mit Agent auflösen (20_resolve_cycle.md -> 20_cycles_solved.yml)
- in Datei korrigieren (18_modules_deps.yml)

- Spezifikationsplan 19_graph.py mit baum  (18_modules_deps.yml + 13_modules_optimized.yml -> 20_plan_order.md)
	- 21_build_order.py mit drei eingaben um Spezifikationsreihenfolge aggregiert auszugeben (21_modules_ordered.yml)
- Batch starten zum erstellen der Spezifikationen

- Baum für Umsetzung Segmentieren und in harte Interface Verträge umwandeln
...