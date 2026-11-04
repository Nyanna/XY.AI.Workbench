Im folgenden Beispiel sieht man wie die Rust AST Engine einen eigentlich zusammenhängenden Bereich unterteilt. Prüfe und korrigiere ob sich dieser bis zu einer Maximalgröße von 1500 Zeichen sinnvoll zusammenlegen lässt.

Rust AST: `/home/user/xyan/xy.ai.workbench/ast-engines/rust/src` 

Beispiel:
```yaml
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/l020/l010/mod.rs
  reason: See how modules are declared
  
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/l020/l010/mod.rs
      nodes:
      - id: batching
        signature: pub mod batching;
      - id: background_pool
        signature: pub mod background_pool;
      - id: capability_model
        signature: pub mod capability_model;
      - id: completion_wait
        signature: pub mod completion_wait;
      - id: extensions
        signature: pub mod extensions;
      - id: model
        signature: pub mod model;
      - id: page_advice
        signature: pub mod page_advice;
      - id: queue
        signature: pub mod queue;
      - id: queue_state
        signature: pub mod queue_state;
      - id: statistics
        signature: pub mod statistics;
      - id: PcT08S|E0WfVD
        signature: "pub use completion_wait::{CompletionNotifier, Subscription};"
```
SystemInit:  id=b8953470-62d7-475d-82a4-837c8670899e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 9d3c-2d
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust/src
  reason: Struktur der Rust AST Engine erkunden
```
/answer 9d3c-2d allow
Control Request:
```yaml
id: 9d3c-2e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust/src
      entries:
      - ".: (1 file below)"
      - "./engine: (8 files below)"
      warning: Result reduced to top-level directories the limit of 50 is exceeded (66). Narrow down the result using the 'pattern' regular expression parameter.
```
/answer 9d3c-2e allow
Control Request:
```yaml
id: 9d3c-2f
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine
    pattern: use_declaration|mod_item|signature
    include:
    - '*.rs'
    limit: 50
  reason: "Stellen finden, an denen mod/use Knoten behandelt werden"
```
/answer 9d3c-2f allow
Control Request:
```yaml
id: 9d3c-30
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine
      matches:
      - path: rust_ast_server.rs
        matches:
        - "dto.set_Signature(Some(rust_ast_engine::signature(file, node, 80)));"
      - path: rust_ast_engine.rs
        matches:
        - /// first printed line for a grouped segment -- the Rust analogue of Java's `signature`.
        - "pub fn signature(file: &syn::File, node: &AddressableNode, limit: usize) -> String {"
```
/answer 9d3c-30 allow
Control Request:
```yaml
id: 9d3c-31
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine
    pattern: \.rs$
  reason: Alle Rust-Dateien auflisten
```
/answer 9d3c-31 allow
Control Request:
```yaml
id: 9d3c-32
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine
      entries:
      - ".:"
      - addressable_node.rs
      - ast_engine_exception.rs
      - document_cache.rs
      - mod.rs
      - node_locator.rs
      - node_path.rs
      - rust_ast_engine.rs
      - rust_ast_server.rs
```
/answer 9d3c-32 allow
Control Request:
```yaml
id: 9d3c-33
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine
    pattern: group|Group|segment|merge|MAX_|1500
    include:
    - '*.rs'
    limit: 50
  reason: Gruppierungs-/Segmentlogik finden
```
/answer 9d3c-33 allow
Control Request:
```yaml
id: 9d3c-34
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine
      matches:
      - path: rust_ast_engine.rs
        matches:
        - /// `print(Object)` dispatch between a single `Node` and a grouped `NodeGroup`.
        - "/// Dispatches to a one-line header rendering for a definition, or the group's own"
        - /// first printed line for a grouped segment -- the Rust analogue of Java's `signature`.
        - "/// A grouped segment never carries a doc comment of its own, same as Java's `NodeGroup`."
      - path: node_locator.rs
        matches:
        - "//! group segments capped at [`SEGMENT_MAX_CHARS`] — \"ImportGroup\" for `use` runs,"
        - //! "StatementGroup" otherwise — exactly like the Java/Python/tree-sitter engines'
        - "//! \"imports\"/\"statements\" segments. Function/method bodies are leaves, never expanded."
        - //! and stable content-hash paths (e.g. `"a1B2c3|d4E5f6"`) for anonymous group
        - "//! segments, rebuilt fresh on every call from the live tree."
        - /// A group keeps accumulating siblings until adding the next one would push its (token) length past this many characters.
        - "const SEGMENT_MAX_CHARS: usize = 1000;"
        - fn segment(
        - "/// Hash of the group's whitespace-stripped first/last 20 chars: stays put even when unrelated edits shift the group's interior."
        - "fn anonymous_segment(content: &str, used: &mut HashMap<String, i32>) -> String {"
        - "fn join_id(parent_id: &str, segment: &str) -> String {"
        - segment.to_string()
        - "format!(\"{}.{}\", parent_id, segment)"
        - "fn group_kind(item: &Item) -> &'static str {"
        - "if matches!(item, Item::Use(_)) { \"ImportGroup\" } else { \"StatementGroup\" }"
        - let kind = group_kind(item);
        - "if is_definition_item(cur) || group_kind(cur) != kind {"
        - "if i > start && length + piece > SEGMENT_MAX_CHARS {"
        - add_item_group(
        - "let seg = segment(name.as_deref(), type_name, used);"
        - fn add_item_group(
        - "let seg = anonymous_segment(&content, used);"
        - "let seg = segment(Some(&f.sig.ident.to_string()), \"ImplItemFn\", &mut used);"
        - "let seg = anonymous_segment(&content, &mut used);"
        - "node_type: \"StatementGroup\".to_string(),"
        - "let seg = segment(Some(&f.sig.ident.to_string()), \"TraitItemFn\", &mut used);"
```
/answer 9d3c-34 allow
ReasoningToken: 48
Control Request:
```yaml
id: 9d3c-35
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine
    script:
    - grep -n 'is_definition_item\|SEGMENT_MAX_CHARS\|fn group_kind\|fn add_item_group' node_locator.rs
  reason: Relevante Zeilen lokalisieren
```
/answer 9d3c-35 deny benutze AST
Control Request:
```yaml
id: 9d3c-36
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine/node_locator.rs
  reason: Struktur von node_locator.rs per AST lesen
```
/answer 9d3c-36 allow
Control Request:
```yaml
id: 9d3c-37
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine/node_locator.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Flattens a `syn::File` into every addressable node, in document order — the…"
      - id: KinMBS|VZwsPN
        signature: use std::collections::HashMap;
      - id: 68GTM6|Dio0nu
        signature: /// A group keeps accumulating siblings until adding the next one would push it…
      - id: locate_all
        signature: "pub fn locate_all(file: &File) -> Vec<AddressableNode> {"
      - id: file_attrs_node
        signature: "fn file_attrs_node(attrs: &[Attribute]) -> AddressableNode {"
        docstring: "A single node covering the file's inner attributes (`//!`/`#![..]`), so that"
      - id: line
        signature: "fn line(span: proc_macro2::Span) -> i64 {"
      - id: end_line
        signature: "fn end_line(span: proc_macro2::Span) -> i64 {"
      - id: sanitize
        signature: "fn sanitize(name: &str) -> String {"
      - id: segment
        signature: "fn segment(     name: Option<&str>,     type_name: &str,     used: &mut HashMap…"
      - id: base62_hash
        signature: "fn base62_hash(text: &str, length: usize) -> String {"
      - id: content_hash
        signature: "fn content_hash(content: &str) -> String {"
      - id: content_prefix_hash
        signature: "fn content_prefix_hash(content: &str) -> String {"
        docstring: "Hash of the group's whitespace-stripped first/last 20 chars: stays put even whe…"
      - id: anonymous_segment
        signature: "fn anonymous_segment(content: &str, used: &mut HashMap<String, i32>) -> String {"
      - id: join_id
        signature: "fn join_id(parent_id: &str, segment: &str) -> String {"
      - id: token_len
        signature: "fn token_len(item: &Item) -> usize {"
      - id: is_definition_item
        signature: "fn is_definition_item(item: &Item) -> bool {"
      - id: group_kind
        signature: "fn group_kind(item: &Item) -> &'static str {"
      - id: item_type_name
        signature: "fn item_type_name(item: &Item) -> &'static str {"
      - id: item_name
        signature: "fn item_name(item: &Item) -> Option<String> {"
      - id: impl_name
        signature: "fn impl_name(imp: &syn::ItemImpl) -> String {"
      - id: is_expandable
        signature: "fn is_expandable(item: &Item) -> bool {"
      - id: walk_items
        signature: "fn walk_items(     items: &[Item],     mod_path: Vec<usize>,     parent_type: &…"
      - id: emit_definition
        signature: "fn emit_definition(     items: &[Item],     idx: usize,     mod_path: Vec<usize…"
      - id: add_item_group
        signature: "fn add_item_group(     items: &[Item],     kind: &str,     mod_path: Vec<usize>…"
      - id: walk_impl_items
        signature: "fn walk_impl_items(     items: &[ImplItem],     mod_path: Vec<usize>,     owner…"
      - id: walk_trait_items
        signature: "fn walk_trait_items(     items: &[TraitItem],     mod_path: Vec<usize>,     own…"
```
/answer 9d3c-37 allow
Control Request:
```yaml
id: 9d3c-38
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine/node_locator.rs
    ids:
    - ModuleDoc
    - 68GTM6|Dio0nu
    - token_len
    - is_definition_item
    - group_kind
    - is_expandable
    - walk_items
    - emit_definition
    - add_item_group
  reason: Gruppierungslogik im Detail prüfen
```
/answer 9d3c-38 allow
Control Request:
```yaml
id: 9d3c-39
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine/node_locator.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Flattens a `syn::File` into every addressable node, in document order — the Rust
          //! analogue of Java's `NodeLocator`. Individually addressable "definitions" are
          //! functions, types (struct/enum/union/trait/trait-alias/type-alias), modules and
          //! impls; module/impl/trait bodies recurse, everything else (`use`, `const`,
          //! `static`, `extern crate`, item macros, foreign blocks) is far too fine-grained
          //! to be individually useful, so consecutive runs of it are collapsed into anonymous
          //! group segments capped at [`SEGMENT_MAX_CHARS`] — "ImportGroup" for `use` runs,
          //! "StatementGroup" otherwise — exactly like the Java/Python/tree-sitter engines'
          //! "imports"/"statements" segments. Function/method bodies are leaves, never expanded.
          //!
          //! Ids are name-based structural paths (e.g. `"MyStruct.foo"`) for named definitions,
          //! and stable content-hash paths (e.g. `"a1B2c3|d4E5f6"`) for anonymous group
          //! segments, rebuilt fresh on every call from the live tree.
      - id: 68GTM6|Dio0nu
        type: StatementGroup
        code: |
          /// A group keeps accumulating siblings until adding the next one would push its (token) length past this many characters.
          const SEGMENT_MAX_CHARS: usize = 1000;
          const HASH_ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
      - id: token_len
        type: ItemFn
        code: |
          fn token_len(item: &Item) -> usize {
              quote::quote! {
                  # item
              }
                  .to_string()
                  .len()
          }
      - id: is_definition_item
        type: ItemFn
        code: |
          fn is_definition_item(item: &Item) -> bool {
              matches!(
                  item, Item::Fn(_) | Item::Struct(_) | Item::Enum(_) | Item::Union(_) |
                  Item::Trait(_) | Item::TraitAlias(_) | Item::Type(_) | Item::Mod(_) |
                  Item::Impl(_)
              )
          }
      - id: group_kind
        type: ItemFn
        code: |
          fn group_kind(item: &Item) -> &'static str {
              if matches!(item, Item::Use(_)) { "ImportGroup" } else { "StatementGroup" }
          }
      - id: is_expandable
        type: ItemFn
        code: |
          fn is_expandable(item: &Item) -> bool {
              match item {
                  Item::Mod(m) => {
                      m.content
                          .as_ref()
                          .map(|(_, items)| items.iter().any(is_definition_item))
                          .unwrap_or(false)
                  }
                  Item::Impl(imp) => imp.items.iter().any(|m| matches!(m, ImplItem::Fn(_))),
                  Item::Trait(t) => t.items.iter().any(|m| matches!(m, TraitItem::Fn(_))),
                  _ => false,
              }
          }
      - id: walk_items
        type: ItemFn
        code: |
          fn walk_items(
              items: &[Item],
              mod_path: Vec<usize>,
              parent_type: &str,
              parent_id: &str,
              out: &mut Vec<AddressableNode>,
          ) {
              let mut used: HashMap<String, i32> = HashMap::new();
              let n = items.len();
              let mut i = 0;
              while i < n {
                  let item = &items[i];
                  if is_definition_item(item) {
                      emit_definition(
                          items,
                          i,
                          mod_path.clone(),
                          parent_type,
                          parent_id,
                          &mut used,
                          out,
                      );
                      i += 1;
                      continue;
                  }
                  let kind = group_kind(item);
                  let start = i;
                  let mut length = 0usize;
                  while i < n {
                      let cur = &items[i];
                      if is_definition_item(cur) || group_kind(cur) != kind {
                          break;
                      }
                      let piece = token_len(cur);
                      if i > start && length + piece > SEGMENT_MAX_CHARS {
                          break;
                      }
                      length += piece;
                      i += 1;
                  }
                  add_item_group(
                      items,
                      kind,
                      mod_path.clone(),
                      start,
                      i,
                      parent_type,
                      parent_id,
                      &mut used,
                      out,
                  );
              }
          }
      - id: emit_definition
        type: ItemFn
        code: |
          fn emit_definition(
              items: &[Item],
              idx: usize,
              mod_path: Vec<usize>,
              parent_type: &str,
              parent_id: &str,
              used: &mut HashMap<String, i32>,
              out: &mut Vec<AddressableNode>,
          ) {
              let item = &items[idx];
              let type_name = item_type_name(item);
              let name = item_name(item);
              let seg = segment(name.as_deref(), type_name, used);
              let id = join_id(parent_id, &seg);
              let expandable = is_expandable(item);
              out.push(AddressableNode {
                  id: id.clone(),
                  path: NodePath::single(mod_path.clone(), Owner::None, None, idx),
                  node_type: type_name.to_string(),
                  name: name.clone(),
                  lineno: line(item.span()),
                  end_lineno: end_line(item.span()),
                  parent_type: parent_type.to_string(),
                  expandable,
                  is_definition: true,
              });
              match item {
                  Item::Mod(m) => {
                      if let Some((_, inner)) = &m.content {
                          let mut child_mod_path = mod_path;
                          child_mod_path.push(idx);
                          walk_items(inner, child_mod_path, type_name, &id, out);
                      }
                  }
                  Item::Impl(imp) => {
                      walk_impl_items(&imp.items, mod_path, idx, type_name, &id, out)
                  }
                  Item::Trait(t) => walk_trait_items(&t.items, mod_path, idx, type_name, &id, out),
                  _ => {}
              }
          }
      - id: add_item_group
        type: ItemFn
        code: |
          fn add_item_group(
              items: &[Item],
              kind: &str,
              mod_path: Vec<usize>,
              start: usize,
              end: usize,
              parent_type: &str,
              parent_id: &str,
              used: &mut HashMap<String, i32>,
              out: &mut Vec<AddressableNode>,
          ) {
              let slice = &items[start..end];
              let content = slice
                  .iter()
                  .map(|m| {
                      quote::quote! {
                          # m
                      }
                          .to_string()
                  })
                  .collect::<Vec<_>>()
                  .join("\n");
              let seg = anonymous_segment(&content, used);
              let id = join_id(parent_id, &seg);
              out.push(AddressableNode {
                  id,
                  path: NodePath::range(mod_path, Owner::None, None, start, end),
                  node_type: kind.to_string(),
                  name: None,
                  lineno: line(slice[0].span()),
                  end_lineno: end_line(slice[slice.len() - 1].span()),
                  parent_type: parent_type.to_string(),
                  expandable: false,
                  is_definition: false,
              });
          }
```
/answer 9d3c-39 allow
Thinking:
Ursache gefunden: Die Obergrenze für Segmentzeichen steigt von 1000 auf 1500, und bodylose `pub mod`-Deklarationen werden künftig zusammen mit `use` gruppiert.


Control Request:
```yaml
id: 9d3c-3a
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine/node_locator.rs
    id: 68GTM6|Dio0nu
    old_text: "const SEGMENT_MAX_CHARS: usize = 1000;"
    new_text: "const SEGMENT_MAX_CHARS: usize = 1500;"
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine/node_locator.rs
    id: is_definition_item
    old_text: |-
      Item::Trait(_) | Item::TraitAlias(_) | Item::Type(_) | Item::Mod(_) |
              Item::Impl(_)
          )
    new_text: |-
      Item::Trait(_) | Item::TraitAlias(_) | Item::Type(_) | Item::Impl(_)
          ) || matches!(item, Item::Mod(m) if m.content.is_some())
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine/node_locator.rs
    id: group_kind
    old_text: "if matches!(item, Item::Use(_)) { \"ImportGroup\" } else { \"StatementGroup\" }"
    new_text: |-
      // `use` and body-less `mod x;` declarations form one contiguous header block.
          if matches!(item, Item::Use(_)) || matches!(item, Item::Mod(m) if m.content.is_none()) {
              "ImportGroup"
          } else {
              "StatementGroup"
          }
  reason: "Limit auf 1500, bodylose mods gruppierbar machen und mit use zusammenlegen"
```
/answer 9d3c-3a allow
ReasoningToken: 535
Control Request:
```yaml
id: 9d3c-3b
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/ast-engines/rust
    pattern: Item::Mod|SEGMENT_MAX|1000
    include:
    - '*.rs'
    - '*.md'
    exclude:
    - target/**
    limit: 30
  reason: "Weitere Stellen prüfen, die Item::Mod oder das Limit nutzen"
```
/answer 9d3c-3b allow
Control Request:
```yaml
id: 9d3c-3c
result:
  structuredContent:
    errors:
    - directory:
      - /home/user/xyan/ast-engines/rust
      pattern: Item::Mod|SEGMENT_MAX|1000
      error: Directory not found or not a directory.
```
/answer 9d3c-3c allow
Text:
 Beim Suchpfad hatte ich mich vertippt. Ich suche noch einmal im richtigen Verzeichnis und validiere die Datei.
Control Request:
```yaml
id: 9d3c-3d
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/rust/src
    pattern: Item::Mod|SEGMENT_MAX|is_definition_item
    include:
    - '*.rs'
    limit: 30
  reason: Weitere Nutzungen von Item::Mod prüfen
```
/answer 9d3c-3d allow
Control Request:
```yaml
id: 9d3c-3e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/rust/src
      matches:
      - path: engine/node_path.rs
        matches:
        - "Item::Mod(m) => cur = &m.content.as_ref()?.1,"
        - "Item::Mod(m) => cur = &mut m.content.as_mut()?.1,"
      - path: engine/rust_ast_engine.rs
        matches:
        - "Item::Mod(i) => &i.attrs,"
      - path: engine/node_locator.rs
        matches:
        - "//! group segments capped at [`SEGMENT_MAX_CHARS`] — \"ImportGroup\" for `use` runs,"
        - "const SEGMENT_MAX_CHARS: usize = 1500;"
        - "fn is_definition_item(item: &Item) -> bool {"
        - ") || matches!(item, Item::Mod(m) if m.content.is_some())"
        - "|| matches!(item, Item::Mod(m) if m.content.is_none())"
        - "Item::Mod(_) => \"ItemMod\","
        - "Item::Mod(m) => Some(m.ident.to_string()),"
        - "Item::Mod(m) => {"
        - ".map(|(_, items)| items.iter().any(is_definition_item))"
        - "if is_definition_item(item) {"
        - "if is_definition_item(cur) || group_kind(cur) != kind {"
        - "if i > start && length + piece > SEGMENT_MAX_CHARS {"
```
/answer 9d3c-3e allow
Control Request:
```yaml
id: 9d3c-3f
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine/node_locator.rs
  reason: Syntax nach Änderung prüfen
```
/answer 9d3c-3f allow
Result Stats: id=b8953470-62d7-475d-82a4-837c8670899e, total: 20396, in: 16, out: 3680, reason: 0, read: 73218, write: 16700, start: 1793828067071, end: 1793828074571, time: 7433, total: 40330, io: 0, control: 67, sent: 0, recv: 159
Ich habe die Gruppierung in `node_locator.rs` geändert. Die Datei besteht die Syntaxprüfung. Gebaut und getestet habe ich nichts, und die Engine habe ich nicht neu gestartet. Die Beispieldatei habe ich deshalb nicht erneut gelesen.

**Ursache**
- Bodylose Deklarationen wie `pub mod batching;` galten als „Definition“ (`Item::Mod`). Jede wurde deshalb als eigener Knoten ausgegeben.
- Nur das `pub use …` am Ende wurde als Gruppe behandelt.
- Das Limit `SEGMENT_MAX_CHARS` stand bei 1000, nicht bei 1500.

**Änderungen**
1. `SEGMENT_MAX_CHARS` ist jetzt 1500.
2. `is_definition_item` zählt ein Modul nur noch dann als Definition, wenn es einen Body hat (`mod x { … }`). Solche Module werden weiterhin einzeln ausgegeben und aufgeklappt.
3. `group_kind` ordnet `use` und bodylose `mod x;` derselben Gruppe `ImportGroup` zu. Ein `mod.rs`-Kopf aus `pub mod …;` plus `pub use …;` wird so zu einem Segment, solange er zusammen höchstens 1500 Zeichen lang ist. Bei mehr Text wird er an der Grenze geteilt.

Das Beispiel aus `l010/mod.rs` sollte damit nur noch ein Segment liefern. Der Kommentar am Dateianfang nennt noch „`use` runs“. Ich kann ihn anpassen, wenn du willst.

Als Nächstes solltest du `cargo test` laufen lassen und die Engine neu starten. Tests, die die Knoten-IDs der bisherigen Einzelknoten (zum Beispiel `batching`) erwarten, müssen angepasst werden. Die IDs der Gruppen sind Content-Hashes.
Lasse die tests laufen, generiere ein release build und erstelle ein Buildscript ebenso.
SystemInit:  id=b8953470-62d7-475d-82a4-837c8670899e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 74
Control Request:
```yaml
id: 9d3c-40
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/ast-engines
    pattern: \.(sh|ps1|bat)$
  reason: Projektlayout und vorhandene Buildscripts prüfen
```
/answer 9d3c-40 allow
Control Request:
```yaml
id: 9d3c-41
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines
      entries:
      - "./java:"
      - run.sh
      - "./java/scripts:"
      - openapi_generate.sh
      - "./rust:"
      - run.sh
```
/answer 9d3c-41 allow
Control Request:
```yaml
id: 9d3c-42
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/ast-engines/rust
    script:
    - cat run.sh
    - cat Cargo.toml | head -30
  reason: "Vorhandenes run.sh und Cargo.toml lesen, um Buildscript passend zu gestalten"
```
/answer 9d3c-42 allow
Control Request:
```yaml
id: 9d3c-43
result:
  structuredContent:
    results:
    - stdout: |
        #!/usr/bin/env bash
        set -euo pipefail

        SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
        cd "$SCRIPT_DIR"

        if [[ "${1:-}" == "--build" ]]; then
          cargo build --release
          shift
        fi

        exec ./.bin/release/xy_ai_ast_rust "$@"
        [package]
        name = "xy_ai_ast_rust"
        version = "0.1.0"
        edition = "2024"

        [dependencies]
        serde_json = "1"
        regex = "1"
        syn = { version = "3", default-features = false, features = ["full", "extra-traits", "visit", "visit-mut", "clone-impls", "printing", "parsing"] }
        quote = "1"
        proc-macro2 = { version = "1", features = ["span-locations"] }
        prettyplease = "0.3"
        sha2 = "0.10"
        sha1 = "0.10"
```
/answer 9d3c-43 allow
Control Request:
```yaml
id: 9d3c-44
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust/build.sh
    source: |
      #!/usr/bin/env bash
      # Builds the Rust AST engine: runs the tests, then produces the release binary.
      # Usage: ./build.sh [--skip-tests]
      set -euo pipefail

      SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
      cd "$SCRIPT_DIR"

      SKIP_TESTS=false
      for arg in "$@"; do
        case "$arg" in
          --skip-tests) SKIP_TESTS=true ;;
          -h|--help)
            echo "Usage: $0 [--skip-tests]"
            exit 0
            ;;
          *)
            echo "Unknown argument: $arg" >&2
            exit 1
            ;;
        esac
      done

      if [[ "$SKIP_TESTS" == false ]]; then
        echo "==> cargo test"
        cargo test
      fi

      echo "==> cargo build --release"
      cargo build --release

      BIN="./.bin/release/xy_ai_ast_rust"
      if [[ -x "$BIN" ]]; then
        echo "==> Built: $SCRIPT_DIR/${BIN#./}"
      else
        echo "==> Build finished (binary not at $BIN; check target-dir config)"
      fi
  reason: Buildscript erstellen
```
/answer 9d3c-44 allow
ReasoningToken: 41
Control Request:
```yaml
id: 9d3c-45
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/ast-engines/rust
    script:
    - cargo test 2>&1 | tail -40
  reason: Tests ausführen
```
/answer 9d3c-45 allow
Control Request:
```yaml
id: 9d3c-46
result:
  structuredContent:
    results:
    - stdout: |2+
           Compiling xy_ai_ast_rust v0.1.0 (/home/user/xyan/xy.ai.workbench/ast-engines/rust)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.61s
             Running unittests src/main.rs (.bin/debug/deps/xy_ai_ast_rust-aa4e3dc525dd594f)

        running 3 tests
        test engine::rust_ast_engine::parse_inner_attrs_tests::parses_doc_comments_only ... ok
        test engine::rust_ast_engine::parse_inner_attrs_tests::splits_trailing_code_into_items_without_hanging ... ok
        test engine::rust_ast_engine::replace_file_attrs_tests::replace_module_doc_with_mixed_doc_and_items ... ok

        test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
/answer 9d3c-46 allow
Text:
 Die Tests sind grün. Jetzt kommen das ausführbare Script und der Release-Build.
Control Request:
```yaml
id: 9d3c-47
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/ast-engines/rust
    script:
    - chmod +x build.sh
    - ./build.sh --skip-tests 2>&1 | tail -15
    - ls -la .bin/release/xy_ai_ast_rust
  reason: Script ausführbar machen und Release-Build über das Script erzeugen
```
/answer 9d3c-47 allow
Control Request:
```yaml
id: 9d3c-48
result:
  structuredContent:
    results:
    - stdout: |
        ==> cargo build --release
           Compiling xy_ai_ast_rust v0.1.0 (/home/user/xyan/xy.ai.workbench/ast-engines/rust)
            Finished `release` profile [optimized] target(s) in 2.45s
        ==> Built: /home/user/xyan/xy.ai.workbench/ast-engines/rust/.bin/release/xy_ai_ast_rust
        -rwxrwxr-x 2 user user 6220616 Nov  4 22:37 .bin/release/xy_ai_ast_rust
```
/answer 9d3c-48 allow
Result Stats: id=b8953470-62d7-475d-82a4-837c8670899e, total: 26010, in: 26, out: 5679, reason: 0, read: 163563, write: 20305, start: 1793828227668, end: 1793828232446, time: 4720, total: 63153, io: 0, control: 58, sent: 0, recv: 111
Die Tests laufen durch, der Release-Build ist erstellt und das Buildscript liegt bereit.

- **Tests:** `cargo test` meldet 3 von 3 bestanden. Keiner dieser Tests deckt die neue Gruppierung von `mod`- und `use`-Zeilen ab.
- **Release-Build:** Das Binary liegt unter `ast-engines/rust/.bin/release/xy_ai_ast_rust` (6,2 MB). Das ist derselbe Pfad, den `run.sh` startet.
- **Buildscript:** `ast-engines/rust/build.sh` führt erst `cargo test` aus und baut dann mit `cargo build --release`. Mit `--skip-tests` überspringt es die Tests. Ich habe den Release-Build über das Script mit `--skip-tests` getestet, weil ich die Tests direkt davor schon einzeln ausgeführt hatte. Der Lauf ohne diese Option ist deshalb nicht als Ganzes getestet.

Ich habe noch nicht geprüft, ob die Engine das `mod.rs`-Beispiel jetzt als ein Segment ausgibt. Dafür müsstest du die Engine neu starten und `ast_read` auf der Beispieldatei erneut ausführen. Ich kann auch einen Unit-Test für `walk_items` ergänzen, der genau diesen Fall absichert.
Ich habe neu gestartet
SystemInit:  id=b8953470-62d7-475d-82a4-837c8670899e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 07b7-1
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/l020/l010/mod.rs
  reason: "Prüfen, ob die mod/use-Zeilen nach dem Neustart als ein Segment ausgegeben werden"
```
/answer 07b7-1 allow
Control Request:
```yaml
id: 07b7-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/l020/l010/mod.rs
      nodes:
      - id: GgJNEQ|4EWCy3
        signature: pub mod batching;
```
/answer 07b7-2 allow
Result Stats: id=b8953470-62d7-475d-82a4-837c8670899e, total: 27291, in: 30, out: 6080, reason: 0, read: 204698, write: 21181, start: 1793828283662, end: 1793828286276, time: 2563, total: 69950, io: 0, control: 51, sent: 0, recv: 47
Das Beispiel wird jetzt als ein einziger Knoten ausgegeben (`GgJNEQ|4EWCy3`). Vorher waren es 11. Die Signatur zeigt nur die erste Zeile (`pub mod batching;`), das ist bei Gruppen so vorgesehen.

Ich habe nicht geprüft, ob der Knoten wirklich alle 11 Zeilen enthält. Dazu kann ich ihn per ID lesen. Ich kann außerdem einen Unit-Test ergänzen, damit das Verhalten abgesichert ist.
Tue das
SystemInit:  id=b8953470-62d7-475d-82a4-837c8670899e, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 07b7-3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/l020/l010/mod.rs
    ids:
    - GgJNEQ|4EWCy3
  reason: Inhalt des Gruppenknotens prüfen
```
/answer 07b7-3 allow
Control Request:
```yaml
id: 07b7-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/l020/l010/mod.rs
      nodes:
      - id: GgJNEQ|4EWCy3
        type: ImportGroup
        code: |
          pub mod batching;
          pub mod background_pool;
          pub mod capability_model;
          pub mod completion_wait;
          pub mod extensions;
          pub mod free_space;
          pub mod model;
          pub mod page_advice;
          pub mod queue;
          pub mod queue_state;
          pub mod statistics;
          pub use completion_wait::{CompletionNotifier, Subscription};
          pub use extensions::Extensions;
          pub use background_pool::{BackgroundPool, JobError, JobFuture, PoolError};
          pub use statistics::{
              dispersion, estimate, median, minimum, Dispersion, Estimator, MIN_DISPERSION_SAMPLES,
          };
```
/answer 07b7-4 allow
Control Request:
```yaml
id: 07b7-5
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/rust/src/engine/rust_ast_engine.rs
  node_type: Mod|ItemMod
  reason: Bestehende Testmodule und deren Stil finden
```
/answer 07b7-5 allow