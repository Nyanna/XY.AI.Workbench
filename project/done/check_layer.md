Annotiere auf Basis von `/home/user/xyan/xy.ai.workbench/docs/layered_anytime.md` und den folgenden Kriterien die Layer ABC und Komponenten in `/home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/layer.py` vollständig und in englischer Sprache. Ziel ist das ein Agent allein auf Basis dieser Datei alle notwendigen Informationen erhellt um vollständig und konform und autonom einen Layer ohne weitere Inspektion und Dokumente implementieren zu können. Dazu gehören Designgrundlagen, Schnittstellen und Paradigmen eines Layers. Nicht notwendig sind Implementierungdetails zu aufrufverhalden der RAG Engine selbst oder Storageimplementierung.

## Kriterien

- Hauptpackage ist "xy.ai.rag"
- Die RAG Engine wird sowohl als Lib eingebunden als sie auch als CLI Utility on demand gestartet wird (kein deamon oder Serverprozess)
- Layer sollen steuerbar sequentiell laufen als auch parallel(performance; Ein Fork/Join Modell) laufen können.
- Dementsprechend wird zwar ein gemeinsames Result Set Objekt notwendig aber die Composition ist Multi Threaded und Variabel. Der erste Layer legt ein Result an, auch wenn es ein Enrichment Layer ist 
	- Das bedeutet die Topologie unterstützt sowohl eine Sequenz in der ein Layer auf ein Result für ein enrichment Observed als auch Layer die vollständig autonom sind als auch Layer die auf ein vollständiges Result Objekt warten für ein Postprocessing.
- Signale laufen über Felder eines Result Entries. Ein Result ist Dabei frei und nicht stark typisiert. Es kann einen Dateinamen beinhalten oder nur eine AST Knoten ID, Zeilennummern oder nur eine Zusammenfassung.
	- Aggregation passiert impliziert, indem Layer auf gemeinsamen Feldern operieren
	- Beispielweise findet eine Grep-artige Textsuche Zeilen in Dateinamen, ein semantischer Layer könnte Zeile aber auf auf einen Textausschnitt ersetzen oder verkleinern während ein Code AST Layer eine Outline und FQND hinzufügt
- Konkrete Layer werden später implementiert
- Weitere Metadaten wie Dateigröße oder Datum werden von weiteren Layern hinzugefügt
- Eine Query besteht aus einem aus einem schwach typisierten Dynamischen Objekt. Es könnte include oder exclude Parameter enthalten oder auch nicht. Eine Suche nach einer Funktion oder einem Text. Die Layer bestimmen wie sie auf welche Felder reagieren.
	- Ein Layer kann durch das Vorhandensein eines Feldes in der Query aktiviert werden oder aber ein Layer inspiziert und analysiert die Query um sich zu aktivieren.
- Ein numerisches Ranking wird nicht durchgeführt aber es gibt eine stabile Sortierung
	- Layer agieren hier für Postprocessing
- 2-Stage Approach wird als Layer umgesetzt. Ein Cache-Layer wartet auf ein Result Set das von bestimmten Layern abgeschlossen wurde, cached das ganze Objekt, reduziert dann den Informationsgehalt und fügt ID's ein.
	- Ein Cache Retriever Layer, reagiert auf das Vorhandensein von ID Parametern und lädt gecachte Einträge per ID in das Result Objekt. Danach können dennoch Layer folgen oder nicht.
- Die ganze RAG Engine ähnelt also einem sich dynamisch aufbauenden und anpassenden DAG
- Wann Layer Hintergrundprozesse aktivieren oder nicht entscheiden sie selbst auf Basis der Query
	- Ein CLI Prozess beendet sich, wenn alle Queries returned wurden und wenn alle Hintergrundaktivitäten aller Layer beendet sind.
	- Ein Layer hat also zwei mögliche Kontrollflüsse 1. einen gekoppelt an einen Channel von Query Objekten, 2. einen globalen Kontrollfluss für Background activities.
	- Layer entscheiden selbst über ihr Verhalten oder Priorisierung beim Eintreffen neuer Queries
- Ein zentraler auf Disk persistierter Index ermöglicht geteilte Change Detection
	- Der Cache ist dabei unspezifisch. Ein Layer kann Einträge für Chunks, Zeilen oder Dateinamen oder Dateipfade anlegen, auch Composite Objekte sind möglich die Pfade, Dateien, Chunks, AST Knoten oder Zeilen korrelieren. Die Unterstützung hierfür ist Sache der Layer
	- Layer können also Resourcen teilen, müssen jedoch nicht
- Ein Layer wird durch eine eindeutige ID identifiziert. Eine Registry verwaltet alle Layer Implementierungen
- Die gemeinsame Persistenz wird unter einem Subfolder ".xyrag" gehalten und liegt im CWD oder wird mittels Argument übergeben
	- Layer erhalten entsprechend ein abstrahiertes Interface für die Verwaltung ihrer Persistenzeinheiten
SystemInit:  id=73332b78-1383-4854-a25b-c4e1b8dded99, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: d5ef-1d
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/docs/layered_anytime.md
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/layer.py
  reason: Grundlage für Annotation der layer.py lesen
```
/answer d5ef-1d allow
ReasoningToken: 11
Control Request:
```yaml
id: d5ef-1e
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/query.py
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/result.py
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/persistence.py
  reason: "Kontext für korrekte Annotation der layer.py Schnittstellen (Query, ResultSet, Persistenz)"
```
/answer d5ef-1e allow
ReasoningToken: 66
Control Request:
```yaml
id: d5ef-1f
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/layer.py
    overwrite: true
    source: |
      """Layer contract of the xy.ai.rag engine.

      This module is the single, self-sufficient reference for implementing a
      concrete layer. Read this docstring together with the class/member docs
      below before writing a new layer; no other document should be necessary.

      Big picture
      -----------
      The RAG engine is a dynamically assembled, variable DAG of independent
      ``Layer`` implementations. The engine (package ``xy.ai.rag``) is used both
      as an embedded library and as an on-demand CLI utility - there is no daemon
      or server process. A single invocation runs the configured layers against
      one or more queries and then exits once all queries have returned and all
      layer background activity has finished (see ``background`` below).

      Topology, not a pipeline
      -------------------------
      Layers are not stages in a fixed pipeline. Depending on configuration and
      on what a layer declares via ``depends_on``/``stage``, the engine composes
      them as:

      - a sequence, where later layers enrich/observe a result produced by an
        earlier layer,
      - a fully parallel fork/join set, where independent layers run concurrently
        on separate threads/tasks against the same shared ``ResultSet``,
      - a mix, where some layers wait for individual other layers (``depends_on``)
        while others wait for the *whole* result set to be complete (stage
        ``POSTPROCESS``) before post-processing it (e.g. stable sort, cache
        reduction).

      The engine performs no numeric ranking. Ordering is a stable sort applied
      by post-processing layers over whatever fields they choose.

      The shared result object
      -------------------------
      All layers in one query run operate on one shared ``ResultSet`` (see
      ``xy.ai.rag.core.result``). It is weakly typed: a ``ResultEntry`` is a free
      bag of fields plus a list of signal names. There is no fixed schema -
      an entry may carry a file path, only an AST node id, line numbers, a text
      excerpt, or just a summary. The *first* layer that contributes in a given
      run creates the ``ResultSet``, even if that layer's own role is enrichment
      rather than generation; there is no separate "root" layer type.

      Aggregation across layers is implicit and happens purely through shared
      field names: a layer looks up existing entries (e.g. via ``ResultSet.find``
      by ``path``) and calls ``ResultEntry.merge`` to add or overwrite fields and
      append signals. Example chain: a grep-like text-search layer creates entries
      with ``path``/``line``; a semantic layer finds those entries by ``path`` and
      replaces ``line`` with a trimmed excerpt; a code-AST layer finds them again
      and adds ``outline``/``fqn`` fields. No layer needs to know about any other
      layer's existence - only about the field names it reads and writes.

      Query activation
      -----------------
      A ``Query`` (see ``xy.ai.rag.core.query``) is likewise a weakly typed,
      dynamic object (free-form fields, e.g. ``include``/``exclude``, a text or
      function-name search, cache ids, ...). A layer decides for itself whether
      it participates in a given query, either by

      - declarative activation: the default ``applies`` implementation checks
        for the presence of a field named after the layer's own ``id`` in the
        query, or
      - inspecting the query: override ``applies`` and use ``query.inspect()``
        to analyze the full field set and decide based on arbitrary logic.

      A layer must not assume any other field is present; always use
      ``Query.has``/``Query.get`` defensively.

      Two control flows
      ------------------
      Every layer has exactly two independent control flows, both defined on
      this base class:

      1. ``run`` - invoked once per query that the layer declared it ``applies``
         to, via whatever channel/scheduler the engine uses to distribute query
         objects. This is where request-scoped work happens (generating
         candidates, enriching existing entries, post-processing).
      2. ``background`` - a global control flow, independent of any single
         query, used for activities such as lazy index/cache building. A layer
         decides entirely on its own, based on query history or its own state,
         whether, when and for how long to run background work; the engine only
         signals cancellation via the ``cancel`` event (e.g. on CLI shutdown).
         The CLI process terminates only once all queries have returned *and*
         all layers' background activity has finished (``background`` returned
         or the ``cancel`` event was honored).

      Stage and dependencies as topology hints
      -----------------------------------------
      ``stage`` and ``depends_on`` describe a layer's place in the DAG, without
      the engine imposing a rigid total order:

      - GENERATE layers may run autonomously (independent of any existing
        entries) or be driven by the query, and typically create new
        ``ResultEntry`` objects.
      - ENRICH layers observe/extend entries that already exist in the shared
        ``ResultSet`` (produced by some earlier GENERATE/ENRICH layer in the same
        run), typically via ``ResultSet.find`` + ``ResultEntry.merge``.
      - POSTPROCESS layers wait until the result set for the current query is
        considered complete (per ``depends_on``, or by engine-level topology
        configuration) and then operate on the set as a whole, e.g. applying a
        stable ``ResultSet.sort``, or reducing/replacing entries.

      ``depends_on`` names the ids of other layers whose contribution to the
      *current query run* must be finished before this layer's ``run`` is
      invoked. Layers without dependencies may run fully in parallel with each
      other (fork), with the engine joining before dependents run.

      The two-stage cache pattern as layers
      ---------------------------------------
      The two-stage (cache/reduce, then fetch-by-id) approach from the design is
      not special-cased by the engine; it is implemented entirely as ordinary
      layers using the primitives above:

      - A cache-writer layer declares a POSTPROCESS stage (or depends on the
        layers whose output it wants to cache), waits for the relevant portion of
        the ``ResultSet`` to be complete, persists the full entries via its
        ``LayerStorage`` (keyed by whatever id scheme it chooses), then reduces
        the information in those entries (e.g. drops large excerpts) and merges
        in a cache/reference id field instead.
      - A cache-retriever layer activates on the presence of id-style fields in
        the incoming ``Query`` (via ``applies``), looks them up in its own
        ``LayerStorage``, and (re)creates/merges full entries in the
        ``ResultSet``. Further layers may or may not follow afterwards - the
        retrieved entries are ordinary entries subject to the same implicit
        aggregation as any other.

      Persistence contract
      ---------------------
      All shared, disk-backed state lives under a ``.xyrag`` sub-folder, located
      in the current working directory or at an explicitly supplied root path
      (see ``xy.ai.rag.core.persistence.resolve_root``). Two kinds of storage are
      handed to a layer through its context objects:

      - ``SharedIndex``: one process-wide, central, persisted file index (path,
        content hash, size, mtime, monotonic sequence number, deletion
        tombstones) used for change detection that is shared *across* layers.
        A layer uses it to detect whether a given path/content has changed since
        it last looked, typically by comparing against a cursor it persisted
        itself (``LayerStorage.get_cursor``/``set_cursor``) and then reading
        ``SharedIndex.changes_since``.
      - ``LayerStorage``: a per-layer, abstracted persistence unit (a simple
        JSON-valued key/value cache plus a private directory for sidecar files,
        e.g. vector files or custom index files). The key space and the
        structure of what is stored under it (an entry for a chunk, a line
        range, a file, a path, or a composite object correlating several of
        these) is entirely up to the layer. Layers may deliberately share
        resources by agreeing on a common key scheme out of band, but nothing
        forces them to; the engine does not interpret the key space.

      A ``LayerContext`` (per-query) and a ``BackgroundContext`` (global) are the
      only engine-provided handles to this persistence layer and to the current
      ``Query``; a layer implementation should not reach for engine internals
      beyond what these contexts expose.

      Identity and registration
      --------------------------
      Every layer implementation is identified by a stable, unique ``id`` string.
      A registry (outside this module) manages the set of known layer
      implementations by this id; the id is also the storage namespace
      (``LayerStorage`` is keyed by it) and, by default, the query field name
      that triggers activation. Treat it as a long-lived, versionless identity -
      if a layer's behavior changes incompatibly, express that through its own
      persisted metadata/versioning inside ``LayerStorage``, not by changing the
      id.
      """
      from __future__ import annotations
      import abc
      import asyncio
      import enum
      from dataclasses import dataclass, field
      from typing import Any
      from xy.ai.rag.core.persistence import LayerStorage, SharedIndex
      from xy.ai.rag.core.query import Query
      from xy.ai.rag.core.result import ResultSet

      class LayerStage(str, enum.Enum):
          """Coarse topology hint for a layer; does not replace explicit ``depends_on``.

          This classifies a layer's typical relationship to the shared
          ``ResultSet`` within one query run; the engine may use it to decide
          default scheduling (e.g. run all GENERATE/ENRICH layers in a fork/join
          group, then join POSTPROCESS layers once that group is done), but
          explicit ``depends_on`` entries always take precedence where present.

          GENERATE: creates new ``ResultEntry`` objects, either autonomously
              (independent of other layers/entries) or driven by matching on the
              ``Query``. The very first layer to contribute in a run implicitly
              creates the ``ResultSet`` itself, regardless of stage.
          ENRICH: observes and extends entries already present in the shared
              ``ResultSet`` (typically via ``ResultSet.find`` + ``merge``),
              without necessarily creating new entries of its own.
          POSTPROCESS: waits for a (sub-)set of the ``ResultSet`` to be complete
              and then operates on it as a whole, e.g. stable sorting or cache
              reduction. Does not imply a numeric ranking - the engine never
              ranks numerically, only sorts stably.
          """
          GENERATE = 'generate'
          ENRICH = 'enrich'
          POSTPROCESS = 'postprocess'

      @dataclass(frozen=True)
      class LayerStatus:
          """Outcome protocol of one layer invocation for one query.

          Returned by ``Layer.run`` so the engine/caller can distinguish "ran and
          produced nothing" from "did not run" (``skipped``, e.g. because
          ``applies`` was false) from "ran but was cancelled/timed out"
          (``aborted``). There is no numeric score here; ``contributions`` is a
          plain count of entries created or merged into, useful for logging and
          for downstream layers that want to know whether upstream work happened
          at all.
          """
          layer_id: str
          stage: LayerStage
          ran: bool
          skipped: bool = False
          aborted: bool = False
          contributions: int = 0
          '# Free-form, layer-specific diagnostic detail (e.g. coverage, timings); not interpreted by the engine.'
          detail: dict[str, Any] = field(default_factory=dict)

      @dataclass
      class LayerContext:
          """Per-query context the engine hands to a layer's ``run`` call.

          ``query``: the current ``Query`` object (also passed separately to
              ``run`` for convenience).
          ``storage``: this layer's own ``LayerStorage`` - the only place a layer
              should persist state; its key space and content structure are
              entirely private to the layer.
          ``shared_index``: the process-wide ``SharedIndex`` for cross-layer
              change detection (path/hash/size/mtime/sequence number).
          """
          query: Query
          storage: LayerStorage
          shared_index: SharedIndex

      @dataclass
      class BackgroundContext:
          """Global context the engine hands to a layer's ``background`` call.

          Same storage/index handles as ``LayerContext``, but deliberately without
          a ``Query``: background activity is decoupled from any single request
          and must decide its own scope and priority (e.g. from what previous
          queries touched, or from ``shared_index`` changes) without being told
          what to do.
          """
          storage: LayerStorage
          shared_index: SharedIndex

      class Layer(abc.ABC):
          """Base class / protocol every concrete RAG layer implements.

          A layer is a fully independent signal producer. It owns its own
          chunking, model choice, index/cache format and query-matching logic;
          the only things it shares with other layers are: the field names on
          ``ResultEntry`` it reads/writes (implicit aggregation), the persistence
          primitives exposed through the context objects, and this contract.

          Two independent control flows:
            1. ``run``        - coupled to the query channel; invoked once per
                                 query for which ``applies`` returned true.
            2. ``background``  - global control flow, fully self-directed by the
                                 layer (e.g. lazy index/cache building), not tied
                                 to any individual query. The hosting CLI process
                                 only exits once all queries have returned *and*
                                 every layer's ``background`` activity has ended.

          Implementing a new layer means: pick a stable, unique ``id``; pick a
          ``stage``/``depends_on`` reflecting where it sits relative to other
          layers that may or may not be present at runtime; override ``applies``
          only if activation needs more than "query has a field named like my
          id"; implement ``run`` to read/write ``ResultEntry`` fields and return a
          ``LayerStatus``; optionally implement ``background`` for lazy building.
          Nothing else is required, and nothing else should be assumed about the
          runtime (no central scheduler API, no fixed pipeline order) beyond what
          is documented here.
          """
          '# Stable, globally unique layer identity. Used for: registry lookup, LayerStorage namespace, and the default Query activation field name. Never reuse an id for a semantically different layer; version internally via LayerStorage metadata instead.'
          id: str
          '# Topology hint; see LayerStage. Affects default scheduling relative to other layers, not correctness - correctness must not depend on stage ordering alone when explicit ordering matters (use depends_on for that).'
          stage: LayerStage = LayerStage.ENRICH
          '# IDs of other layers whose contribution to the *current query run* must be complete before this layer'"'"'s ``run`` is invoked. Layers without (mutual) dependencies may be scheduled fully in parallel (fork); the engine joins dependents after their dependencies finish. Leave empty for layers that can run at any point relative to others.'
          depends_on: frozenset[str] = frozenset()

          def applies(self, query: Query) -> bool:
              """Decide whether this layer participates in the given query.

              Default: activate when the query carries a field named exactly
              like ``self.id`` (``query.has(self.id)``) - the declarative
              activation path. Override to inspect the full query
              (``query.inspect()``) and decide based on arbitrary combinations of
              fields instead (the analytical activation path), e.g. a layer that
              only reacts when both a ``text`` and a ``language`` field are
              present, or that activates on any cache-id-shaped field.

              Called by the engine before ``run``; must be cheap and side-effect
              free, and must not assume any field other than those it explicitly
              checks is present.
              """
              return query.has(self.id)

          @abc.abstractmethod
          async def run(self, query: Query, result_set: ResultSet, ctx: LayerContext) -> LayerStatus:
              """Process one query against the shared ``ResultSet``.

              Invoked once per query for which ``applies`` returned true. Must
              implement exactly one of the three roles implied by ``stage``:

              - GENERATE: create new ``ResultEntry`` objects (``ResultSet.add``)
                from the query and/or from this layer's own data source, either
                autonomously or triggered by matching query fields. If this is
                the first layer to contribute in the run, it is responsible for
                the ``ResultSet`` coming into existence (the engine still owns
                and passes the instance; the layer simply populates it).
              - ENRICH: look up existing entries via shared fields
                (``ResultSet.find``/``ResultSet.get``) and extend them via
                ``ResultEntry.merge`` (adding/overwriting fields, appending
                signal names). Must tolerate zero matching entries (e.g. because
                an expected upstream layer did not run or is absent from this
                topology) and should then report ``skipped``/``contributions=0``
                rather than fail.
              - POSTPROCESS: operate on the (assumed complete, per
                ``depends_on``) result set as a whole, e.g. ``ResultSet.sort``
                with a caller-chosen key for stable ordering, or reduce/replace
                entries (e.g. the cache-reduction half of the two-stage pattern).

              Must be cooperative with cancellation/timeboxing performed by the
              caller (accept being cancelled between awaited points) and must
              reflect what actually happened in the returned ``LayerStatus``
              (``ran``/``skipped``/``aborted``/``contributions``), since that is
              the only outcome signal available to the caller - there is no
              numeric score to fall back on.

              Must not assume any particular set or order of other layers is
              present beyond what it declared via ``depends_on``; all other
              coordination happens implicitly through shared ``ResultEntry``
              field names.
              """
              raise NotImplementedError

          async def background(self, ctx: BackgroundContext, cancel: asyncio.Event) -> None:
              """Optional, self-directed background activity (e.g. lazy index/cache build).

              Default: no background activity. A layer that overrides this is
              entirely responsible for deciding, on its own, whether and when to
              run (e.g. based on what recent queries touched, or on changes seen
              through ``ctx.shared_index``), how to prioritize its work, and how
              to make it resumable/abortable. There is no query object here -
              this flow is global and decoupled from any single request.

              Must check ``cancel`` frequently (between small units of work) and
              return promptly once it is set: the hosting CLI process will not
              terminate until every layer's ``background`` call has returned, so
              unresponsive cancellation handling stalls process shutdown. Persist
              progress incrementally via ``ctx.storage`` so an aborted run loses
              at most the current unit of work and leaves previously written data
              usable.
              """
              return None
  reason: "layer.py vollständig mit englischen, selbstgenügenden Annotationen gemäß Designdokument neu schreiben"
```
/answer d5ef-1f allow
Control Request:
```yaml
id: d5ef-20
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/xy/ai/rag/core/layer.py
  reason: Syntax nach Neuschreiben prüfen
```
/answer d5ef-20 allow