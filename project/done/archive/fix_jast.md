Es gibt ein Problem mit der `/home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java`. Diese sollte große semantische Blöcke als Knoten produzieren. Listet aber einzelne Imports und Anweisungen auf. Die Java Engine soll sich dabei ähnlich `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py` verhalten.

Beispiel:
```yaml
id: 243b-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptInputHandler.java
      nodes:
      - id: java_util_ArrayList
        signature: import java.util.ArrayList;
      - id: java_util_Arrays
        signature: import java.util.Arrays;
      - id: java_util_List
        signature: import java.util.List;
      - id: xy_ai_workbench_editor_md_AbstractRule
        signature: import xy.ai.workbench.editor.md.AbstractRule;
      - id: PromptInputHandler
        signature: "/**  * Input handling extracted from {"
        docstring: "Input handling extracted from {@link PromptHandler}: aggregates editor/config"
        children:
        - id: PromptInputHandler.YAML_BLOCK
          signature: private static final Pattern YAML_BLOCK = Pattern.compile("^```yaml\\R(.*?)^```…
        - id: PromptInputHandler.cfg
          signature: private final ConfigManager cfg;
        - id: PromptInputHandler.detectSelection
          signature: "private void detectSelection(Selection sel, PromptArguments arg)"
          docstring: "Selection mode: a real (multi-char) selection is a BlockSelection, an"
        - id: PromptInputHandler.detectFullFile
          signature: "private void detectFullFile(Selection content, PromptArguments arg)"
        - id: PromptInputHandler.detectedCmd
          signature: "private boolean detectedCmd(String line, String[] lines, int lineIndex, PromptA…"
        - id: PromptInputHandler.captureYamlBlock
          signature: "private String captureYamlBlock(String[] lines, int lineIndex)"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
      nodes:
      - id: java_io_BufferedReader
        signature: import java.io.BufferedReader;
      - id: java_io_IOException
        signature: import java.io.IOException;
      - id: java_io_InputStreamReader
        signature: import java.io.InputStreamReader;
      - id: java_util_HashMap
        signature: import java.util.HashMap;
      - id: java_util_Map
        signature: import java.util.Map;
      - id: java_util_regex_Matcher
        signature: import java.util.regex.Matcher;
      - id: java_util_regex_Pattern
        signature: import java.util.regex.Pattern;
      - id: org_eclipse_core_filebuffers_FileBuffers
        signature: import org.eclipse.core.filebuffers.FileBuffers;
      - id: org_eclipse_core_filebuffers_ITextFileBuffer
        signature: import org.eclipse.core.filebuffers.ITextFileBuffer;
      - id: org_eclipse_core_filebuffers_ITextFileBufferManager
        signature: import org.eclipse.core.filebuffers.ITextFileBufferManager;
      - id: org_eclipse_core_filebuffers_LocationKind
        signature: import org.eclipse.core.filebuffers.LocationKind;
      - id: org_eclipse_core_resources_IFile
        signature: import org.eclipse.core.resources.IFile;
      - id: org_eclipse_core_resources_IMarker
        signature: import org.eclipse.core.resources.IMarker;
      - id: org_eclipse_core_resources_IResource
        signature: import org.eclipse.core.resources.IResource;
      - id: org_eclipse_core_resources_IResourceChangeEvent
        signature: import org.eclipse.core.resources.IResourceChangeEvent;
      - id: org_eclipse_core_resources_IResourceChangeListener
        signature: import org.eclipse.core.resources.IResourceChangeListener;
      - id: org_eclipse_core_resources_IResourceDelta
        signature: import org.eclipse.core.resources.IResourceDelta;
      - id: org_eclipse_core_resources_IResourceDeltaVisitor
        signature: import org.eclipse.core.resources.IResourceDeltaVisitor;
      - id: org_eclipse_core_resources_IWorkspaceRoot
        signature: import org.eclipse.core.resources.IWorkspaceRoot;
      - id: org_eclipse_core_resources_ResourcesPlugin
        signature: import org.eclipse.core.resources.ResourcesPlugin;
      - id: org_eclipse_ui_IWorkbenchPartReference
        signature: import org.eclipse.ui.IWorkbenchPartReference;
      - id: org_eclipse_ui_IWorkbenchWindow
        signature: import org.eclipse.ui.IWorkbenchWindow;
      - id: org_eclipse_ui_PlatformUI
        signature: import org.eclipse.ui.PlatformUI;
      - id: org_eclipse_ui_texteditor_ITextEditor
        signature: import org.eclipse.ui.texteditor.ITextEditor;
      - id: org_osgi_framework_BundleContext
        signature: import org.osgi.framework.BundleContext;
      - id: xy_ai_workbench_ConfigManager
        signature: import xy.ai.workbench.ConfigManager;
      - id: xy_ai_workbench_LOG
        signature: import xy.ai.workbench.LOG;
      - id: xy_ai_workbench_OutputMode
        signature: import xy.ai.workbench.OutputMode;
      - id: xy_ai_workbench_editor_AISessionEditor
        signature: import xy.ai.workbench.editor.AISessionEditor;
      - id: xy_ai_workbench_models_AIAnswer
        signature: import xy.ai.workbench.models.AIAnswer;
      - id: MarkerRessourceScanner
        signature: "public class MarkerRessourceScanner implements IResourceChangeListener, IResour…"
        children:
        - id: MarkerRessourceScanner.AIREQ_PREFIX
          signature: private static final String AIREQ_PREFIX = "xy.ai.req";
        - id: MarkerRessourceScanner.MARKER_ID
          signature: private static final String MARKER_ID = "xy.ai.workbench.promptmarker";
        - id: MarkerRessourceScanner.MARKER_REQ_ID_ATTR
          signature: private static final String MARKER_REQ_ID_ATTR = "requestId";
        - id: MarkerRessourceScanner.MARKER_OFF_ID_ATTR
          signature: private static final String MARKER_OFF_ID_ATTR = "offset";
        - id: MarkerRessourceScanner.MARKER_LEN_ID_ATTR
          signature: private static final String MARKER_LEN_ID_ATTR = "length";
        - id: MarkerRessourceScanner.pattern
          signature: private final Pattern pattern;
        - id: MarkerRessourceScanner.cfg
          signature: private ConfigManager cfg;
        - id: MarkerRessourceScanner.MarkerRessourceScanner
          signature: "public MarkerRessourceScanner(ConfigManager cfg, BundleContext context)"
        - id: MarkerRessourceScanner.dispose
          signature: public void dispose(BundleContext context)
        - id: MarkerRessourceScanner.visit
          signature: public boolean visit(IResourceDelta delta) throws CoreException
        - id: MarkerRessourceScanner.rescannFile
          signature: private void rescannFile(IFile file)
        - id: MarkerRessourceScanner.resourceChanged
          signature: public void resourceChanged(IResourceChangeEvent event)
        - id: MarkerRessourceScanner.findAndReplaceMarkers
          signature: public boolean findAndReplaceMarkers(AIAnswer ans)
          docstring: '@param ans'
        - id: MarkerRessourceScanner.isAutoFollowModeEnabled
          signature: private boolean isAutoFollowModeEnabled()
        - id: MarkerRessourceScanner.shouldAutoFollow
          signature: "private boolean shouldAutoFollow(ITextEditor editor, IDocument doc)"
        - id: MarkerRessourceScanner.AutoFollowState
          signature: "/**  * Holds, per doc, the auto-follow {"
          docstring: "Holds, per doc, the auto-follow {@link IDocumentListener} together with the"
          children:
          - id: MarkerRessourceScanner.AutoFollowState.editor
            signature: final ITextEditor editor;
          - id: MarkerRessourceScanner.AutoFollowState.listener
            signature: final IDocumentListener listener;
          - id: MarkerRessourceScanner.AutoFollowState.AutoFollowState
            signature: "AutoFollowState(ITextEditor editor, IDocumentListener listener)"
        - id: MarkerRessourceScanner.autoFollowListeners
          signature: "private final Map<IDocument, AutoFollowState> autoFollowListeners = new HashMap…"
        - id: MarkerRessourceScanner.getOpenTextEditors
          signature: private java.util.List<ITextEditor> getOpenTextEditors()
        - id: MarkerRessourceScanner.getPromptTag
          signature: "public static String getPromptTag(String meta, String id)"
```
SystemInit:  id=b50cc03c-97f5-45c9-90a2-d72ed3cca730, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 243b-3
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
  reason: Analyse aktueller Java Engine und Python-Referenzimplementierung
```
/answer 243b-3 allow
ReasoningToken: 10
Control Request:
```yaml
id: 243b-4
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/java
    pattern: class.*Located|locate_all|locateAll|AddressableNode|import.*ImportDeclaration
    include:
    - '*.java'
  reason: Java Engine Struktur finden für Knotenerzeugung
```
/answer 243b-4 allow
Control Request:
```yaml
id: 243b-5
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/java
      matches:
      - path: src/xy/ai/mcpc/ast/openapi/AppendInfoNodesValidateClient.java
        matches:
        - lineno: 20
          match: "* Engine.parse + Engine.locate_all — every addressable node, in document order."
      - path: src/xy/ai/mcpc/ast/openapi/AppendInfoNodesValidateServer.java
        matches:
        - lineno: 50
          match: "* Engine.parse + Engine.locate_all — every addressable node, in document order."
      - path: src/xy/ai/mcpc/ast/engine/JavaAstServer.java
        matches:
        - lineno: 109
          match: "private AddressableNode find(CompilationUnit cu, String nodeId) {"
        - lineno: 110
          match: List<AddressableNode> located = NodeLocator.locateAll(cu);
        - lineno: 111
          match: "for (AddressableNode a : located) if (a.id.equals(nodeId))"
        - lineno: 140
          match: "private xy.ai.mcpc.ast.openapi.components.Node toDto(AddressableNode a, boolean includeCode) {"
        - lineno: 196
          match: "for (AddressableNode a : NodeLocator.locateAll(resolved.cu())) list.add(toDto(a, includeCode));"
        - lineno: 211
          match: "AddressableNode target = find(resolved.cu(), nodeId);"
        - lineno: 224
          match: "AddressableNode target = find(resolved.cu(), nodeId);"
        - lineno: 241
          match: "AddressableNode target = find(resolved.cu(), nodeId);"
        - lineno: 260
          match: "AddressableNode target = find(resolved.cu(), nodeId);"
      - path: src/xy/ai/mcpc/ast/engine/AddressableNode.java
        matches:
        - lineno: 11
          match: "public final class AddressableNode {"
        - lineno: 23
          match: "public AddressableNode(String id, Node astNode, String type, String name, int lineno, int endLineno, String parentType, boolean expandable, boolean isDefinition, NodeList<?> container) {"
      - path: src/xy/ai/mcpc/ast/engine/NodeLocator.java
        matches:
        - lineno: 4
          match: import com.github.javaparser.ast.ImportDeclaration;
        - lineno: 24
          match: "* the JavaParser equivalent of the Python/tree-sitter engines' {@code locate_all}."
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 243b-5 allow
ReasoningToken: 70
Control Request:
```yaml
id: 243b-6
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/NodeLocator.java
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/AddressableNode.java
  reason: "NodeLocator-Implementierung prüfen, um Gruppierung von Imports/Statements zu Blöcken zu ergänzen"
```
/answer 243b-6 allow
Control Request:
```yaml
id: 243b-7
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: SEGMENT_MAX_CHARS|def id_segment|_StatementGroup|_is_expandable|_DEF_TYPES|_IMPORT_TYPES
    include:
    - '*.py'
  reason: Details zu Gruppierungslogik und Grenzwerten in der Python-Referenzimplementierung finden
```
/answer 243b-7 allow
Control Request:
```yaml
id: 243b-8
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: base.py
        matches:
        - lineno: 126
          match: SEGMENT_MAX_CHARS = 500
        - lineno: 179
          match: "def id_segment(name: str | None, index: int, used: dict[str, int], *, hash_only: bool=False, content: str | None=None) -> str:"
      - path: generic/_css.py
        matches:
        - lineno: 4
          match: '``_CSS_SEGMENT_MAX_CHARS``) - the plain native tree-sitter grammar (every'
        - lineno: 17
          match: _CSS_SEGMENT_MAX_CHARS = 1500
        - lineno: 20
          match: "_DEF_TYPES = {'rule_set', 'media_statement', 'supports_statement', 'keyframes_statement', 'at_rule', 'keyframe_block'}"
        - lineno: 24
          match: "_IMPORT_TYPES = {'import_statement', 'charset_statement', 'namespace_statement'}"
        - lineno: 52
          match: return node_type in _DEF_TYPES
        - lineno: 67
          match: "if node.type in _DEF_TYPES:"
        - lineno: 72
          match: expandable = bool(body) and any((c.type in _DEF_TYPES for c in body.named_children))
        - lineno: 91
          match: kind = 'imports' if node.type in _IMPORT_TYPES else 'declarations'
        - lineno: 95
          match: "if current.type in _DEF_TYPES:"
        - lineno: 97
          match: current_kind = 'imports' if current.type in _IMPORT_TYPES else 'declarations'
        - lineno: 101
          match: "if i > start and length + piece > _CSS_SEGMENT_MAX_CHARS:"
      - path: generic/_java.py
        matches:
        - lineno: 4
          match: '``SEGMENT_MAX_CHARS``) since the plain native tree-sitter grammar (every'
        - lineno: 10
          match: "from xy.ai.mcpc.tools.ast.base import SEGMENT_MAX_CHARS, Located, Tree, id_segment"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 243b-8 allow
ReasoningToken: 1428
Control Request:
```yaml
id: 243b-9
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_java.py
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    min_line: 100
    max_line: 220
  reason: Referenz-Java-Engine (tree-sitter) und id_segment/SEGMENT_MAX_CHARS Definitionen lesen für Portierung nach JavaParser"
```
/answer 243b-9 allow
Control Request:
```yaml
id: 243b-a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_java.py
      checksum: 9dc7459bcab592a843f05d33e90b6490155a12575129b6c8c22f31f03799d657
      content: |-
        """Java tree-sitter engine: Classes/Interfaces/Enums -> Methods, mirroring the
        Python engine's shape (def-like nodes are individually addressable, everything
        else collapses into 'imports'/'statements' segments capped at
        ``SEGMENT_MAX_CHARS``) since the plain native tree-sitter grammar (every
        expression/identifier addressable) is far too fine-grained to be useful.
        """
        from __future__ import annotations
        import re
        from typing import Any
        from xy.ai.mcpc.tools.ast.base import SEGMENT_MAX_CHARS, Located, Tree, id_segment
        from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _SynthNode
        __all__ = ['JavaEngine']
        '#: Individually addressable "def-like" node types (types + methods/constructors).'
        _TYPE_DEF_TYPES = {
            'class_declaration',
            'interface_declaration',
            'enum_declaration',
            'record_declaration',
            'annotation_type_declaration'}
        _DEF_TYPES = _TYPE_DEF_TYPES | {'method_declaration', 'constructor_declaration'}
        _IMPORT_TYPES = {'package_declaration', 'import_declaration'}
        '#: A single-line annotation (e.g. \'@Override\', \'@SuppressWarnings("x")\'), to skip when hunting for a definition\'s actual header line.'
        _ANNOTATION_LINE = re.compile('^@[A-Za-z_][\\w.]*(\\([^)]*\\))?$')
        "#: A type declaration's own body-container child, whose children are its members."
        _BODY_TYPES = {'class_body', 'interface_body', 'annotation_type_body'}

        def _body_of(def_node: Any) -> Any | None:
            for child in def_node.named_children:
                if child.type in _BODY_TYPES:
                    return child
            return None

        class JavaEngine(TreeSitterEngine):
            """Tree-sitter Java restructured like the Python engine: types/methods as
            real nodes, everything else grouped into statement/import segments."""

            def __init__(self) -> None:
                super().__init__('java')

            def is_definition(self, node_type: str) -> bool:
                return node_type in _DEF_TYPES

            def signature(self, node: Any, limit: int=80) -> str:
                """Like the base heading, but skips leading annotation-only lines
            (e.g. a bare "@Override") to find the actual declaration header; if
            none turns up, falls back to the first line and the char limit."""
                text = node.text.decode('utf-8', 'replace') if node.text else ''
                lines = [line.strip() for line in text.splitlines() if line.strip()]
                first_line = next((line for line in lines if not _ANNOTATION_LINE.match(line)), lines[0] if lines else '')
                return first_line if len(first_line) <= limit else first_line[:limit - 1] + '…'

            def locate_all(self, tree: Tree) -> list[Located]:
                results: list[Located] = []
                source = tree.source.encode('utf-8')

                def walk(children: list[Any], container: Any, path: str) -> None:
                    used: dict[str, int] = {}
                    i, n = (0, len(children))
                    while i < n:
                        node = children[i]
                        if node.type in _DEF_TYPES:
                            name = self._name(node)
                            seg = id_segment(name, i, used)
                            nid = f'{path}.{seg}' if path else seg
                            body = _body_of(node) if node.type in _TYPE_DEF_TYPES else None
                            expandable = bool(body) and any((c.type in _DEF_TYPES for c in body.named_children))
                            results.append(
                                Located(
                                    tree=tree,
                                    node=node,
                                    parent=container,
                                    index=i,
                                    node_id=nid,
                                    node_type=node.type,
                                    name=name,
                                    lineno=node.start_point[0] + 1,
                                    end_lineno=node.end_point[0] + 1,
                                    parent_type=container.type,
                                    expandable=expandable))
                            if body is not None:
                                walk(body.named_children, body, nid)
                            i += 1
                            continue
                        start = i
                        kind = 'imports' if node.type in _IMPORT_TYPES else 'statements'
                        length = 0
                        while i < n:
                            current = children[i]
                            if current.type in _DEF_TYPES:
                                break
                            current_kind = 'imports' if current.type in _IMPORT_TYPES else 'statements'
                            if current_kind != kind:
                                break
                            piece = current.end_byte - current.start_byte
                            if i > start and length + piece > SEGMENT_MAX_CHARS:
                                break
                            length += piece
                            i += 1
                        group = _SynthNode(kind, children[start:i], source)
                        seg = id_segment(None, start, used, content=self.node_code(group))
                        nid = f'{path}.{seg}' if path else seg
                        results.append(
                            Located(
                                tree=tree,
                                node=group,
                                parent=container,
                                index=start,
                                node_id=nid,
                                node_type=kind,
                                name=None,
                                lineno=group.start_point[0] + 1,
                                end_lineno=group.end_point[0] + 1,
                                parent_type=container.type,
                                expandable=False))
                walk(tree.raw.root_node.named_children, tree.raw.root_node, '')
                return results
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      checksum: 2a438a751613449502ba4f4e442f485d1b731de646396a27104f9e3d444c886f
      content: |
        class OutlineNode:
            """One node in a structural (list/find/read) result.

            ``id`` is the node's unique, primarily name-based path used by every tool to
            address it. ``code`` carries the node's full source and is populated by
            ``find``/``read`` – ``list`` always leaves it ``None``. ``signature``/
            ``docstring`` are only set for class/function nodes whose ``code`` is
            *not* included, since the full source already makes them visible.
            Serialization drops ``None``/empty fields, see :func:`to_dict`.
            """
            id: str
            type: str | None
            lines: str | None
            signature: str | None
            docstring: str | None
            code: str | None = None
            children: list['OutlineNode'] = field(default_factory=list)

        def line_range(loc: Located) -> str:
            """Return ``loc``'s start line, or a ``"start-end"`` range if it spans several."""
            if loc.end_lineno == loc.lineno:
                return str(loc.lineno)
            return f'{loc.lineno}-{loc.end_lineno}'
        _ID_CLEAN_RE = re.compile('\\W+')
        '#: A statement/anonymous segment keeps accumulating siblings until adding the'
        '#: next one would push its source past this many characters (then it splits).'
        SEGMENT_MAX_CHARS = 500

        def _hash(name: str, length: int) -> str:
            return hashlib.sha1(name.encode('utf-8')).hexdigest()[:length]
        _ID_HASH_ALPHABET = string.digits + string.ascii_letters

        def _base62_hash(text: str, length: int) -> str:
            """Base62 (0-9a-zA-Z) digest of ``text``."""
            digest = int.from_bytes(hashlib.sha1(text.encode('utf-8')).digest(), 'big')
            base = len(_ID_HASH_ALPHABET)
            chars = []
            for _ in range(length):
                digest, rem = divmod(digest, base)
                chars.append(_ID_HASH_ALPHABET[rem])
            return ''.join(chars)

        def _content_hash(content: str, length: int=6) -> str:
            """Base62 digest of ``content``, stable across unrelated tree edits."""
            return _base62_hash(content, length)

        def _content_prefix_hash(content: str, length: int=6) -> str:
            """Base62 digest of ``content``'s whitespace-stripped first/last 20 chars.

            Forms an anonymous id segment's stable prefix: it only depends on the
            node's outer shape, so it survives edits that shift the node's interior
            (and with it ``_content_hash``) while leaving its boundaries intact.
            """
            stripped = re.sub('\\s+', '', content)
            return _base62_hash(stripped[:20] + stripped[-20:], length)
        _ID_SUFFIX_RE = re.compile('_\\d+$')

        def _id_prefix(segment: str) -> str | None:
            """An anonymous id segment's stable prefix (before ``'|'``), if any."""
            segment = _ID_SUFFIX_RE.sub('', segment)
            prefix, sep, _hash = segment.partition('|')
            return prefix if sep else None

        def resolve_by_prefix(located: list['Located'], target_id: str) -> 'Located | None':
            """Fallback selector for a ``target_id`` that matches no node exactly.

            Used when an anonymous node's id has gone stale (its content hash shifted
            due to edits elsewhere in the file) while its stable prefix hash and
            ancestor path have not. Returns the single node sharing both, or ``None``
            if ``target_id`` carries no prefix or several/no nodes match.
            """
            parent, _, last = target_id.rpartition('.')
            prefix = _id_prefix(last)
            if prefix is None:
                return None
            hits = [loc for loc in located if loc.node_id.rpartition(
                '.')[0] == parent and _id_prefix(loc.node_id.rpartition('.')[2]) == prefix]
            return hits[0] if len(hits) == 1 else None

        def id_segment(name: str | None, index: int, used: dict[str, int], *, hash_only: bool=False, content: str | None=None) -> str:
            """Return a unique-within-siblings id segment, name-based when feasible.

            A clean, short name becomes the segment verbatim; a long/awkward name collapses
            to a short hash; a nameless node falls back to a
            ``"<prefix>|<hash>"`` content hash (both 6 chars, base62) or, lacking
            ``content``, its numeric ``index``. The prefix, hashed from the node's
            whitespace-stripped first/last 20 chars, stays put even when unrelated
            edits shift the trailing hash, letting :func:`resolve_by_prefix` recover a
            stale id. With ``hash_only`` the name is *always* reduced to
            a 6-char hex hash (used for Markdown headings, whose id must never be the
            literal heading text). Collisions among siblings get a numeric suffix.
            """
            seg: str | None = None
            if name:
                if hash_only:
                    seg = _hash(name, 6)
                else:
                    cleaned = _ID_CLEAN_RE.sub('_', name).strip('_')
                    seg = cleaned if cleaned and len(cleaned) <= 40 else 'h' + _hash(name, 8)
            if not seg:
                seg = f'{_content_prefix_hash(content)}|{_content_hash(content)}' if content else str(index)
            count = used.get(seg, 0)
            used[seg] = count + 1
            return seg if count == 0 else f'{seg}_{count}'

        def node_outline(loc: Located, *, with_code: bool=False, with_lines: bool=True, with_type: bool=True, children: list[OutlineNode] | None=None) -> OutlineNode:
            """Build an :class:`OutlineNode` describing ``loc`` (source only if ``with_code``, lines only if ``with_lines``).

            ``signature``/``docstring`` are only computed when ``code`` is not, since the
            full source already makes them visible.
            """
            engine = loc.tree.engine
            if with_code:
                signature = docstring = None
                code = engine.node_code(loc.node)
            else:
                signature = engine.signature(
                    loc.node) if engine.is_definition(
                        loc.node_type) else engine.default_signature(
                            loc.node)
                docstring = engine.docstring(loc.node)
```
/answer 243b-a allow
ReasoningToken: 80
Control Request:
```yaml
id: 243b-b
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstServer.java
  reason: "Vollständige Verwendung von AddressableNode/JavaAstEngine im Server verstehen, um Gruppierungslogik konsistent zu integrieren\""
```
/answer 243b-b allow
ReasoningToken: 977
Control Request:
```yaml
id: 243b-c
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    min_line: 1
    max_line: 100
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    min_line: 210
    max_line: 320
  reason: "Engine-Basisklasse und node_outline vollständig lesen, um default_signature/docstring-Verhalten zu verstehen\""
```
/answer 243b-c allow
Control Request:
```yaml
id: 243b-d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      checksum: 1db6cd6b0023d1de78d432c5681bf6c29b760c5379a5365f3fe7b3e2ef4374ac
      content: |
        """Engine-agnostic model shared by every ``ast_*`` tool.

        The tools address nodes by *selector* (id, type, name, line
        range or parent type) and never touch a concrete parser. Two engines implement
        :class:`Engine`: a Python one built on the standard-library ``ast`` module and a
        generic tree-sitter one for every other language/format. :mod:`.core` picks the
        engine per file extension and exposes a thin facade the tools call.

        A :class:`Tree` carries a back-reference to the engine that produced it, so
        every helper here can dispatch to the right engine without the tools knowing
        which one is in play.
        """
        from __future__ import annotations
        import difflib
        import hashlib
        import re
        import string
        from abc import ABC, abstractmethod
        from dataclasses import asdict, dataclass, field
        from pathlib import Path
        from typing import Any

        class AstError(Exception):
            """A user-facing, path-free error raised by the AST tools."""

        class AstAmbiguous(AstError):
            """Raised when a text-based node search (no id given) finds several
            unrelated candidates instead of a single node."""

            def __init__(self, message: str, candidates: list[str]) -> None:
                super().__init__(message)
                self.candidates = candidates

        class AstTextError(AstError):
            """Raised when a text/marker-based edit's search text could not be applied.

            Carries the shared-matcher's diagnosis (see ``tools._text_match``): ``reason``
            classifies the cause (e.g. ``whitespace_mismatch``, ``content_changed``,
            ``guard_rejected``, ``marker_order``, ``ambiguous``, ``not_found``);
            ``corrected_text`` is a verified fix (whitespace-only difference), ``guess``
            an unverified single-candidate heuristic, ``next_step`` a fallback instruction.
            """

            def __init__(self, message: str, *, reason: str | None=None, position: str | None=None, corrected_text: str | None=None, guess: str | None=None, next_step: str | None=None) -> None:
                super().__init__(message)
                self.reason = reason
                self.position = position
                self.corrected_text = corrected_text
                self.guess = guess
                self.next_step = next_step

        @dataclass
        class Tree:
            """A parsed file/snippet plus the engine that owns it.

            Attributes:
                engine: The engine that parsed ``raw`` and knows how to mutate it.
                raw: Engine-native tree (``ast.Module`` or ``tree_sitter.Tree``).
                source: Current source text; the single source of truth for tree-sitter
                    engines, refreshed by the Python engine only on save.
                path: Absolute path the tree was loaded from, or ``None`` for snippets.
            """
            engine: 'Engine'
            raw: Any
            source: str
            path: Path | None = None

        @dataclass
        class Located:
            """A node with the engine-independent metadata the selectors match on.

            Attributes:
                tree: The owning tree (for engine dispatch).
                node: Engine-native node object.
                parent: Engine-native container node.
                index: Position of ``node`` among its parent's addressable children.
                node_id: The node's unique ``id`` — its fully-qualified path from the
                    root (e.g. ``"MyClass.method"``), in name/hash form or, for nameless
                    nodes/segments, a stable content-hash fallback. There is no separate FQN.
                node_type: Engine-reported node type name.
                name: Simple name, if the node carries one.
                lineno / end_lineno: 1-based inclusive line span.
                parent_type: Type name of ``parent``, or ``None`` at the top level.
                expandable: Whether ``read`` should descend into children instead of
                    returning the node's full source (a pure container of nested defs).
            """
            tree: Tree
            node: Any
            parent: Any
            index: int
            node_id: str
            node_type: str
            name: str | None
            lineno: int
            end_lineno: int
            parent_type: str | None
            expandable: bool = False

        @dataclass(frozen=True)
        class OutlineNode:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      checksum: c1277cd3d8e430fc4658efd3684b2288367fca52187f68b5c64cd6b4a7fde1c5
      content: |2
            """
            engine = loc.tree.engine
            if with_code:
                signature = docstring = None
                code = engine.node_code(loc.node)
            else:
                signature = engine.signature(
                    loc.node) if engine.is_definition(
                        loc.node_type) else engine.default_signature(
                            loc.node)
                docstring = engine.docstring(loc.node)
                code = None
            return OutlineNode(
                id=loc.node_id,
                type=loc.node_type if with_type else None,
                lines=line_range(loc) if with_lines else None,
                signature=signature,
                docstring=docstring,
                code=code,
                children=children or [])

        def _compact(value: Any) -> Any:
            """Recursively drop ``None`` values and empty lists from a dataclass-derived structure."""
            if isinstance(value, dict):
                return {k: _compact(v) for k, v in value.items() if v is not None and v != []}
            if isinstance(value, list):
                return [_compact(v) for v in value]
            return value

        def to_dict(node: OutlineNode) -> dict:
            """Serialize an :class:`OutlineNode` to MCP output, omitting empty fields."""
            return _compact(asdict(node))

        @dataclass
        class _TreeNode:
            loc: Located
            children: list['_TreeNode'] = field(default_factory=list)

        def _build_forest(located: list[Located]) -> list[_TreeNode]:
            """Nest a pre-order list of ``Located`` into a forest via ``node_id`` prefixes."""
            roots: list[_TreeNode] = []
            stack: list[_TreeNode] = []
            for loc in located:
                node = _TreeNode(loc)
                while stack and (not loc.node_id.startswith(stack[-1].loc.node_id + '.')):
                    stack.pop()
                (stack[-1].children if stack else roots).append(node)
                stack.append(node)
            return roots

        def build_outline(located: list[Located], *, with_code: bool=False, with_lines: bool=True, with_type: bool=True) -> list[OutlineNode]:
            """Build the nested outline of ``located`` (source only if ``with_code``, lines only if ``with_lines``).

            Non-expandable nodes (no nested defs worth descending into) are rendered with
            their full source instead of being fragmented into ``children``.
            """
            return _outline_nodes(_build_forest(located), with_code=with_code, with_lines=with_lines, with_type=with_type)

        def _outline_nodes(nodes: list['_TreeNode'], *, with_code: bool, with_lines: bool=True, with_type: bool=True) -> list[OutlineNode]:
            """Convert a forest into OutlineNodes, collapsing non-expandable nodes to full source instead of ``children``."""
            result: list[OutlineNode] = []
            for t in nodes:
                if t.loc.expandable and t.children:
                    result.append(
                        node_outline(
                            t.loc,
                            with_code=False,
                            with_lines=with_lines,
                            with_type=with_type,
                            children=_outline_nodes(
                                t.children,
                                with_code=with_code,
                                with_lines=with_lines,
                                with_type=with_type)))
                else:
                    result.append(node_outline(t.loc, with_code=with_code, with_lines=with_lines, with_type=with_type))
            return result

        def _resolve_by_name(key: str, by_name: dict[str, list['_TreeNode']]) -> tuple['_TreeNode | None', str | None]:
            """Resolve ``key`` against node names when it doesn't match an id directly.

            Tries an exact name match first (agents commonly pass a function/class name
            instead of its full id), then a single sufficiently close fuzzy match. The
            fuzzy cutoff scales with ``key``'s length so short names still require a
            near-exact match. Returns ``(None, reason)`` with a human-readable reason
            when a match exists but is ambiguous, or ``(None, None)`` when nothing is
            close enough.
            """
            exact = by_name.get(key)
            if exact:
                if len(exact) == 1:
                    return (exact[0], None)
                return (None, f"'{key}' matches {len(exact)} nodes by name; use a specific id.")
            if not by_name:
                return (None, None)
            cutoff = 0.5 + min(0.35, 1.4 / max(len(key), 1))
            scored = sorted(((difflib.SequenceMatcher(None, key, name).ratio(), name) for name in by_name), reverse=True)
            best_score, best_name = scored[0]
            if best_score < cutoff:
                return (None, None)
            if len(scored) > 1 and scored[1][0] == best_score:
                return (None, f"'{key}' is ambiguous between similarly named nodes; use a specific id.")
            candidates = by_name[best_name]
            if len(candidates) != 1:
                return (None, f"'{key}' matches {len(candidates)} nodes named '{best_name}'; use a specific id.")
            return (candidates[0], None)

        def read_subtrees(located: list[Located], keys: list[str], *, with_lines: bool=True) -> tuple[list[OutlineNode], list[str]]:
            """Return one read subtree per resolvable ``keys`` entry.

            Each key is matched, in order, by exact id, by exact node name, by a
```
/answer 243b-d allow
ReasoningToken: 8909
Control Request:
```yaml
id: 243b-e
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: java|JavaAstServer|http|port|subprocess
    include:
    - '*.py'
    limit: 30
  reason: "Finden, wie die Java-Engine (HTTP-Server) in den Python-basierten ast_outline Tool eingebunden wird\""
```
/answer 243b-e deny warum sollte das relevant sein?
ReasoningToken: 323
Control Request:
```yaml
id: 243b-f
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: JavaAstServer|class JavaEngine|ExternalEngine|http
    include:
    - '*.py'
  reason: "Prüfen, ob JavaAstServer über HTTP von der Python-Tool-Schicht angesprochen wird, um zu verstehen, welches Ausgabeformat maßgeblich ist\""
```
/answer 243b-f deny warum sollte das relevant sein?
ReasoningToken: 18
Control Request:
```yaml
id: 243b-10
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: JavaAstServer|java-javaparser|ExternalEngine
    include:
    - '*.py'
  reason: "User bat um Behebung der Blockgruppierung in JavaAstEngine.java; muss ermitteln, welche Komponente die HTTP-Antworten des Java-Engines (JavaAstServer) konsumiert und in das gezeigte YAML-Ausgabeformat (id/signature/children) übersetzt, um die Korrektur an der richtigen Stelle korrekt konsistent umzusetzen."
```
/answer 243b-10 allow du musst die Korrektur in der JavaEngine vornehmen. Das YAML Format ist dafür gänzlich irrelevant, genauso wie der Transport. Es muss sinnvolle AST-Knoten geben.
Control Request:
```yaml
id: 243b-11
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches: []
```
/answer 243b-11 allow
ReasoningToken: 69
Control Request:
```yaml
id: 243b-12
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/NodeGroup.java
    mode: replace
    content: |
      package xy.ai.mcpc.ast.engine;

      import com.github.javaparser.ast.Node;
      import com.github.javaparser.ast.NodeList;
      import java.util.List;

      /**
       * A contiguous run of sibling declarations (imports, or non-definition class
       * members such as fields/initializers) collapsed into one addressable
       * "ImportGroup"/"StatementGroup" segment, mirroring the Python/tree-sitter
       * engines: JavaParser's per-declaration granularity (every import, every
       * field, on its own) is far too fine to be individually useful.
       */
      public final class NodeGroup {
        public static final String KIND_IMPORTS = "ImportGroup";
        public static final String KIND_STATEMENTS = "StatementGroup";

        public final String kind;
        public final NodeList<?> container;
        public final int start;
        public final int end;

        public NodeGroup(String kind, NodeList<?> container, int start, int end) {
          this.kind = kind;
          this.container = container;
          this.start = start;
          this.end = end;
        }

        @SuppressWarnings("unchecked")
        public List<Node> members() {
          return (List<Node>) container.subList(start, end);
        }
      }
  reason: Neue NodeGroup-Klasse zur Repräsentation gruppierter Import-/Statement-Bereiche anlegen
```
/answer 243b-12 allow
Control Request:
```yaml
id: 243b-13
toolName: edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/AddressableNode.java
    old_text: |-
      public final class AddressableNode {
        public final String id;
        public final Node astNode;
    new_text: |-
      public final class AddressableNode {
        public final String id;
        public final Object astNode;
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/AddressableNode.java
    old_text: "public AddressableNode(String id, Node astNode, String type,"
    new_text: "public AddressableNode(String id, Object astNode, String type,"
  reason: AddressableNode auf gruppierte Knoten (NodeGroup neben Node) erweitern
```
/answer 243b-13 allow
ReasoningToken: 14440
Text:
 Nun `NodeLocator.java` vollständig neu schreiben mit Gruppierungslogik:
Control Request:
```yaml
id: 243b-14
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/NodeLocator.java
    mode: replace
    content: |
      package xy.ai.mcpc.ast.engine;

      import com.github.javaparser.ast.CompilationUnit;
      import com.github.javaparser.ast.ImportDeclaration;
      import com.github.javaparser.ast.Node;
      import com.github.javaparser.ast.NodeList;
      import com.github.javaparser.ast.body.BodyDeclaration;
      import com.github.javaparser.ast.body.ConstructorDeclaration;
      import com.github.javaparser.ast.body.EnumConstantDeclaration;
      import com.github.javaparser.ast.body.EnumDeclaration;
      import com.github.javaparser.ast.body.MethodDeclaration;
      import com.github.javaparser.ast.body.TypeDeclaration;
      import java.math.BigInteger;
      import java.nio.charset.StandardCharsets;
      import java.security.MessageDigest;
      import java.security.NoSuchAlgorithmException;
      import java.util.ArrayList;
      import java.util.HashMap;
      import java.util.List;
      import java.util.Map;
      import java.util.regex.Pattern;

      /**
       * Flattens a {@link CompilationUnit} into every addressable node, in document order —
       * the JavaParser equivalent of the Python/tree-sitter engines' {@code locate_all}.
       * Only types (classes/interfaces/enums/records/annotations), their nested types,
       * methods and constructors are individually addressable "definitions"; method/
       * constructor/initializer bodies are treated as leaves (not expanded further).
       * Everything else — imports at the top level, fields/initializers/annotation
       * members inside a type — is far too fine-grained to be individually useful, so
       * consecutive runs of it are collapsed into anonymous {@link NodeGroup} segments
       * capped at {@link #SEGMENT_MAX_CHARS}, exactly like the Python/tree-sitter engines'
       * "imports"/"statements" segments.
       *
       * Ids are name-based structural paths (e.g. {@code "MyClass.foo_2"}) for named
       * definitions, and stable content-hash paths (e.g. {@code "a1B2c3|d4E5f6"}) for
       * anonymous group segments, rebuilt fresh on every call from the live tree; stable
       * as long as the declaration order/names/content haven't changed since the id was
       * handed out. A node no longer found by id is reported as a 404 by the caller, not
       * silently misresolved.
       */
      public final class NodeLocator {
        /** A group keeps accumulating siblings until adding the next one would push its source past this many characters (then it splits). */
        private static final int SEGMENT_MAX_CHARS = 500;
        private static final Pattern UNSAFE = Pattern.compile("[^A-Za-z0-9_]+");
        private static final String HASH_ALPHABET = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

        private NodeLocator() {
        }

        public static List<AddressableNode> locateAll(CompilationUnit cu) {
          List<AddressableNode> out = new ArrayList<>();
          groupImports(cu, out);
          Map<String, Integer> usedTypes = new HashMap<>();
          NodeList<TypeDeclaration<?>> types = cu.getTypes();
          for (int i = 0; i < types.size(); i++) walkType(types.get(i), types, "CompilationUnit", "", usedTypes, out);
          return out;
        }

        private static void groupImports(CompilationUnit cu, List<AddressableNode> out) {
          NodeList<ImportDeclaration> imports = cu.getImports();
          Map<String, Integer> used = new HashMap<>();
          int n = imports.size(), i = 0;
          while (i < n) {
            int start = i, length = 0;
            while (i < n) {
              int piece = imports.get(i).toString().length();
              if (i > start && length + piece > SEGMENT_MAX_CHARS)
                break;
              length += piece;
              i++;
            }
            addGroup(imports, NodeGroup.KIND_IMPORTS, "CompilationUnit", "", start, i, used, out);
          }
        }

        private static void walkType(TypeDeclaration<?> t, NodeList<?> container, String parentType, String parentId, Map<String, Integer> used, List<AddressableNode> out) {
          String simple = simpleName(t);
          String seg = segment(t.getNameAsString(), simple, used);
          String id = parentId.isEmpty() ? seg : parentId + "." + seg;
          boolean expandable = hasDefinitionMembers(t);
          out.add(new AddressableNode(id, t, simple, t.getNameAsString(), line(t), endLine(t), parentType, expandable, true, container));
          Map<String, Integer> childUsed = new HashMap<>();
          if (t instanceof EnumDeclaration enumDecl) {
            NodeList<EnumConstantDeclaration> entries = enumDecl.getEntries();
            for (int i = 0; i < entries.size(); i++) {
              EnumConstantDeclaration ec = entries.get(i);
              String eseg = segment(ec.getNameAsString(), "EnumConstantDeclaration", childUsed);
              out.add(new AddressableNode(id + "." + eseg, ec, "EnumConstantDeclaration", ec.getNameAsString(), line(ec), endLine(ec), simple, false, false, entries));
            }
          }
          NodeList<BodyDeclaration<?>> members = t.getMembers();
          int n = members.size(), i = 0;
          while (i < n) {
            BodyDeclaration<?> m = members.get(i);
            if (m instanceof TypeDeclaration<?> nested) {
              walkType(nested, members, simple, id, childUsed, out);
              i++;
              continue;
            }
            if (isDefinitionMember(m)) {
              Described d = describe(m);
              String mseg = segment(d.name, d.type, childUsed);
              out.add(new AddressableNode(id + "." + mseg, m, d.type, d.name, line(m), endLine(m), simple, false, true, members));
              i++;
              continue;
            }
            int start = i, length = 0;
            while (i < n) {
              BodyDeclaration<?> current = members.get(i);
              if (current instanceof TypeDeclaration<?> || isDefinitionMember(current))
                break;
              int piece = current.toString().length();
              if (i > start && length + piece > SEGMENT_MAX_CHARS)
                break;
              length += piece;
              i++;
            }
            addGroup(members, NodeGroup.KIND_STATEMENTS, simple, id, start, i, childUsed, out);
          }
        }

        /** Adds one anonymous, content-hash-addressed {@link NodeGroup} node spanning {@code [start, end)} of {@code container}. */
        private static void addGroup(NodeList<? extends Node> container, String kind, String parentType, String parentId, int start, int end, Map<String, Integer> used, List<AddressableNode> out) {
          List<? extends Node> slice = container.subList(start, end);
          StringBuilder sb = new StringBuilder();
          for (Node m : slice) {
            if (sb.length() > 0)
              sb.append('\n');
            sb.append(m.toString());
          }
          String seg = anonymousSegment(sb.toString(), used);
          String id = parentId.isEmpty() ? seg : parentId + "." + seg;
          Node first = slice.get(0);
          Node last = slice.get(slice.size() - 1);
          NodeGroup group = new NodeGroup(kind, container, start, end);
          out.add(new AddressableNode(id, group, kind, null, line(first), endLine(last), parentType, false, false, container));
        }

        private static boolean isDefinitionMember(BodyDeclaration<?> m) {
          return m instanceof MethodDeclaration || m instanceof ConstructorDeclaration;
        }

        private static boolean hasDefinitionMembers(TypeDeclaration<?> t) {
          for (BodyDeclaration<?> m : t.getMembers()) if (m instanceof TypeDeclaration<?> || m instanceof MethodDeclaration || m instanceof ConstructorDeclaration)
            return true;
          return false;
        }

        private record Described(String name, String type) {
        }

        private static Described describe(BodyDeclaration<?> m) {
          if (m instanceof MethodDeclaration md)
            return new Described(md.getNameAsString(), "MethodDeclaration");
          ConstructorDeclaration cd = (ConstructorDeclaration) m;
          return new Described(cd.getNameAsString(), "ConstructorDeclaration");
        }

        private static String simpleName(Node n) {
          return n.getClass().getSimpleName();
        }

        private static String segment(String name, String type, Map<String, Integer> used) {
          String base = name != null ? sanitize(name) : type;
          int count = used.merge(base, 1, Integer::sum);
          return count == 1 ? base : base + "_" + count;
        }

        private static String sanitize(String name) {
          return UNSAFE.matcher(name).replaceAll("_");
        }

        /** Unique-within-siblings id segment for an anonymous group node: {@code "<shapeHash>|<contentHash>"}, both 6-char base62. */
        private static String anonymousSegment(String content, Map<String, Integer> used) {
          String base = contentPrefixHash(content) + "|" + contentHash(content);
          int count = used.merge(base, 1, Integer::sum);
          return count == 1 ? base : base + "_" + count;
        }

        private static String contentHash(String content) {
          return base62Hash(content, 6);
        }

        /** Hash of the group's whitespace-stripped first/last 20 chars: stays put even when unrelated edits shift the group's interior. */
        private static String contentPrefixHash(String content) {
          String stripped = content.replaceAll("\\s+", "");
          int len = stripped.length();
          String prefix = stripped.substring(0, Math.min(20, len));
          String suffix = stripped.substring(Math.max(0, len - 20));
          return base62Hash(prefix + suffix, 6);
        }

        private static String base62Hash(String text, int length) {
          byte[] digestBytes;
          try {
            digestBytes = MessageDigest.getInstance("SHA-1").digest(text.getBytes(StandardCharsets.UTF_8));
          } catch (NoSuchAlgorithmException e) {
            throw new IllegalStateException(e);
          }
          BigInteger digest = new BigInteger(1, digestBytes);
          BigInteger base = BigInteger.valueOf(HASH_ALPHABET.length());
          StringBuilder sb = new StringBuilder();
          for (int i = 0; i < length; i++) {
            BigInteger[] dr = digest.divideAndRemainder(base);
            digest = dr[0];
            sb.append(HASH_ALPHABET.charAt(dr[1].intValue()));
          }
          return sb.toString();
        }

        private static int line(Node n) {
          return n.getRange().map(r -> r.begin.line).orElse(1);
        }

        private static int endLine(Node n) {
          return n.getRange().map(r -> r.end.line).orElse(1);
        }
      }
  reason: "NodeLocator umbauen: Imports und nicht-definierende Klassenmitglieder zu semantischen Blöcken gruppieren statt einzeln aufzulisten\""
```
/answer 243b-14 allow
Control Request:
```yaml
id: 243b-15
toolName: edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    old_text: |2-
        public void replace(AddressableNode target, String code) {
          Node replacement = parseReplacementFor(target.astNode, code);
          if (!target.astNode.replace(replacement))
            throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node could not be replaced in place: " + target.id);
        }

        @SuppressWarnings("unchecked")
        public int insert(AddressableNode target, String code, String position) {
          Node fragment = parseReplacementFor(target.astNode, code);
          NodeList<Node> container = (NodeList<Node>) target.container;
          int idx = container.indexOf(target.astNode);
          if (idx < 0)
            throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node no longer present: " + target.id);
          if ("after".equals(position))
            idx++;
          container.add(idx, fragment);
          return 1;
        }

        public void delete(AddressableNode target) {
          if (!target.astNode.remove())
            throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node could not be removed: " + target.id);
        }
    new_text: |2-
        public void replace(AddressableNode target, String code) {
          if (target.astNode instanceof NodeGroup group) {
            replaceGroup(group, code);
            return;
          }
          Node node = (Node) target.astNode;
          Node replacement = parseReplacementFor(node, code);
          if (!node.replace(replacement))
            throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node could not be replaced in place: " + target.id);
        }

        @SuppressWarnings("unchecked")
        public int insert(AddressableNode target, String code, String position) {
          if (target.astNode instanceof NodeGroup group)
            return insertGroup(group, code, position);
          Node node = (Node) target.astNode;
          Node fragment = parseReplacementFor(node, code);
          NodeList<Node> container = (NodeList<Node>) target.container;
          int idx = container.indexOf(node);
          if (idx < 0)
            throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node no longer present: " + target.id);
          if ("after".equals(position))
            idx++;
          container.add(idx, fragment);
          return 1;
        }

        public void delete(AddressableNode target) {
          if (target.astNode instanceof NodeGroup group) {
            deleteGroup(group);
            return;
          }
          Node node = (Node) target.astNode;
          if (!node.remove())
            throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node could not be removed: " + target.id);
        }

        @SuppressWarnings("unchecked")
        private NodeList<Node> groupContainer(NodeGroup group) {
          return (NodeList<Node>) group.container;
        }

        private void replaceGroup(NodeGroup group, String code) {
          NodeList<Node> container = groupContainer(group);
          List<Node> fragments = parseGroupFragments(group.kind, code);
          for (int i = group.end - 1; i >= group.start; i--) container.remove(i);
          container.addAll(group.start, fragments);
        }

        private int insertGroup(NodeGroup group, String code, String position) {
          NodeList<Node> container = groupContainer(group);
          List<Node> fragments = parseGroupFragments(group.kind, code);
          int idx = "after".equals(position) ? group.end : group.start;
          container.addAll(idx, fragments);
          return fragments.size();
        }

        private void deleteGroup(NodeGroup group) {
          NodeList<Node> container = groupContainer(group);
          for (int i = group.end - 1; i >= group.start; i--) container.remove(i);
        }

        private List<Node> parseGroupFragments(String kind, String code) {
          return NodeGroup.KIND_IMPORTS.equals(kind) ? parseImportsFragment(code) : parseMembersFragment(code);
        }

        /** Parses {@code code} as one or more import declarations, for an {@link NodeGroup#KIND_IMPORTS} group. */
        public List<Node> parseImportsFragment(String code) {
          CompilationUnit fragment = unwrap(parser.parse(code), "imports");
          List<Node> result = new ArrayList<>();
          for (ImportDeclaration imp : fragment.getImports()) result.add(imp.clone());
          if (result.isEmpty())
            throw new AstEngineException(AstEngineException.Kind.SYNTAX, "no import declarations found: " + code);
          return result;
        }

        /** Parses {@code code} as one or more member declarations, for a {@link NodeGroup#KIND_STATEMENTS} group. */
        public List<Node> parseMembersFragment(String code) {
          CompilationUnit wrapper = unwrap(parser.parse("class __Wrapper__ { " + code + " }"), "member declarations");
          ClassOrInterfaceDeclaration decl = (ClassOrInterfaceDeclaration) wrapper.getType(0);
          List<Node> result = new ArrayList<>();
          for (BodyDeclaration<?> m : decl.getMembers()) result.add(m.clone());
          if (result.isEmpty())
            throw new AstEngineException(AstEngineException.Kind.SYNTAX, "no member declarations found: " + code);
          return result;
        }
  reason: Mutationen (replace/insert/delete) um NodeGroup-Unterstützung erweitern"
```
/answer 243b-15 allow
Control Request:
```yaml
id: 243b-16
toolName: edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    old_text: |-
      import com.github.javaparser.ast.body.ClassOrInterfaceDeclaration;
      import com.github.javaparser.ast.body.ConstructorDeclaration;
    new_text: |-
      import com.github.javaparser.ast.body.ClassOrInterfaceDeclaration;
      import com.github.javaparser.ast.body.ConstructorDeclaration;
      import java.util.ArrayList;
  reason: ArrayList-Import für parseImportsFragment/parseMembersFragment ergänzen
```
/answer 243b-16 allow
Control Request:
```yaml
id: 243b-17
toolName: edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    old_text: |2-
        public String print(Node node) {
          Indentation indent = new Indentation(IndentType.SPACES, 2);
    new_text: |2-
        /** Dispatches to {@link #print(Node)} or, for a grouped segment, joins each member's own print. */
        public String print(Object node) {
          if (node instanceof NodeGroup group)
            return printGroup(group);
          return print((Node) node);
        }

        private String printGroup(NodeGroup group) {
          StringBuilder sb = new StringBuilder();
          for (Node m : group.members()) {
            if (sb.length() > 0)
              sb.append('\n');
            sb.append(print(m));
          }
          return sb.toString();
        }

        public String print(Node node) {
          Indentation indent = new Indentation(IndentType.SPACES, 2);
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    old_text: "  public String signature(Node node, int limit) {"
    new_text: |2-
        /** Dispatches to {@link #signature(Node, int)} or, for a grouped segment, its own first printed line. */
        public String signature(Object node, int limit) {
          if (node instanceof NodeGroup group) {
            String text = firstLine(printGroup(group)).replace('\n', ' ').replace('\r', ' ').strip();
            return text.length() <= limit ? text : text.substring(0, limit - 1) + "…";
          }
          return signature((Node) node, limit);
        }

        public String signature(Node node, int limit) {
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    old_text: "  public String docstring(Node node, int limit) {"
    new_text: |2-
        /** Dispatches to {@link #docstring(Node, int)}; a grouped segment never carries a Javadoc. */
        public String docstring(Object node, int limit) {
          if (node instanceof NodeGroup)
            return null;
          return docstring((Node) node, limit);
        }

        public String docstring(Node node, int limit) {
  reason: "Object-Overloads für print/signature/docstring hinzufügen, die NodeGroup-Segmente behandeln\""
```
/answer 243b-17 allow
Control Request:
```yaml
id: 243b-18
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
  reason: "Vollständige Datei nach den Edits prüfen (Konsistenz, fehlende Casts, Duplikate)"
```
/answer 243b-18 allow
Control Request:
```yaml
id: 243b-19
toolName: edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    old_text: |-
      import com.github.javaparser.ast.body.ConstructorDeclaration;
      import java.util.ArrayList;
      import com.github.javaparser.ast.body.EnumConstantDeclaration;
    new_text: |-
      import com.github.javaparser.ast.body.ConstructorDeclaration;
      import com.github.javaparser.ast.body.EnumConstantDeclaration;
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    old_text: |-
      import com.github.javaparser.ast.visitor.VoidVisitor;
      import com.github.javaparser.printer.configuration.DefaultPrinterConfiguration;
    new_text: |-
      import com.github.javaparser.ast.visitor.VoidVisitor;
      import java.util.ArrayList;
      import com.github.javaparser.printer.configuration.DefaultPrinterConfiguration;
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    old_text: |2-
        /**
         * Short first-paragraph rendering of {@code node}'s Javadoc, if any.
         */
        /** Dispatches to {@link #docstring(Node, int)}; a grouped segment never carries a Javadoc. */
        public String docstring(Object node, int limit) {
          if (node instanceof NodeGroup)
            return null;
          return docstring((Node) node, limit);
        }

        public String docstring(Node node, int limit) {
    new_text: |2-
        /** Dispatches to {@link #docstring(Node, int)}; a grouped segment never carries a Javadoc. */
        public String docstring(Object node, int limit) {
          if (node instanceof NodeGroup)
            return null;
          return docstring((Node) node, limit);
        }

        /**
         * Short first-paragraph rendering of {@code node}'s Javadoc, if any.
         */
        public String docstring(Node node, int limit) {
  reason: Import-Reihenfolge und Javadoc-Kommentare bereinigen
```
/answer 243b-19 allow
Control Request:
```yaml
id: 243b-1a
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/AddressableNode.java
  reason: Endgültigen Zustand von AddressableNode nach Edits prüfen"
```
/answer 243b-1a allow
Control Request:
```yaml
id: 243b-1b
toolName: edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/AddressableNode.java
    old_text: |-
      import com.github.javaparser.ast.Node;
      import com.github.javaparser.ast.NodeList;

      /**
       * One node the HTTP API can list/get/replace/insert/delete, plus the sibling
       * {@link NodeList} needed to splice a replacement fragment in (JavaParser's
       * {@code Node.replace}/{@code Node.remove} cover the other two mutations directly).
       */
    new_text: |-
      import com.github.javaparser.ast.NodeList;

      /**
       * One node the HTTP API can list/get/replace/insert/delete, plus the sibling
       * {@link NodeList} needed to splice a replacement fragment in (JavaParser's
       * {@code Node.replace}/{@code Node.remove} cover the other two mutations directly).
       * {@code astNode} is either a JavaParser {@link com.github.javaparser.ast.Node} or,
       * for a grouped "imports"/"statements" segment, a {@link NodeGroup}.
       */
  reason: Ungenutzten Import entfernen und astNode-Semantik dokumentieren
```
/answer 243b-1b allow
Result Stats: id=b50cc03c-97f5-45c9-90a2-d72ed3cca730, total: 126083, in: 233, out: 40597, reason: 357, read: 1030486, write: 85253
Text:
 Jetzt `JavaAstEngine.java` anpassen: `replace`/`insert`/`delete` gruppenfähig machen, `print`/`signature`/`docstring` um `Object`-Overloads erweitern, plus Fragment-Parser für Import-/Member-Gruppen.
ReasoningToken: 38
ReasoningToken: 62
Text:
 Nun die Datei zur Kontrolle vollständig lesen, um Konsistenz zu prüfen.
ReasoningToken: 257
Text:
 Nun `AddressableNode.java` und `JavaAstServer.java` prüfen, ob mit `Object astNode` alles noch kompiliert (implizite Overload-Auflösung).
Text:
 Import unused (`Node`) doesn't break compile. Alles konsistent. Der `import com.github.javaparser.ast.Node;` in AddressableNode ist jetzt unbenutzt aber harmlos; entferne ihn der Sauberkeit halber.
Die Java-AST-Engine gruppiert nun konsistent mit der Python/tree-sitter-Referenz:

- **`NodeLocator`**: Top-level Imports und nicht-definierende Klassenmitglieder (Felder, Initializer) werden zu zusammenhängenden `NodeGroup`-Segmenten (max. 500 Zeichen) zusammengefasst statt einzeln gelistet zu werden. Typen, Methoden, Konstruktoren und Enum-Konstanten bleiben individuell adressierbar.
- **`NodeGroup`**: neue Klasse für einen Bereich `[start,end)` innerhalb einer `NodeList`.
- **`AddressableNode.astNode`**: jetzt `Object` (JavaParser-`Node` oder `NodeGroup`).
- **`JavaAstEngine`**: `print`/`signature`/`docstring` per Overload gruppenfähig; `replace`/`insert`/`delete` behandeln Gruppen durch Parsen mehrerer Import-/Member-Deklarationen und Splicing im Container.
Enum konstanten sind einzeln addressierbar?
Result Stats: id=b50cc03c-97f5-45c9-90a2-d72ed3cca730, total: 126667, in: 235, out: 40779, reason: 63, read: 1115739, write: 85653
SystemInit:  id=b50cc03c-97f5-45c9-90a2-d72ed3cca730, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
ReasoningToken: 63
Ja – Enum-Konstanten bleiben wie zuvor einzeln adressierbar (nicht gruppiert), da sie üblicherweise gezielt einzeln bearbeitet werden. Nur Imports und nicht-definierende Klassenmitglieder (Felder, Initializer) werden zu Blöcken zusammengefasst.
Warum sollte man ein einzelnes Wort addressieren wollen?
SystemInit:  id=b50cc03c-97f5-45c9-90a2-d72ed3cca730, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Text:
 Gute Kritik – ein einzelner Enum-Konstante ist genauso feingranular wie ein einzelnes Feld. Das war inkonsistent. Ich gruppiere sie jetzt auch.
Control Request:
```yaml
id: 243b-1c
toolName: edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/NodeLocator.java
    old_text: |2-
          Map<String, Integer> childUsed = new HashMap<>();
          if (t instanceof EnumDeclaration enumDecl) {
            NodeList<EnumConstantDeclaration> entries = enumDecl.getEntries();
            for (int i = 0; i < entries.size(); i++) {
              EnumConstantDeclaration ec = entries.get(i);
              String eseg = segment(ec.getNameAsString(), "EnumConstantDeclaration", childUsed);
              out.add(new AddressableNode(id + "." + eseg, ec, "EnumConstantDeclaration", ec.getNameAsString(), line(ec), endLine(ec), simple, false, false, entries));
            }
          }
    new_text: |2-
          Map<String, Integer> childUsed = new HashMap<>();
          if (t instanceof EnumDeclaration enumDecl) {
            NodeList<EnumConstantDeclaration> entries = enumDecl.getEntries();
            int en = entries.size(), ei = 0;
            while (ei < en) {
              int start = ei, length = 0;
              while (ei < en) {
                int piece = entries.get(ei).toString().length();
                if (ei > start && length + piece > SEGMENT_MAX_CHARS)
                  break;
                length += piece;
                ei++;
              }
              addGroup(entries, NodeGroup.KIND_STATEMENTS, simple, id, start, ei, childUsed, out);
            }
          }
    exact: true
  reason: Enum-Konstanten wie andere nicht-definierende Member zu Blöcken gruppieren statt einzeln zu adressieren
```
/answer 243b-1c allow
ReasoningToken: 461
Control Request:
```yaml
id: 243b-1d
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/NodeLocator.java
    min_line: 1
    max_line: 110
  reason: Aktualisierte Datei nach Enum-Gruppierungs-Edit prüfen"
```
/answer 243b-1d allow
Control Request:
```yaml
id: 243b-1e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/NodeLocator.java
      checksum: 1c810d959e6741fe7386df58f37b9085939ac86dd0f5de03a698911856edba8c
      content: |
        package xy.ai.mcpc.ast.engine;

        import com.github.javaparser.ast.CompilationUnit;
        import com.github.javaparser.ast.ImportDeclaration;
        import com.github.javaparser.ast.Node;
        import com.github.javaparser.ast.NodeList;
        import com.github.javaparser.ast.body.BodyDeclaration;
        import com.github.javaparser.ast.body.ConstructorDeclaration;
        import com.github.javaparser.ast.body.EnumConstantDeclaration;
        import com.github.javaparser.ast.body.EnumDeclaration;
        import com.github.javaparser.ast.body.MethodDeclaration;
        import com.github.javaparser.ast.body.TypeDeclaration;
        import java.math.BigInteger;
        import java.nio.charset.StandardCharsets;
        import java.security.MessageDigest;
        import java.security.NoSuchAlgorithmException;
        import java.util.ArrayList;
        import java.util.HashMap;
        import java.util.List;
        import java.util.Map;
        import java.util.regex.Pattern;

        /**
         * Flattens a {@link CompilationUnit} into every addressable node, in document order —
         * the JavaParser equivalent of the Python/tree-sitter engines' {@code locate_all}.
         * Only types (classes/interfaces/enums/records/annotations), their nested types,
         * methods and constructors are individually addressable "definitions"; method/
         * constructor/initializer bodies are treated as leaves (not expanded further).
         * Everything else — imports at the top level, fields/initializers/annotation
         * members inside a type — is far too fine-grained to be individually useful, so
         * consecutive runs of it are collapsed into anonymous {@link NodeGroup} segments
         * capped at {@link #SEGMENT_MAX_CHARS}, exactly like the Python/tree-sitter engines'
         * "imports"/"statements" segments.
         *
         * Ids are name-based structural paths (e.g. {@code "MyClass.foo_2"}) for named
         * definitions, and stable content-hash paths (e.g. {@code "a1B2c3|d4E5f6"}) for
         * anonymous group segments, rebuilt fresh on every call from the live tree; stable
         * as long as the declaration order/names/content haven't changed since the id was
         * handed out. A node no longer found by id is reported as a 404 by the caller, not
         * silently misresolved.
         */
        public final class NodeLocator {
          /** A group keeps accumulating siblings until adding the next one would push its source past this many characters (then it splits). */
          private static final int SEGMENT_MAX_CHARS = 500;
          private static final Pattern UNSAFE = Pattern.compile("[^A-Za-z0-9_]+");
          private static final String HASH_ALPHABET = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

          private NodeLocator() {
          }

          public static List<AddressableNode> locateAll(CompilationUnit cu) {
            List<AddressableNode> out = new ArrayList<>();
            groupImports(cu, out);
            Map<String, Integer> usedTypes = new HashMap<>();
            NodeList<TypeDeclaration<?>> types = cu.getTypes();
            for (int i = 0; i < types.size(); i++) walkType(types.get(i), types, "CompilationUnit", "", usedTypes, out);
            return out;
          }

          private static void groupImports(CompilationUnit cu, List<AddressableNode> out) {
            NodeList<ImportDeclaration> imports = cu.getImports();
            Map<String, Integer> used = new HashMap<>();
            int n = imports.size(), i = 0;
            while (i < n) {
              int start = i, length = 0;
              while (i < n) {
                int piece = imports.get(i).toString().length();
                if (i > start && length + piece > SEGMENT_MAX_CHARS)
                  break;
                length += piece;
                i++;
              }
              addGroup(imports, NodeGroup.KIND_IMPORTS, "CompilationUnit", "", start, i, used, out);
            }
          }

          private static void walkType(TypeDeclaration<?> t, NodeList<?> container, String parentType, String parentId, Map<String, Integer> used, List<AddressableNode> out) {
            String simple = simpleName(t);
            String seg = segment(t.getNameAsString(), simple, used);
            String id = parentId.isEmpty() ? seg : parentId + "." + seg;
            boolean expandable = hasDefinitionMembers(t);
            out.add(new AddressableNode(id, t, simple, t.getNameAsString(), line(t), endLine(t), parentType, expandable, true, container));
            Map<String, Integer> childUsed = new HashMap<>();
            if (t instanceof EnumDeclaration enumDecl) {
              NodeList<EnumConstantDeclaration> entries = enumDecl.getEntries();
              int en = entries.size(), ei = 0;
              while (ei < en) {
                int start = ei, length = 0;
                while (ei < en) {
                  int piece = entries.get(ei).toString().length();
                  if (ei > start && length + piece > SEGMENT_MAX_CHARS)
                    break;
                  length += piece;
                  ei++;
                }
                addGroup(entries, NodeGroup.KIND_STATEMENTS, simple, id, start, ei, childUsed, out);
              }
            }
            NodeList<BodyDeclaration<?>> members = t.getMembers();
            int n = members.size(), i = 0;
            while (i < n) {
              BodyDeclaration<?> m = members.get(i);
              if (m instanceof TypeDeclaration<?> nested) {
                walkType(nested, members, simple, id, childUsed, out);
                i++;
                continue;
              }
              if (isDefinitionMember(m)) {
                Described d = describe(m);
                String mseg = segment(d.name, d.type, childUsed);
```
/answer 243b-1e allow
Result Stats: id=b50cc03c-97f5-45c9-90a2-d72ed3cca730, total: 134382, in: 243, out: 43332, reason: 355, read: 1462296, write: 90807
ReasoningToken: 37
ReasoningToken: 318
Sieht korrekt aus. Enum-Konstanten werden jetzt ebenso wie Felder/Initializer zu `StatementGroup`-Blöcken zusammengefasst statt einzeln adressiert.
