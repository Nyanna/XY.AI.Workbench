- Alle Anforderungen in Stichpunkte umformulieren

- Textuelle Anforderungen aufbereiten (01_requirements)
- Anforderungen per Reverse Engineering aus dem Code ableiten (02_reverse_engineered)
- Alle textuellen Anforderungen zu einem Dokument zusammenführen und vereinheitlichen (03_merge_requirements.md, 03_requirements/function_all.md)
- Technische und textuelle Anforderungen zusammenführen (04_add_technical.md)
- Formulierungen korrigieren und optimieren (05_optimize_requirements.md -> 05_corpus_reformulated_terms.md)

- Beide Bestände zu einem Korpus verschmelzen (04_corpus_combined.md)
- Korpus optimieren und umformulieren (05_corpus_reformulated_terms.md)

- Modulbaum ableiten (YAML, 12_create_modules.md -> 12_modules_raw.yml)
- Modulbaum optimieren (13_optimize_modules.md -> 13_modules_optimized.yml)
- Modulbaum prüfen (13_tree_visual.py)

- Korpus in Anforderungssegmente aufteilen (14_split_corpus.py -> 14_split_corpus)
- Segmente in Verzeichnissen gruppieren (manuell oder per Skript nach Dateigröße)
- Anforderungen segmentweise den Modulen zuordnen (15_korpus2modul.md)
- Anforderungen aller Module zusammenführen (15_all_reqs.yml -> 15_merge_req.py -> 15_all_reqs_merged.yml)
- Nach dem Zusammenführen wieder in Dateien mit ca. 300 Zeilen aufteilen (15_corpus2modules)

- Anforderungsmenge deduplizieren und optimieren (16_req_dedup.md -> 16_modules_optimized)
- Zur Übersicht erneut in einer gemeinsamen Liste zusammenführen (16_all_reqs_merged_dedup.yml)

- Modulabhängigkeiten segmentweise anhand des Komponentenbaums zuordnen (17_add_dependencies.md)
- Fertige Modulliste mit Abhängigkeiten zusammenführen (18_modules_deps.yml)
- Zyklen prüfen (19_graph.py -> 19_cyles.yml)
- Zyklen manuell und mit Agent auflösen (20_resolve_cycle.md -> 20_cycles_solved.yml)
- Korrekturen in die Datei übernehmen (18_modules_deps.yml)

- Spezifikationsplan mit Baum erstellen (19_graph.py; 18_modules_deps.yml + 13_modules_optimized.yml -> 20_plan_order.md)
	- Reihenfolge der Spezifikation aggregiert ausgeben, mit drei Eingaben (21_build_order.py -> 21_modules_ordered.yml)
- Batch zur Erstellung der Spezifikationen starten (23_batch/batch_specs.py -> 23_batch/specs)

- Implementierungslauf
...