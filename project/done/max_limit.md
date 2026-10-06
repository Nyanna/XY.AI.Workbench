Das maximale Result Limit der Layer soll optional über den Query Parameter "maxResults" steuerbar sein (Default ist pro Layer bereits auf 50 beschränkt). In der CLI soll das entsprechend von einem benannten Argument abgebildet sein.

`/home/user/xyan/xy.ai.workbench/rag/src/layers/glob_layer.rs`
`/home/user/xyan/xy.ai.workbench/rag/src/layers/grep_layer.rs`
`/home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs`
`/home/user/xyan/xy.ai.workbench/rag/src/core/query.rs`
SystemInit:  id=33b967fb-da86-491b-87d2-547d4c0c8acb, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5
Control Request:
```yaml
id: a446-1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/layers/glob_layer.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/layers/grep_layer.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/core/query.rs
  reason: Understand structure before implementing maxResults option
```
/answer a446-1 allow
Control Request:
```yaml
id: a446-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/glob_layer.rs
      nodes:
      - id: ModuleDoc
        signature: //! Glob-pattern based directory/file search layer.
      - id: 1dAG1t|IzJ7BW
        signature: "use std::path::{Path, PathBuf};"
      - id: hXrAAw|YF28qS
        signature: /// Max number of directory listings kept in memory at once.
      - id: Candidate
        signature: "/// One candidate found below the search root, prior to glob matching. #[derive…"
        docstring: "One candidate found below the search root, prior to glob matching."
      - id: join_rel
        signature: "fn join_rel(prefix: &str, name: &str) -> String {"
      - id: walk
        signature: "/// Collects every file/directory below `root`, via `cache`. /// /// Traverses …"
        docstring: "Collects every file/directory below `root`, via `cache`."
      - id: wildcard_count
        signature: /// Number of glob meta characters in a pattern - a rough specificity /// signa…
        docstring: Number of glob meta characters in a pattern - a rough specificity
      - id: Guiu6E|DB8rLf
        signature: /// Max length (in chars) of the comma-separated "Files"/"Subdirectories"
      - id: truncate_join
        signature: "/// Joins `names` with \", \", stopping before exceeding `limit` chars. /// Retur…"
        docstring: "Joins `names` with \", \", stopping before exceeding `limit` chars."
      - id: GlobLayer
        signature: /// Glob-pattern based file/directory search layer. /// /// See the module docu…
        docstring: Glob-pattern based file/directory search layer.
      - id: impl_GlobLayer
        signature: "impl GlobLayer {"
        children:
        - id: impl_GlobLayer.new
          signature: "pub fn new() -> Self {"
      - id: impl_Default_for_GlobLayer
        signature: "impl Default for GlobLayer {"
        children:
        - id: impl_Default_for_GlobLayer.default
          signature: "fn default() -> Self {"
      - id: impl_Layer_for_GlobLayer
        signature: "#[async_trait] impl Layer for GlobLayer {"
        children:
        - id: impl_Layer_for_GlobLayer.id
          signature: "fn id(&self) -> &str {"
        - id: impl_Layer_for_GlobLayer.stage
          signature: "fn stage(&self) -> LayerStage {"
        - id: impl_Layer_for_GlobLayer.applies
          signature: "fn applies(&self, query: &Query) -> bool {"
        - id: impl_Layer_for_GlobLayer.run
          signature: "async fn run(     &self,     query: &Query,     result_set: &ResultSet,     _ct…"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/grep_layer.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Grep-style, regex based file-content search layer."
      - id: D7wDOB|QGWOYW
        signature: "use std::collections::{HashMap, VecDeque};"
      - id: bxxdmX|Ap2kWL
        signature: /// Max number of directory listings kept in memory at once.
      - id: FileCandidate
        signature: "/// One file discovered below the search root, prior to content search. struct …"
        docstring: "One file discovered below the search root, prior to content search."
      - id: join_rel
        signature: "fn join_rel(prefix: &str, name: &str) -> String {"
      - id: collect_files
        signature: "/// Collects every regular file below `root`, via `cache`. /// /// Traverses th…"
        docstring: "Collects every regular file below `root`, via `cache`."
      - id: CompiledToken
        signature: "/// A compiled search token together with a rough specificity score. #[derive(C…"
        docstring: A compiled search token together with a rough specificity score.
      - id: regex_specificity
        signature: "fn regex_specificity(pattern: &str) -> usize {"
      - id: LineMatch
        signature: "/// One matching line found in one file. #[derive(Clone)] struct LineMatch {"
        docstring: One matching line found in one file.
      - id: build_excerpt
        signature: /// Builds a combined excerpt for a line given all of its match spans /// (byte…
        docstring: Builds a combined excerpt for a line given all of its match spans
      - id: search_file
        signature: "/// Searches one file for all `tokens`, line by line; CPU-bound, runs on /// th…"
        docstring: "Searches one file for all `tokens`, line by line; CPU-bound, runs on"
      - id: GrepLayer
        signature: "/// Grep-style, regex based file-content search layer. /// /// See the module d…"
        docstring: "Grep-style, regex based file-content search layer."
      - id: impl_GrepLayer
        signature: "impl GrepLayer {"
        children:
        - id: impl_GrepLayer.new
          signature: "pub fn new() -> Self {"
      - id: impl_Default_for_GrepLayer
        signature: "impl Default for GrepLayer {"
        children:
        - id: impl_Default_for_GrepLayer.default
          signature: "fn default() -> Self {"
      - id: impl_Layer_for_GrepLayer
        signature: "#[async_trait] impl Layer for GrepLayer {"
        children:
        - id: impl_Layer_for_GrepLayer.id
          signature: "fn id(&self) -> &str {"
        - id: impl_Layer_for_GrepLayer.stage
          signature: "fn stage(&self) -> LayerStage {"
        - id: impl_Layer_for_GrepLayer.applies
          signature: "fn applies(&self, query: &Query) -> bool {"
        - id: impl_Layer_for_GrepLayer.run
          signature: "async fn run(     &self,     query: &Query,     result_set: &ResultSet,     ctx…"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs
      nodes:
      - id: ModuleDoc
        signature: "//! CLI entry point of the xy.ai.rag engine (on-demand, no daemon)."
      - id: KJlHpU|DpQGE3
        signature: use std::path::Path;
      - id: Cli
        signature: "#[derive(Parser, Debug)] #[command(name = \"xyrag\", about = \"xy.ai.rag - Layered…"
      - id: parse_query
        signature: "fn parse_query(args: &[String], json_query: Option<&str>) -> Result<Query> {"
      - id: build_default_registry
        signature: /// Builds the registry with all known layer implementations. /// /// Concrete …
        docstring: Builds the registry with all known layer implementations.
      - id: status_to_json
        signature: "fn status_to_json(s: &LayerStatus) -> Value {"
      - id: numeric_keys_for_lines
        signature: /// Recursively rewrites `Lines` objects so their (numeric) string keys /// bec…
        docstring: Recursively rewrites `Lines` objects so their (numeric) string keys
      - id: main
        signature: "#[tokio::main] async fn main() -> Result<()> {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/query.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Weakly typed, dynamic query object of the RAG engine."
      - id: fZqUGN|WDDBsn
        signature: "use std::path::{Path, PathBuf};"
      - id: default_document_root
        signature: "fn default_document_root() -> PathBuf {"
      - id: Query
        signature: "/// Dynamic, weakly typed query object. /// /// Layers decide for themselves wh…"
        docstring: "Dynamic, weakly typed query object."
      - id: impl_Default_for_Query
        signature: "impl Default for Query {"
        children:
        - id: impl_Default_for_Query.default
          signature: "fn default() -> Self {"
      - id: impl_Query
        signature: "impl Query {"
        children:
        - id: impl_Query.new
          signature: "pub fn new() -> Self {"
        - id: impl_Query.from_fields
          signature: "pub fn from_fields(fields: Map<String, Value>) -> Self {"
        - id: impl_Query.set_document_root
          signature: "/// Overrides the document root (fallback for `directory`), e.g. with /// the R…"
          docstring: "Overrides the document root (fallback for `directory`), e.g. with"
        - id: impl_Query.document_root
          signature: "pub fn document_root(&self) -> &Path {"
        - id: impl_Query.directory
          signature: "/// Resolves the directory a layer should operate on: the `directory` /// field…"
          docstring: "Resolves the directory a layer should operate on: the `directory`"
        - id: impl_Query.resolve_search_root
          signature: "/// Resolves the actual filesystem directory a layer should search, /// plus th…"
          docstring: "Resolves the actual filesystem directory a layer should search,"
        - id: impl_Query.has
          signature: "pub fn has(&self, field: &str) -> bool {"
        - id: impl_Query.get
          signature: "pub fn get(&self, field: &str) -> Option<&Value> {"
        - id: impl_Query.get_str
          signature: "pub fn get_str(&self, field: &str) -> Option<&str> {"
        - id: impl_Query.inspect
          signature: /// Full copy of the fields for free analysis by layers. pub fn inspect(&self) …
          docstring: Full copy of the fields for free analysis by layers.
        - id: impl_Query.with_fields
          signature: "pub fn with_fields(&self, overrides: Map<String, Value>) -> Query {"
        - id: impl_Query.set
          signature: "pub fn set(&mut self, key: impl Into<String>, value: Value) {"
        - id: impl_Query.fields
          signature: "pub fn fields(&self) -> &Map<String, Value> {"
```
/answer a446-2 allow
ReasoningToken: 13
Control Request:
```yaml
id: a446-3
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/layers/glob_layer.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/layers/grep_layer.rs
  id: impl_Layer_for_GlobLayer.run
  reason: See run method implementations to find result limit logic (50)
```
/answer a446-3 allow
Control Request:
```yaml
id: a446-4
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/glob_layer.rs
      nodes:
      - id: impl_Layer_for_GlobLayer.run
        code: |2
              async fn run(
                  &self,
                  query: &Query,
                  result_set: &ResultSet,
                  _ctx: &LayerContext,
              ) -> LayerStatus {
                  let mut status = LayerStatus::new(self.id(), self.stage());
                  let text = match query.get_str("query") {
                      Some(t) => t,
                      None => {
                          status.skipped = true;
                          return status;
                      }
                  };
                  let tokens: Vec<&str> = text.split_whitespace().collect();
                  if tokens.is_empty() {
                      status.skipped = true;
                      return status;
                  }
                  let (search_root, rel_prefix) = query.resolve_search_root();
                  let cache_stats_before = self.cache.stats();
                  let traversal_start = Instant::now();
                  let mut candidates = Vec::new();
                  walk(&self.cache, &search_root, &rel_prefix, &mut candidates);
                  let traversal_ms = traversal_start.elapsed().as_secs_f64() * 1000.0;
                  let mut hits: Vec<(usize, usize, Candidate)> = Vec::new();
                  for token in &tokens {
                      let pattern = match Pattern::new(token) {
                          Ok(p) => p,
                          Err(_) => continue,
                      };
                      let wc = wildcard_count(token);
                      for cand in &candidates {
                          if pattern.matches(&cand.rel_path) {
                              let rank = if cand.rel_path == *token { 0 } else { 1 };
                              hits.push((rank, wc, cand.clone()));
                          }
                      }
                  }
                  hits.sort_by(|a, b| {
                      a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.rel_path.cmp(&b.2.rel_path))
                  });
                  let mut seen = std::collections::HashSet::new();
                  hits.retain(|(_, _, cand)| seen.insert(cand.rel_path.clone()));
                  hits.truncate(MAX_MATCHES);
                  for (_, _, cand) in &hits {
                      let mut fields = Map::new();
                      if cand.is_dir {
                          fields.insert("Directory".into(), Value::String(cand.rel_path.clone()));
                          if let Ok(listing) = self.cache.list(&cand.abs_path) {
                              let file_names: Vec<String> = listing
                                  .files
                                  .iter()
                                  .map(|f| f.name.clone())
                                  .collect();
                              let (files_str, files_truncated) = truncate_join(
                                  &file_names,
                                  MAX_LIST_CHARS,
                              );
                              fields.insert("Files".into(), Value::String(files_str));
                              if files_truncated && !file_names.is_empty() {
                                  fields.insert("fileCount".into(), json!(file_names.len()));
                              }
                              let (dirs_str, dirs_truncated) = truncate_join(
                                  &listing.dirs,
                                  MAX_LIST_CHARS,
                              );
                              fields.insert("Subdirectories".into(), Value::String(dirs_str));
                              if dirs_truncated && !listing.dirs.is_empty() {
                                  fields
                                      .insert("directoryCount".into(), json!(listing.dirs.len()));
                              }
                          }
                      } else {
                          fields.insert("File".into(), Value::String(cand.rel_path.clone()));
                      }
                      result_set.add(ResultEntry::new(None, fields));
                      status.contributions += 1;
                  }
                  status.ran = true;
                  status.detail.insert("root".into(), json!(search_root.display().to_string()));
                  let cache_delta = self.cache.stats().delta(&cache_stats_before);
                  status.detail.insert("dir_cache_misses".into(), json!(cache_delta.misses));
                  status
                      .detail
                      .insert("dir_cache_update_ms".into(), json!(cache_delta.update_ms()));
                  status.detail.insert("traversal_ms".into(), json!(traversal_ms));
                  status.detail.insert("files_searched".into(), json!(0));
                  status
              }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/grep_layer.rs
      nodes: []
```
/answer a446-4 allow
Control Request:
```yaml
id: a446-5
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: 50
    include:
    - '*.rs'
  reason: Find where the default max result limit of 50 is defined
```
/answer a446-5 allow
Control Request:
```yaml
id: a446-6
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: layers/glob_layer.rs
        matches:
        - lineno: 22
          match: "const MAX_MATCHES: usize = 50;"
      - path: layers/trigram/trie.rs
        matches:
        - lineno: 506
          match: "let trie = CompactTrie::build(params, [1u32, 2, 1000, 50000, 65535]).unwrap();"
      - path: layers/grep_layer.rs
        matches:
        - lineno: 34
          match: "const MAX_MATCHES: usize = 50;"
```
/answer a446-6 allow
ReasoningToken: 71
Control Request:
```yaml
id: a446-7
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/layers/grep_layer.rs
  id: impl_Layer_for_GrepLayer.run
  reason: See grep_layer run method to find where MAX_MATCHES used for truncation
```
/answer a446-7 allow
Control Request:
```yaml
id: a446-8
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/grep_layer.rs
      nodes:
      - id: impl_Layer_for_GrepLayer.run
        code: |2
              async fn run(
                  &self,
                  query: &Query,
                  result_set: &ResultSet,
                  ctx: &LayerContext,
              ) -> LayerStatus {
                  let mut status = LayerStatus::new(self.id(), self.stage());
                  let text = match query.get_str("query") {
                      Some(t) => t,
                      None => {
                          status.skipped = true;
                          return status;
                      }
                  };
                  let tokens: Vec<&str> = text.split_whitespace().collect();
                  if tokens.is_empty() {
                      status.skipped = true;
                      return status;
                  }
                  let compiled: Vec<CompiledToken> = tokens
                      .iter()
                      .filter_map(|t| {
                          Regex::new(t)
                              .ok()
                              .map(|regex| CompiledToken {
                                  regex,
                                  specificity: regex_specificity(t),
                              })
                      })
                      .collect();
                  if compiled.is_empty() {
                      status.skipped = true;
                      return status;
                  }
                  let tokens = Arc::new(compiled);
                  let (search_root, rel_prefix) = query.resolve_search_root();
                  let cache_stats_before = self.cache.stats();
                  let traversal_start = Instant::now();
                  let mut files = Vec::new();
                  collect_files(&self.cache, &search_root, &rel_prefix, &mut files);
                  let traversal_ms = traversal_start.elapsed().as_secs_f64() * 1000.0;
                  let files_searched = files.len();
                  let search_nanos = Arc::new(AtomicU64::new(0));
                  let mut queue: VecDeque<FileCandidate> = files.into();
                  let start = Instant::now();
                  let deadline = start + MAX_RUNTIME;
                  let start_cutoff = start
                      + Duration::from_secs_f64(MAX_RUNTIME.as_secs_f64() * START_CUTOFF_RATIO);
                  let mut pending = FuturesUnordered::new();
                  let mut all_hits: Vec<LineMatch> = Vec::new();
                  let mut aborted = false;
                  loop {
                      while Instant::now() < start_cutoff {
                          let Some(file) = queue.pop_front() else { break };
                          let cpu = ctx.cpu.clone();
                          let tokens = Arc::clone(&tokens);
                          let rel_path = file.rel_path;
                          let abs_path = file.abs_path;
                          let search_nanos = Arc::clone(&search_nanos);
                          pending
                              .push(async move {
                                  let t0 = Instant::now();
                                  let hits = cpu
                                      .spawn(move || search_file(&abs_path, &rel_path, &tokens))
                                      .await
                                      .unwrap_or_default();
                                  search_nanos
                                      .fetch_add(
                                          t0.elapsed().as_nanos() as u64,
                                          Ordering::Relaxed,
                                      );
                                  hits
                              });
                      }
                      if pending.is_empty() {
                          break;
                      }
                      let now = Instant::now();
                      if now >= deadline {
                          aborted = true;
                          break;
                      }
                      tokio::select! {
                          maybe_hits = pending.next() => { if let Some(hits) = maybe_hits {
                          all_hits.extend(hits); } } _ = tokio::time::sleep(deadline - now) => {
                          aborted = true; break; }
                      }
                  }
                  all_hits
                      .sort_by(|a, b| {
                          a.rank
                              .cmp(&b.rank)
                              .then(a.specificity.cmp(&b.specificity))
                              .then(a.rel_path.cmp(&b.rel_path))
                              .then(a.line_no.cmp(&b.line_no))
                      });
                  all_hits.truncate(MAX_MATCHES);
                  let mut order: Vec<String> = Vec::new();
                  let mut grouped: HashMap<String, Vec<(usize, String)>> = HashMap::new();
                  for hit in all_hits {
                      grouped
                          .entry(hit.rel_path.clone())
                          .or_insert_with(|| {
                              order.push(hit.rel_path.clone());
                              Vec::new()
                          })
                          .push((hit.line_no, hit.text));
                  }
                  for rel in order {
                      let mut lines = grouped.remove(&rel).unwrap_or_default();
                      lines.sort_by_key(|(ln, _)| *ln);
                      let mut lines_obj = Map::new();
                      for (ln, text) in lines {
                          lines_obj.insert(ln.to_string(), Value::String(text));
                      }
                      let mut fields = Map::new();
                      fields.insert("File".into(), Value::String(rel));
                      fields.insert("Lines".into(), Value::Object(lines_obj));
                      result_set.add(ResultEntry::new(None, fields));
                      status.contributions += 1;
                  }
                  status.ran = true;
                  status.aborted = aborted;
                  status.detail.insert("root".into(), json!(search_root.display().to_string()));
                  let cache_delta = self.cache.stats().delta(&cache_stats_before);
                  status.detail.insert("dir_cache_misses".into(), json!(cache_delta.misses));
                  status
                      .detail
                      .insert("dir_cache_update_ms".into(), json!(cache_delta.update_ms()));
                  status.detail.insert("traversal_ms".into(), json!(traversal_ms));
                  status.detail.insert("files_searched".into(), json!(files_searched));
                  status
                      .detail
                      .insert(
                          "search_ms".into(),
                          json!(search_nanos.load(Ordering::Relaxed) as f64 / 1_000_000.0),
                      );
                  status
              }
```
/answer a446-8 allow
Control Request:
```yaml
id: a446-9
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/core/query.rs
  id: impl_Query.get_str
  reason: See get_str helper to model a get_usize helper for maxResults
```
/answer a446-9 allow
Control Request:
```yaml
id: a446-a
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/core/query.rs
      nodes:
      - id: impl_Query.get_str
        code: |2
              pub fn get_str(&self, field: &str) -> Option<&str> {
                  self.fields.get(field).and_then(Value::as_str)
              }
```
/answer a446-a allow
ReasoningToken: 25
Control Request:
```yaml
id: a446-b
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs
  id: Cli
  reason: See CLI struct fields to add maxResults named arg"
```
/answer a446-b allow
Control Request:
```yaml
id: a446-c
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs
      nodes:
      - id: Cli
        code: |
          #[derive(Parser, Debug)]
          #[command(name = "xyrag", about = "xy.ai.rag - Layered Anytime Retrieval Engine")]
          struct Cli {
              /// Query fields as key=value
              query: Vec<String>,
              /// Query as a JSON object
              #[arg(long = "json")]
              json_query: Option<String>,
              /// RAG root directory; storage lives at <root>/.xyrag (default: CWD)
              #[arg(long)]
              root: Option<String>,
              /// Execution model of the layer topology
              #[arg(long, default_value = "parallel")]
              mode: String,
              /// Comma-separated glob patterns; only matching names/paths are
              /// returned by searches (wins over --exclude on conflicts)
              #[arg(long, value_delimiter = ',')]
              include: Vec<String>,
              /// Comma-separated glob patterns; matching names/paths are dropped
              /// from searches, unless also matched by --include
              #[arg(long, value_delimiter = ',')]
              exclude: Vec<String>,
          }
```
/answer a446-c allow
Control Request:
```yaml
id: a446-d
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs
  id: parse_query
  reason: See parse_query to add maxResults field mapping"
```
/answer a446-d allow
Control Request:
```yaml
id: a446-e
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs
      nodes:
      - id: parse_query
        code: |
          fn parse_query(args: &[String], json_query: Option<&str>) -> Result<Query> {
              let mut fields = Map::new();
              if let Some(j) = json_query {
                  if let Value::Object(m) = serde_json::from_str(j)? {
                      fields.extend(m);
                  }
              }
              for item in args {
                  let (key, value) = item
                      .split_once('=')
                      .ok_or_else(|| {
                          anyhow!("Invalid Query-Field (exspected key=value): {item}")
                      })?;
                  fields.insert(key.to_string(), Value::String(value.to_string()));
              }
              Ok(Query::from_fields(fields))
          }
```
/answer a446-e allow
ReasoningToken: 327
Control Request:
```yaml
id: a446-f
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs
  id: main
  reason: See main to find how include/exclude cli args map into query fields
```
/answer a446-f allow
Control Request:
```yaml
id: a446-10
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs
      nodes:
      - id: main
        code: |
          #[tokio::main]
          async fn main() -> Result<()> {
              let cli = Cli::parse();
              let mode = ExecutionMode::parse(&cli.mode)
                  .ok_or_else(|| anyhow!("Invalid mode: {}", cli.mode))?;
              let mut query = parse_query(&cli.query, cli.json_query.as_deref())?;
              let registry = build_default_registry();
              let persistence = PersistenceManager::new(cli.root.as_deref().map(Path::new))?;
              query.set_document_root(persistence.root.clone());
              let filter = PathFilter::new(&cli.include, &cli.exclude)
                  .map_err(|e| anyhow!("Invalid --include/--exclude pattern: {}", e))?;
              configure_global_filter(persistence.root.clone(), filter);
              let mut engine = Engine::new(registry, persistence, mode);
              engine.start_background();
              let (result_set, statuses) = engine.run_query(query).await?;
              engine.shutdown(false).await;
              let results: Vec<Value> = result_set
                  .entries()
                  .iter()
                  .map(|e| Value::Object(e.to_dict()))
                  .collect();
              let layers: Vec<Value> = statuses.iter().map(status_to_json).collect();
              let output = json!({ "results" : results, "layers" : layers });
              let mut yaml_value = serde_yaml::to_value(&output)?;
              numeric_keys_for_lines(&mut yaml_value);
              println!("{}", serde_yaml::to_string(& yaml_value) ?.trim_end());
              Ok(())
          }
```
/answer a446-10 allow
ReasoningToken: 23
Control Request:
```yaml
id: a446-11
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/core/query.rs
    id: impl_Query.get_str
    position: after
    source: |2

          /// Resolves a numeric field as `usize`, e.g. `maxResults`, accepting
          /// both JSON numbers and numeric strings (as passed via the CLI).
          pub fn get_usize(&self, field: &str) -> Option<usize> {
              match self.fields.get(field)? {
                  Value::Number(n) => n.as_u64().map(|v| v as usize),
                  Value::String(s) => s.parse::<usize>().ok(),
                  _ => None,
              }
          }
  reason: Add Query::get_usize helper to read maxResults from either CLI string or JSON number fields
```
/answer a446-11 allow
ReasoningToken: 174
Text:
 Now update glob_layer.rs and grep_layer.rs to use the optional override.
Control Request:
```yaml
id: a446-12
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/glob_layer.rs
    old_text: hits.truncate(MAX_MATCHES);
    new_text: |-
      let max_results = query.get_usize("maxResults").unwrap_or(MAX_MATCHES);
              hits.truncate(max_results);
  - path: /home/user/xyan/xy.ai.workbench/rag/src/layers/grep_layer.rs
    old_text: all_hits.truncate(MAX_MATCHES);
    new_text: |-
      let max_results = query.get_usize("maxResults").unwrap_or(MAX_MATCHES);
              all_hits.truncate(max_results);
  reason: Allow maxResults query parameter to override the per-layer default truncation limit
```
/answer a446-12 allow
Text:
 Now add the named CLI argument and wire it into the query.
Control Request:
```yaml
id: a446-13
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs
    old_text: |2-
          #[arg(long, value_delimiter = ',')]
          exclude: Vec<String>,
      }
    new_text: |2-
          #[arg(long, value_delimiter = ',')]
          exclude: Vec<String>,
          /// Overrides the per-layer result limit (default: 50 per layer)
          #[arg(long = "max-results")]
          max_results: Option<u64>,
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs
    old_text: "    let mut query = parse_query(&cli.query, cli.json_query.as_deref())?;"
    new_text: |2-
          let mut query = parse_query(&cli.query, cli.json_query.as_deref())?;
          if let Some(max_results) = cli.max_results {
              query.set("maxResults", json!(max_results));
          }
  reason: Add --max-results named CLI argument and inject it into query fields as maxResults"
```
/answer a446-13 allow
Control Request:
```yaml
id: a446-14
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/layers/glob_layer.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/layers/grep_layer.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/bin/xyrag.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/core/query.rs
  reason: Verify edited files compile syntactically
```
/answer a446-14 allow
Control Request:
```yaml
id: a446-15
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/rag
  script: cargo build 2>&1 | tail -60
  reason: Verify the project compiles after adding maxResults support
```
/answer a446-15 allow
Text:
 Now run cargo build to confirm full compilation.
Control Request:
```yaml
id: a446-16
result:
  structuredContent:
    stdout: |2
         Compiling xy-ai-rag v0.1.0 (/home/user/xyan/xy.ai.workbench/rag)
          Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.25s
```
/answer a446-16 allow
Result Stats: id=33b967fb-da86-491b-87d2-547d4c0c8acb, total: 40066, in: 153, out: 3798, reason: 0, read: 123628, write: 36115
Compiles cleanly. Summary of changes:

- `Query::get_usize` (query.rs): reads a numeric field (JSON number or numeric string) by name.
- `glob_layer.rs` / `grep_layer.rs`: truncation now uses `query.get_usize("maxResults").unwrap_or(MAX_MATCHES)` instead of the hard-coded constant.
- `xyrag.rs`: new `--max-results` CLI arg, injected into the query as `maxResults` when set.
