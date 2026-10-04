Die YAML Sortierung erzeugt eine falsche Reihenfolge: `/home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs`

```yaml
- File: src/layers/trigram/trie.rs
  Lines:
    1: '...-4 trie for sets of integer keys (trigram IDs).'
    118: '... cells for a set of keys.'
    121: '    keys: impl IntoIterator<...'
    124: '    for k in keys {'
    139: '... form from a set of keys.'
    144: '        keys: impl IntoIterator<...'
    146: '...rted_cells(&params, keys)?;'
    3: '...e same set of `u32` keys:'
    327: '... mask directly from keys.'
    328: '    pub fn from_keys('
```
Control Request:
```yaml
id: 7c4c-8f
toolName: ask_user
arguments:
  question: Welche Datei bzw. welcher Code-Bereich ist betroffen? Bitte Pfad oder Beschreibung der Sortierfunktion nennen.
  reason: Need to locate the sorting code the user refers to before fixing it.
```
/answer 7c4c-8f allow ist doch angegeben
SystemInit:  id=026aa79b-eaff-41db-9869-bb521b17afa9, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5
Control Request:
```yaml
id: 7c4c-90
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs
    id: numeric_keys_for_lines
    old_text: |2-
                          if let serde_yaml::Value::Mapping(lines) = val {
                              *lines = lines
                                  .iter()
                                  .map(|(k, v)| {
                                      let key = k
                                          .as_str()
                                          .and_then(|s| s.parse::<i64>().ok())
                                          .map(serde_yaml::Value::from)
                                          .unwrap_or_else(|| k.clone());
                                      (key, v.clone())
                                  })
                                  .collect();
                          }
    new_text: |2-
                          if let serde_yaml::Value::Mapping(lines) = val {
                              let mut entries: Vec<(i64, serde_yaml::Value, serde_yaml::Value)> = lines
                                  .iter()
                                  .filter_map(|(k, v)| {
                                      let n = k.as_str()?.parse::<i64>().ok()?;
                                      Some((n, serde_yaml::Value::from(n), v.clone()))
                                  })
                                  .collect();
                              entries.sort_by_key(|(n, _, _)| *n);
                              *lines = entries.into_iter().map(|(_, k, v)| (k, v)).collect();
                          }
  reason: Fix lexicographic string-key sort bug; sort Lines entries numerically by parsed line number before emitting integer keys"
```