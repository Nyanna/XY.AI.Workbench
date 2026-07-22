Erkläre mir die Struktur des MDAST.
Agent:
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=a4dd2ac6-c6eb-400e-b385-945a640b624b | model=claude-sonnet-5 | plugins=
ReasoningToken: 152
# MDAST – Markdown Abstract Syntax Tree

MDAST ist eine Spezifikation dafür, wie Markdown-Dokumente als **abstrakter Syntaxbaum (AST)** repräsentiert werden. Es ist Teil des **unist**-Ökosystems (Universal Syntax Tree) und wird vor allem von **remark** (dem Markdown-Prozessor im unified-Framework) verwendet.

## Grundprinzip: unist

MDAST baut auf **unist** auf, das definiert, wie Syntaxbäume generell aussehen. Jeder Knoten (Node) hat mindestens:

```js
{
  type: "...",       // Art des Knotens
  children: [...],   // optional, bei Eltern-Knoten
  value: "...",      // optional, bei Text-/Blattknoten
  position: {...}    // optional, Position im Quelltext (Zeile/Spalte/Offset)
}
```

## Hierarchie der Knotentypen

### 1. Root
Der Wurzelknoten des gesamten Dokuments:
```js
{ type: "root", children: [...] }
```

### 2. Block-Knoten (Flow Content)
Strukturieren das Dokument auf oberster Ebene:

| Typ | Beispiel Markdown | Beschreibung |
|---|---|---|
| `heading` | `# Titel` | hat `depth` (1–6) |
| `paragraph` | normaler Text | |
| `blockquote` | `> Zitat` | |
| `list` | `- item` | hat `ordered`, `spread`, `start` |
| `listItem` | einzelnes Listenelement | hat `checked` (Task-Listen) |
| `code` | ```` ```js ``` ```` | Codeblock, hat `lang`, `meta`, `value` |
| `thematicBreak` | `---` | horizontale Linie |
| `html` | eingebettetes HTML | als Rohtext |
| `table` | GFM-Tabelle | hat `align` |
| `tableRow`, `tableCell` | Tabellenzeilen/-zellen | |

### 3. Inline-Knoten (Phrasing Content)
Stehen innerhalb von Blöcken:

| Typ | Beispiel | Beschreibung |
|---|---|---|
| `text` | einfacher Text | hat `value` |
| `emphasis` | `*kursiv*` | Kind-Knoten |
| `strong` | `**fett**` | Kind-Knoten |
| `delete` | `~~durchgestrichen~~` | (GFM) |
| `inlineCode` | `` `code` `` | hat `value` |
| `break` | erzwungener Zeilenumbruch | |
| `link` | `[text](url)` | hat `url`, `title` |
| `image` | `![alt](url)` | hat `url`, `alt`, `title` |
| `linkReference` / `imageReference` | `[text][ref]` | referenzierte Links |
| `footnoteReference` | `[^1]` | |

### 4. Sonstige
- `definition` – Link-Referenz-Definitionen (`[ref]: url`)
- `footnoteDefinition` – Fußnoteninhalte
- `yaml` – Frontmatter (mit Plugin)

## Beispiel

Markdown:
```markdown
# Hallo

Das ist **fett** und *kursiv*.
```

Wird zu MDAST (vereinfacht):

```js
{
  type: "root",
  children: [
    {
      type: "heading",
      depth: 1,
      children: [{ type: "text", value: "Hallo" }]
    },
    {
      type: "paragraph",
      children: [
        { type: "text", value: "Das ist " },
        { type: "strong", children: [{ type: "text", value: "fett" }] },
        { type: "text", value: " und " },
        { type: "emphasis", children: [{ type: "text", value: "kursiv" }] },
        { type: "text", value: "." }
      ]
    }
  ]
}
```

## Warum wichtig?

- **Transformierbarkeit**: Plugins (z. B. remark-gfm, remark-toc) können den Baum manipulieren, bevor er zu HTML (via `mdast-util-to-hast`), JSX oder anderen Formaten wird.
- **Werkzeuge**: `unist-util-visit` erlaubt einfaches Durchlaufen/Verändern des Baums.
- **Ökosystem**: MDX, remark, rehype nutzen alle dasselbe Prinzip – nur mit unterschiedlichen Baum-Typen (mdast → hast für HTML).

Möchtest du ein konkretes Beispiel mit `unified`/`remark`-Code sehen, wie man MDAST erzeugt oder manipuliert?

User:
Ich einen auf SWT basierten Editor mit einem MDAST versehen. Der Zweck ist leichteres Handling extrem großer Dateien sowie besser Region Support für Spellcheck und Syntaxhervorhebung. Gibt es Implementierungen in Java?
Agent:
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=a4dd2ac6-c6eb-400e-b385-945a640b624b | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: b90613aa-11c9-4680-a81e-67e613beec27
toolName: web-search-exa
arguments:
  query: Java Markdown parser AST source position flexmark commonmark-java SWT editor
  reason: Find Java MDAST-like implementations suitable for SWT editor
```
/answer b90613aa-11c9-4680-a81e-67e613beec27 allow
User:

Agent:
ReasoningToken: 259
Control Request:
```yaml
id: 4ba4efe4-c1bb-4df0-b9ee-8f908791736f
result:
  structuredContent:
    content: |-
      Title: vsch/flexmark-java: CommonMark/Markdown ...
      URL: https://github.com/vsch/flexmark-java
      Published: 2016-01-23T15:29:29.000Z
      Author: N/A
      Highlights:
      CommonMark/Markdown Java parser with source level AST. CommonMark 0.28, emulation of: pegdown, kramdown, markdown.pl, MultiMarkdown. With HTML to MD, MD to PDF, MD to DOCX conversion modules.
      ...
      **flexmark-java** is a Java implementation of **[CommonMark (spec 0.28)]** parser using the
      blocks first, inlines after Markdown parsing architecture.
      ...
      Its strengths are speed, flexibility, Markdown source element based AST with details of the
      source position down to individual characters of lexemes that make up the element and
      extensibility.
      ...
      **flexmark-java** is a fork of [commonmark-java] project, modified to generate an AST which
      reflects all the elements in the original source, full source position tracking for all elements
      in the AST and easier JetBrains Open API PsiTree generation.
      ...
      [commonmark-java] has an excellent parsing architecture that is easy to understand and extend.
      The goal was to ensure that adding source position tracking in the AST would not change the ease
      of parsing and generating the AST more than absolutely necessary.
      ...
      | Feature | flexmark-java | commonmark-java | pegdown |
      ...
      |:---------------------------------------------------------------------------------|:-----------------------------------------------------------------|:------------------------------------------------------------------|:---------------------------------------------------------------------|
      ...
      parse time (less
      ...
      better) | :heavy_check_mark: 1x
      ...
      1)](#1) | :
      ...
      ) | :x
      ...
      in the AST | :heavy_check_mark: | :x: | :heavy_check_mark: |
      ...
      | AST elements with source position | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: with some errors and idiosyncrasies |
      ...
      | AST elements have detailed source position for all parts | :heavy_check_mark: | :x: | :x: only node start/end |
      ...
      I am very pleased with the decision to switch to [commonmark-java] based parser for my own
      projects. Even though I had to do major surgery on its innards to get full source position
      tracking and AST that matches source elements, it is a pleasure to work with and is now a
      pleasure to extend. If you don't need source level element AST or the rest of what flexmark-java
      added and [CommonMark] is your target markdown parser then I encourage you to use
      [commonmark-java] as it is an excellent choice for your needs and its performance does not
      suffer for the overhead of features that you will not use.

      ---

      Title: README.md
      URL: https://github.com/vsch/flexmark-java/blob/master/README.md
      Published: N/A
      Author: N/A
      Highlights:
      **flexmark-java** is a Java implementation of **[CommonMark (spec 0.28)]** parser using the
      blocks first, inlines after Markdown parsing architecture.
      ...
      Its strengths are speed, flexibility, Markdown source element based AST with details of the
      source position down to individual characters of lexemes that make up the element and
      extensibility.
      ...
      **flexmark-java** is a fork of [commonmark-java] project, modified to generate an AST which
      reflects all the elements in the original source, full source position tracking for all elements
      in the AST and easier JetBrains Open API PsiTree generation.
      ...
      changed to allow
      ...
      installed extensions.
      ...
      extensions come with many
      ...
      tweaking options for
      ...
      The end goal is to have the
      ...
      renderer be able
      ...
      with great degree
      ...
      [commonmark-java] has an excellent parsing architecture that is easy to understand and extend.
      The goal was to ensure that adding source position tracking in the AST would not change the ease
      of parsing and generating the AST more than absolutely necessary.
      ...
      Feature | flexmark-java | commonmark-java
      ...
      | All source elements in the AST | :heavy_check_mark: | :x: | :heavy_check_mark: |
      ...
      | AST elements with source position | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: with some errors and idiosyncrasies |
      ...
      | :heavy_check_mark
      ...
      mechanism | :heavy
      ...
      check_mark
      ...
      is an extension mechanism | :x
      ...
      node's parent
      ...
      , children as List<>. |
      ...
      | AST elements have detailed source position for all parts | :heavy_check_mark: | :x: | :x: only node start/end |
      ...
      I am very pleased with the decision to switch to [commonmark-java] based parser for my own
      projects. Even though I had to do major surgery on its innards to get full source position
      tracking and AST that matches source elements, it is a pleasure to work with and is now a
      pleasure to extend. If you don't need source level element AST or the rest of what flexmark-java
      added and [CommonMark] is your target markdown parser then I encourage you to use
      [commonmark-java] as it is an excellent choice for your needs and its performance does not
      suffer for the overhead of features that you will not use.

      ---

      Title: flexmark/src/main/java/com/vladsch/flexmark/parser/Parser.java at master · vsch/flexmark-java
      URL: https://github.com/vsch/flexmark-java/blob/master/flexmark/src/main/java/com/vladsch/flexmark/parser/Parser.java
      Published: N/A
      Author: N/A
      Highlights:
      - Repository: vsch/flexmark-java | CommonMark/Markdown Java parser with source level AST. CommonMark 0.28, emulation of: pegdown, kramdown, markdown.pl, MultiMarkdown. With HTML to MD, MD to PDF, MD to DOCX conversion modules. | 3K stars | Java
      - Branch: master
      ...
      /**
       * Parses input text to a tree of nodes.
       * <p>
       * Start with the {@link #builder} method, configure the parser and build it. Example:
       * <pre>{@code
       * Parser parser = Parser.builder().build();
       * Node document = parser.parse("input text");
       * }</pre>
       */
      public class Parser implements IParse {
          final public static DataKey<Collection<Extension>> EXTENSIONS = SharedDataKeys.EXTENSIONS;

          final public static DataKey<KeepType> REFERENCES_KEEP = new DataKey<>("REFERENCES_KEEP", KeepType.FIRST);
          final public static DataKey<ReferenceRepository> REFERENCES = new DataKey<>("REFERENCES", new ReferenceRepository(null), ReferenceRepository::new);

          final public static DataKey<Boolean> ASTERISK_DELIMITER_PROCESSOR = new DataKey<>("ASTERISK_DELIMITER_PROCESSOR", true);

          final public static DataKey
      ...
      TRACK_DOCUMENT_LINES = new DataKey<>("TRACK_DOCUMENT_LINES", false);
      ...
      final public static DataKey<Boolean> UND
      ...
      _PROCESSOR = new DataKey<>("UNDERSCORE_DELIMITER_PROCESSOR", true);
          final public static DataKey<Boolean
      ...
      _AST = SharedDataKeys.BLANK_LINES_IN
      ...
      ALLOW_NAME_SPACE", false);
      ...
      LISTS_ITEM_INDENT or double that where indentation counts from first list item indent
          final public static DataKey<Integer> LISTS_CODE_INDENT = new DataKey<>("LISTS_CODE_INDENT", 4);
          final public static DataKey<Integer> LISTS_ITEM_INDENT = new DataKey<>("LIST
      ...
      4);
      ...
      );
      ...
      LISTS_ITEM_INDENT
      ...
      List<CustomBlockParserFactory> blockParser
      ...
      ;
          final private Map<Character, DelimiterProcessor> delimiterProcessors;
          final private BitSet delimiterCharacters;
          final private BitSet specialCharacters;
          final private List<PostProcessorManager.PostProcessorDependencyStage> postProcessorDependencies;
          final private List<List<ParagraphPreProcessorFactory>> paragraphPreProcessorFactories;
          final private List<List<BlockPreProcessorFactory>> blockPreProcessorDependencies;
          final private
      ...
      ProcessorData linkRefProcessors;
          final private List<InlineParserExtensionFactory> inlineParserExtensionFactories;
          final private InlineParserFactory inlineParserFactory;
      ...
      DataSet options = builder.toImmutable();
              this.blockParserFactories = DocumentParser.calculateBlockParserFactories(options, builder.blockParserFactories);

              List<Special
      ...
      InHandlers.add(escaper);
                  }
              }

              MutableDataSet optionsWithSpecialLead
      ...
      Handlers = new MutableDataSet(builder);
              optionsWithSpecial
      ...
      InHandlers.set(SPECIAL_LEAD_IN_HANDLERS, specialLeadInHandlers);

              this.options = optionsWithSpecial
      ...
      Parser}.
           *
           * @return a builder
           */
          public static Builder builder() {
              return new Builder();
          }

          public static Builder builder(DataHolder options) {
              return new Builder(options);
          }

          /**
           * Parse the specified input text into a tree of nodes.
           * <p>
           * Note that this method is thread-safe (a new parser state is used for each invocation).
           *
           * @param input the text to parse
           * @return the root node
           */
          public @NotNull Document parse(@NotNull BasedSequence input) {
              // NOTE: parser can only handle contiguous sequences with no out of base characters
              if (input instanceof ReplacedBasedSequence) {
                  throw new IllegalArgumentException("" +
                          "Parser.parse() does not support BasedSequences with replaced or non-contiguous segments.\n" +
                          "Use BasedSequence.of(input.toString()) to convert to contiguous based sequence." +
                          "");
              }

              DocumentParser documentParser = new DocumentParser(options
                      , blockParserFactories
                      , paragraphPreProcessorFactories
                      , blockPreProcessorDependencies
                      , inlineParserFactory.inlineParser(options, specialCharacters, delimiterCharacters, delimiterProcessors, linkRefProcessors, inlineParserExtensionFactories));
              Document document = documentParser.parse(input);
              return postProcess(document);
          }

          /**
           * Parse the specified input text into a tree of nodes.
           * <p>
           * Note that this method is thread-safe (a new parser state is used for each invocation).
           *
           * @param input the text to parse
           * @return the root node
           */
          public @NotNull Document parse(@NotNull String input) {
              DocumentParser documentParser = new DocumentParser(options
                      , blockParserFactories
                      , paragraphPreProcessorFactories
                      , blockPreProcessorDependencies
                      , inlineParserFactory.inlineParser(options, specialCharacters, delimiterCharacters, delimiterProcessors, linkRefProcessors, inlineParserExtensionFactories));
              Document document = documentParser.parse(BasedSequence.of(input));
              return postProcess(document);
          }

          /**
           * Parse the specified reader into a tree of nodes. The caller is responsible for closing the reader.
           * <p>
           * Note that this method is thread-safe (a new parser state is used for each invocation).
           *
           * @param input the reader to parse
           * @return the root node
           * @throws IOException when reading throws an exception
           */
          public @NotNull Document parseReader(@NotNull Reader input) throws IOException {
              DocumentParser documentParser = new DocumentParser(options
                      , blockParserFactories
                      , paragraphPreProcessorFactories
                      , blockPreProcessorDependencies
                      , inlineParserFactory.inlineParser(options, specialCharacters, delimiterCharacters, delimiterProcessors, linkRefProcessors, inlineParserExtensionFactories));
              Document document = documentParser.parse(input);
              return postProcess(document);
          }

          private Document postProcess(Document document) {
              document = PostProcessorManager.processDocument(document, postProcessorDependencies);
              return document;
          }
      ...
      }
      ...
      this;
              }
          }

          /**
           * Extension for {@link Parser}.
           * <p>
           * Implementations of this interface should done by all Extensions that extend the core parser
           * <p>
           * Each will be called via {@link ParserExtension#extend(Builder)} method giving it a chance to call back
           * on the builder methods to register parser extension points
           */
          public interface ParserExtension extends Extension {
              /**
               * This method is called first on all extensions so that they can adjust the options that must be common to all extensions.
               *
               * @param options option set that will be used for the builder
               */
              void parserOptions(MutableDataHolder options);

              /**
      ...
      method is called on all extensions so that they can register their custom processors
               *
               * @param parserBuilder parser builder with which to register extensions
               * @see Builder#customBlockParserFactory(CustomBlockParserFactory)
               * @see Builder#customInlineParserExtensionFactory(InlineParserExtensionFactory)
               * @see Builder#customInlineParserFactory(InlineParserFactory)
               * @see Builder#customDelimiterProcessor(DelimiterProcessor)
               * @see Builder#postProcessorFactory(PostProcessorFactory)
               * @see Builder#paragraphPreProcessorFactory(ParagraphPreProcessorFactory)
               * @see Builder#blockPreProcessorFactory(BlockPreProcessorFactory)
               * @see Builder#linkRefProcessorFactory(LinkRefProcessorFactory)
               * @see Builder#specialLeadInHandler(SpecialLeadInHandler)
               */
              void extend(Builder parserBuilder);
          }

          /**
           *

      ---

      Title: commonmark/commonmark-java
      URL: https://github.com/commonmark/commonmark-java
      Published: N/A
      Author: N/A
      Highlights:
      Java library for parsing and rendering CommonMark (Markdown)
      ...
      Java library for parsing and rendering [Markdown] text according to the
      [CommonMark] specification (and some extensions).
      ...
      Provides classes for parsing input to an abstract syntax tree (AST),
      visiting and manipulating nodes, and rendering to HTML or back to Markdown.
      It
      ...
      as a port of [commonmark.js
      ...
      has since evolved into an
      extensible library with the following features:
      ...
      * Small (core has no dependencies, extensions in separate artifacts)
      * Fast (10-20 times faster than [pegdown] which used to be a popular Markdown
       library, see benchmarks in repo)
      * Flexible (manipulate the AST after parsing, customize HTML rendering)
      * Extensible (tables, strikethrough, autolinking and more, see below)
      ...
      #### Source positions
      ...
      If you want to know where a parsed `Node` appeared in the input source text,
      you can request the parser to return source positions like this:
      ...
      ```java
      var parser = Parser.builder().includeSourceSpans(IncludeSourceSpans.BLOCKS_AND_INLINES).build();
      ```
      ...
      Then parse nodes and inspect source positions:
      ...
      ```java
      var source = "foo\n\nbar *baz*";
      var doc = parser.parse(source);
      var emphasis = doc.getLastChild().getLastChild();
      var s = emphasis.getSourceSpans().get(0);
      s.getLineIndex();    // 2 (third line)
      s.getColumnIndex();  // 4 (fifth column)
      s.getInputIndex();   // 9 (string index 9)
      s.getLength();       // 5
      source.substring(s.getInputIndex(), s.getInputIndex() + s.getLength());  // "*baz*"
      ```
      ...
      If you're only interested in blocks and not inlines, use `IncludeSourceSpans.BLOCKS`.
      ...
      * [Markwon](https://github.com/noties/Markwon):
      ...
      system-native Spannables
      * [flexmark-java](https://github.com/vsch/flexmark-java): Fork that added support for a lot more syntax and flexibility

      ---

      Title: src/com/vladsch/Main.java
      URL: https://github.com/vsch/flexmark-java-sample/blob/master/src/com/vladsch/Main.java
      Published: N/A
      Author: N/A
      Highlights:
      import com.vladsch.flexmark.ast.Node;
      ...
      import com.vladsch.flexmark.parser.Parser;
      ...
      public class Main {
       public enum ForUseBy {
       PARSER,
       JAVAFX,
       SWING,
       HTML
       }

       static class Options {
       public boolean abbreviations = false;
       public boolean autoLinks = true;
       public boolean anchorLinks = true;
       public boolean definitions = false;
       public boolean fencedCode = true;
       public boolean hardWraps = false;
       public boolean atxHeadingSpace = true;
       public boolean typographicQuotes = false;
       public boolean typographicSmarts = false;
       public boolean relaxedThematicBreak = true;
       public boolean strikeThrough = true;
       public boolean tables = true;
       public boolean renderTablesGFM = true;
       public boolean taskListItems = true;
       public boolean wikiLinks = false;
       public boolean wikiLinkGfmSyntax = true;
       public boolean footnotes = false;
       public boolean tableOfContents = true;
       public boolean jekyllFrontMatter = false;
       public boolean emojiShortcuts = false;
       public String emojiImageDirectory = "";
       }

       private static MutableDataHolder options(ForUseBy purpose, Options options) {
       MutableDataSet dataSet = new MutableDataSet();
       ArrayList extensions = new ArrayList<>();

       dataSet.set(Parser.PARSE_INNER_HTML_COMMENTS, true);
       dataSet.set(Parser.INDENTED_CODE_NO_TRAILING_BLANK_LINES, true);
       dataSet.set(HtmlRenderer.SUPPRESS_HTML_BLOCKS, false);
       dataSet.set(HtmlRenderer.SUPPRESS_INLINE_HTML, false);

       // add default extensions in pegdown
       extensions.add(EscapedCharacterExtension.create());

       // Setup Block Quote Options
       dataSet.set(Parser.BLOCK_QUOTE_TO_BLANK_LINE, true);

       // Setup List Options for GitHub profile
       dataSet.set(Parser.LISTS_AUTO_LOOSE, false);
       dataSet.set(Parser.LISTS_AUTO_LOOSE, false);
       dataSet.set(Parser.LISTS_BULLET_MATCH, false);
       dataSet.set(Parser.LISTS_ITEM_TYPE_MATCH, false);
       dataSet.set(Parser.LISTS_ITEM_MISMATCH_TO_SUBITEM, false);
       dataSet.set(Parser.LISTS_END_ON_DOUBLE_BLANK, false);
       dataSet.set(Parser.LISTS_FIXED_INDENT, 4);
       dataSet.set(Parser.LISTS_BULLET_ITEM_INTERRUPTS_PARAGRAPH, false);
       dataSet.set(Parser.LISTS_BULLET_ITEM_INTERRUPTS_ITEM_PARAGRAPH, true);
       dataSet.set(Parser.LISTS_ORDERED_ITEM_DOT_ONLY, true);
       dataSet.set(Parser.LISTS_ORDERED_ITEM_INTERRUPTS_PARAGRAPH, false);
       dataSet.set(Parser.LISTS_ORDERED_ITEM_INTERRUPTS_ITEM_PARAGRAPH, true);
       dataSet.set(Parser.LISTS_ORDERED_NON_ONE_ITEM_INTERRUPTS_PARAGRAPH, false);
       dataSet.set(Parser.LISTS_ORDERED_NON_ONE_ITEM_INTERRUPTS_PARENT_ITEM_PARAGRAPH, true);
       dataSet.set(Parser.LISTS_ORDERED_LIST_MANUAL_START, false);

       if (options.abbreviations) {
       extensions.add(AbbreviationExtension.create());
       dataSet.set(AbbreviationExtension.ABBREVIATIONS_KEEP, KeepType.LAST);
       }

       if (options.anchorLinks) {
       extensions.add(AnchorLinkExtension.create());
       dataSet.set(AnchorLinkExtension.ANCHORLINKS_WRAP_TEXT, true);
       }

       if (options.autoLinks) {
       extensions.add(AutolinkExtension.create());
       }

       if (options.definitions) {
       // not implemented yet, but have placeholder
       extensions.add(DefinitionExtension.create());
       }

       if (options.fencedCode) {
       // disable fenced code blocks
       dataSet.set(Parser.MATCH_CLOSING_FENCE_CHARACTERS, false);
       } else {
       dataSet.set(Parser.FENCED_CODE_BLOCK_PARSER, false);
       }

       if (options.hardWraps) {
       dataSet.set(HtmlRenderer.SOFT_BREAK, " \n");
       dataSet.set(HtmlRenderer.HARD_BREAK, " \n \n");
       }

       if (!options.atxHeadingSpace) {
       dataSet.set(Parser.HEADING_NO_ATX_SPACE, true);
       }
       dataSet.set(Parser.HEADING_NO_LEAD_SPACE, true);

       if (purpose == ForUseBy.PARSER) {
       // 3 for pegdown compatibility, 1 for commonmark, something else for GFM which will take 1 without trailing spaces if in a list, outside a list 1 or 2+ with spaces even if in a list
       dataSet.set(Parser.HEADING_SETEXT_MARKER_LENGTH, 3);
       }

       if (options.typographicQuotes || options.typographicSmarts) {
       // not implemented yet, have placeholder
       extensions.add(TypographicExtension.create());
       dataSet.set(TypographicExtension.TYPOGRAPHIC_SMARTS, options.typographicSmarts);
       dataSet.set(TypographicExtension.TYPOGRAPHIC_QUOTES, options.typographicQuotes);
       }
      ...
      if (options.emojiShortcuts) {
       // requires copying the emoji images to some directory and setting it here
       extensions.add(EmojiExtension.create());
       if (options.emojiImageDirectory.isEmpty()) {
       dataSet.set(EmojiExtension.USE_IMAGE_URLS, true);
       } else {
       dataSet
      ...
      Extension.ROOT_
      ...
      _PATH, options.
      ...
      Directory);
       }
       }

       if (purpose == ForUseBy.JAVAFX) {
       // set rendering
      ...
      for JavaFX
       // set to true if java fx, else false
       dataSet.set(HtmlRenderer.INDENT_SIZE, 2);
       dataSet.set(Parser.LISTS_LOOSE_ON_PREV_LOOSE_ITEM, true);

       if (options.tables && options.renderTablesGFM) {
       dataSet.set(TablesExtension.COLUMN_SPANS, false)
       .set(TablesExtension.MIN_HEADER_ROWS,
      ...
      1)
       .set(TablesExtension.MAX_HEADER_ROWS, 1)
       .set(TablesExtension.APPEND_MISSING_COLUMNS, true)
       .set(TablesExtension.DISCARD_EXTRA_COLUMNS, true)
       .set(TablesExtension.HEADER_SEPARATOR_COLUMN_MATCH, true);
       }

       if (options.fencedCode) {
       dataSet.set(HtmlRenderer.FENCED_CODE_
      ...
      _CLASS_PREFIX, "");
       }

       if (options.anchorLinks) {
       dataSet.set(AnchorLinkExtension.ANCHORLINKS_SET_
      ...
      , true);
       dataSet.set(AnchorLinkExtension.ANCHORLINKS_ANCHOR_CLASS, "anchor");
       dataSet.set(
      ...
      Extension.ANCHOR
      ...
      KS_SET_NAME, true);
       dataSet.set(
      ...
      Extension.ANCHOR
      ...
      KS_TEXT_PREFIX, " ");
       }

       if (options.taskListItems) {
       dataSet.set(TaskListExtension.ITEM_DONE_MARKER,
      ...
      ");
       dataSet
      ...
      set(TaskListExtension.ITEM_NOT_DONE_MARKER, " O ");
       }

       dataSet.set(HtmlRenderer.RENDER_HEADER_ID, true);

       if (!options.wikiLinks) {
       dataSet.set(WikiLinkExtension.DISABLE_RENDERING, true);
       }
       } else if (purpose == ForUseBy.SWING) {
       // set rendering options for Swing
       dataSet.set(HtmlRenderer.INDENT_SIZE, 2);
       dataSet.set(Parser.LISTS_LOOSE_ON_PREV_LOOSE_ITEM, true);

       if (options.tables && options.renderTablesGFM) {
       dataSet.set(TablesExtension.COLUMN_SPANS, false)
       .set(TablesExtension.MIN_HEADER_ROWS, 1)
       .set(TablesExtension.MAX_HEADER_ROWS, 1)
       .set(TablesExtension.APPEND_MISSING_COLUMNS, true)
       .set(TablesExtension.DISCARD_EXTRA_COLUMNS, true)
       .set(TablesExtension.HEADER_SEPARATOR_COLUMN_MATCH, true);
       }

       if (options.fencedCode) {
       dataSet.set(HtmlRenderer.FENCED_CODE_
      ...
      _CLASS_PREFIX, "");
       }

       if (options.anchorLinks) {
       dataSet.set(AnchorLinkExtension.ANCHORLINKS_SET_
      ...
      , false);
       dataSet.set(AnchorLinkExtension.ANCHORLINKS_ANCHOR_CLASS, "");
       dataSet.set(AnchorLinkExtension.ANCHORLINKS_SET_NAME, true);
       dataSet.set(AnchorLinkExtension.ANCHORLINKS_TEXT_PREFIX, "");
       }

       if (options.taskListItems) {
       dataSet.set(TaskListExtension
      ...
      ER, "");
       dataSet.set(
      ...
      _NOT_
      ...
      , "");
       }

       dataSet.set
      ...
      , true);

       if
      ...
      options.wiki
      ...
      dataSet.set(WikiLinkExtension.DISABLE_
      ...
      , true);
       }
       } else if (purpose == ForUseBy.HTML) {
       // set rendering
      ...
      dataSet.set(HtmlRenderer.INDENT_
      ...
      , 2);
       dataSet.
      ...
      (Parser.LIST
      ...
      _ON_PRE
      ...
      _LOOSE_ITEM, false);

       if (
      ...
      ) {
      ...
      }
      ...
      1)
      ...
      Extension.APPEND
      ...
      _COLUMNS, true)
      ...
      Extension.HEADER_SEPARATOR_COLUMN_MATCH, true);
       }

       data
      ...
      set(HtmlRenderer
      ...
      HEADER_ID
      ...
      );
       dataSet
      ...
      set(Html
      ...
      _ID, true);

       // set flexmark example spec rendering
       dataSet.set(SpecExampleExtension.SPEC_EXAMPLE_RENDER_AS, RenderAs.FENCED_CODE);
       dataSet.set(SpecExampleExtension.SPEC_EXAMPLE_RENDER_RAW_HTML, false);
       }

       dataSet.set(Parser.EXTENSIONS, extensions);

       return dataSet;
       }

       public static void main(String[] args) {
       // write your code here
       Options options = new Options();
       DataHolder dataHolder = options(ForUseBy.HTML, options);
       IParse parser = Parser.builder(dataHolder).build();
       IRender render = HtmlRenderer.builder(dataHolder).build();

       Node document = parser.parse("\n" +
       "# Heading\n" +
       "\n" +
       "Paragraph Text\n" +
       "with lazy continuation\n" +
       "\n" +
       "---\n" +
       "\n");

       String html = render.render(document);
       System.out.println(html);
       }
      }

      ---

      Title: Working with Parsers and ASTs
      URL: https://brunoscheufler.com/2024-01-07-working-with-parsers-and-asts/
      Published: 2024-01-07T18:00:00.000Z
      Author: N/A
      Highlights:
      For the last couple of days, I pushed through the tedious task of setting up a Markdown parser and formatter in Java consistent with our existing system written in TypeScript. For this first iteration, I decided to customize flexmark-java, a popular Markdown parser library for Java based on the commonmark-java package. For CodeTrail, we store internal documentation as Markdown as part of your codebase. For the biggest part, this is plain old GitHub-flavored Markdown, but we’ve added some custom syntax for our features.
      ...
      To implement custom parsing and formatting behavior for our custom Markdown blocks, I systematically debugged the parser at every step, figuring out the internal structure and approach used to turn a Markdown document string into an AST data structure. The purpose of this task wasn’t to fully understand the codebase but to find potential extension points to use.
      ...
      Fundamentally, we write tests that start with a source Markdown string, which gets parsed by our customized parser. The resulting AST is checked for correctness, then we run it through a formatter to produce another Markdown string. This is compared to the initial document and should be a match. This end-to-end flow ensures that the entire pipeline works as expected and leads to consistent outputs. We wouldn’t want users to receive slightly different Markdown files every time they saved a document without changing anything.
      ...
      In addition to setting up a solid testing workflow and using the debugger, using the decompiled classes isn’t a great help. Go and modern JavaScript codebases allow to vendor or “fork” dependencies effortlessly. Unfortunately, I haven’t found a well-documented way to achieve this in Java, so I forked the entire parser for the time being. This way, we get direct access to the Java source and can modify anything we need. This may not be the optimal setup for dependencies with frequent updates, but thankfully, parsers aren’t really changing a lot.
      ...
      With the Java implementation, we now have a grand total of two aligned Markdown parsers. While they may be consistent for the time being, for any new feature or fixed bug we’ll have to maintain parity. This requires a certain effort we should weigh against alternative implementations.
      ...
      The biggest benefit of a simple Java implementation is that it’ll run anywhere our remaining Java code already runs (portability). It doesn’t require any downloads, platform-specific code, or other complexity. It’s just Java code.
      ...
      A hard requirement is to run everything on the developer’s machine. This means we cannot simply host our parser and send requests via HTTP. In an ideal world, the parser could be embedded in our Java application to ensure full portability without any additional required dependencies.
      ...
      Since we already maintain a reference Markdown parser implementation in TypeScript, which is used by multiple components in the system, it makes sense to reuse that code for our Java codebase.
      ...
      Unfortunately, due to several dependencies and JavaScript ecosystem quirks it’s not as straightforward as transpiling our JavaScript source to Java. Instead, we can instruct our Java code to invoke a customized entry point for the JavaScript Markdown parser.
      ...
      This entry point could be a simple CLI. In this case, we’d bundle all parser-related code into one file and make it executable. We don’t really want to force our users to install Node.js on their system, so we’d have to use a tool like`bun build --compile`,`deno compile`, pkg, or nexe to create a single executable including the JavaScript runtime. These files are typically in the range of 100MBs that need to be delivered to our users.
      ...
      Alternatively, we could use an embedded JavaScript engine. Java bindings for V8 have existed for some time now and communication is straightforward thanks to the built-in serialization features.

      ---

      Title: flexmark-java - Fedora Packages
      URL: https://packages.fedoraproject.org/pkgs/flexmark-java/flexmark-java
      Published: N/A
      Author: N/A
      Highlights:
      CommonMark/Markdown Java parser with source level AST
      ...
      Flexmark-java is a Java CommonMark (spec 0.28) parser using the blocks first, inlines after Markdown parsing architecture. Its strengths are speed, flexibility, Markdown source element based AST with details of the source position down to individual characters of lexemes that make up the element and extensibility. The API allows granular control of the parsing process and is optimized for parsing with a large number of installed extensions. The parser and extensions come with plenty of options for parser behavior and HTML rendering variations. The end goal is to have the parser and renderer be able to mimic other parsers with great degree of accuracy. This is now partially complete with the implementation of Markdown Processor Emulation. This package contains the core library for parsing markdown and rendering to HTML.

      ---

      Title: Testing · vsch/flexmark-java Wiki · GitHub
      URL: https://github.com/vsch/flexmark-java/wiki/Testing
      Published: N/A
      Author: vsch
      Highlights:
      The commonmark spec.txt file is an excellent format which provides the narrative description, source and generated HTML. It is a format against which to run parser compliance tests.
      ...
      The format of this file was modified to add AST output to allow testing of the generated AST which is crucial for using this parser for syntax highlighting. All test util classes were modified to handle the original format and the extended format.
      ...
      cases the original format
      ...
      `FullSpecTestCase` regenerates the spec text with the expected HTML and AST replaced by the parser generated results then asserting that this is equal to the original, in addition to running the individual tests. This allows comparing compliance to full spec in one place and in the case of running the tests in JetBrains IDEA, makes it easy to copy generated results to the expected inputs to make creating and updating expected results easier.....
      ...
      If the spec file does not have an AST section then the expected AST will not be generated or validated, nor will it be present in the generated full file result.
      ...
      The first part is the markdown source, the expected HTML is separated by single`.` on the line. Expected AST is added as a third part to the original spec.txt, also separated by a single`.` on the line.
      ...
      The best way to create a test for an extension is to start with a copy of an existing one and modify the markdown source for the extension, deleting the expected HTML and AST text but leaving the`.` separator lines. Running the`RendererSpecTest` derived test will create a full spec file with all section filled in with actual results. These should be validated then copied to the spec file.
      ...
      ```
      import com.vladsch.flexmark.ext.typographic.TypographicExtension;
      import com.vladsch.flexmark.parser.Parser;
      import com.vladsch.flexmark.util.data.DataHolder;
      import com.vladsch.flexmark.util.data.MutableDataSet;

      import java.util.Arrays;
      import java.util.Collections;
      ...
      public class ComboCustomSpecTest extends RendererSpecTest {
       final private static String SPEC_RESOURCE = "/ext_typographic_ast_spec.md";
       final public static @NotNull ResourceLocation RESOURCE_LOCATION = ResourceLocation.of(COMBO_CUSTOM_SPEC_TEST.class, SPEC_RESOURCE);

       final private static DataHolder OPTIONS = new MutableDataSet()
       .set(Parser.EXTENSIONS, Arrays.asList(TypographicExtension.create()))
       .toImmutable();

       final private static Map<String, DataHolder> optionsMap = new HashMap<String, DataHolder>();
       static {
       optionsMap.put("option", new MutableDataSet()
       .set(CustomExtension.USE_CUSTOM_OPTION, true)
       );
       }
       public ComboCustomSpecTest() {
       super(example, optionsMap, OPTIONS);
       }

       @Parameterized.Parameters(name = "{0}")
       public static List<Object[]> data() {
       return getTestData(RESOURCE_LOCATION);
       }

       @Override
       public @NotNull
       ResourceLocation getSpecResourceLocation() {
       return RESOURCE_LOCATION;
       }
      }

      ---

      Title: Markdown Formatter · vsch/flexmark-java Wiki · GitHub
      URL: https://github.com/vsch/flexmark-java/wiki/Markdown-Formatter
      Published: N/A
      Author: vsch
      Highlights:
      Formatter renders the AST as markdown with various formatting options to clean up and make the source consistent. This also comes with an API to allow extensions to provide formatting options and handle rendering of markdown for custom nodes.
      ...
      The`Formatter` class is a renderer that outputs markdown and formats it to specified options. Use it in place of`HtmlRenderer` to get formatted markdown. It can also be used to convert indentations from one`ParserEmulationProfile` to another:
      ...
      public class PegdownToCommonMark {
           final private static DataHolder OPTIONS = PegdownOptionsAdapter.flexmarkOptions(
                  Extensions.ALL
          );

          static final MutableDataSet FORMAT_OPTIONS = new MutableDataSet();
          static {
              // copy extensions from Pegdown compatible to Formatting, but leave the rest default
              FORMAT_OPTIONS.set(Parser.EXTENSIONS, Parser.EXTENSIONS.get(OPTIONS));
          }

          static final Parser PARSER = Parser.builder(OPTIONS).build();
          static final Formatter RENDERER = Formatter.builder(FORMAT_OPTIONS).build();

          // use the PARSER to parse pegdown indentation rules and RENDERER to render CommonMark

      }
      ...
      These are options available in the`Formatter` class. Extensions which handle formatting of their custom node can and do provide their own formatting options.
      ...
      `FORMATTER_EMULATION_PROFILE`: default`Parser.PARSER_EMULATION_PROFILE`, emulation profile to use for formatting. Can be used to change indenting rules from the ones used by the parser.
      ...
      `INDENTED_CODE_MINIMIZE_INDENT`: default`true`, when true will remove extra indent common to all content lines
      ...
      `LIST_ADD_BLANK_LINE_BEFORE`: default`false`, when`true` will add a blank line before the first list item if it follows a paragraph
      ...
      `SKIP_FENCED_CODE`, default`false`. When`true` will convert fenced code to indented code in generated markdown.
      ...
      Best source of sample code is existing extensions that implement custom node formatting and the formatter module itself:
      ...
      - `flexmark-ext-abbreviation`
      - `flexmark-ext-definition`
      - `flexmark-ext-footnotes`
      - `flexmark-ext-jekyll-front-matter`
      - `flexmark-ext-tables`
      - `flexmark-formatter`

      ---

      Title: Should there be additional information in the AST? - Spec - CommonMark Discussion
      URL: https://talk.commonmark.org/t/should-there-be-additional-information-in-the-ast/2138
      Published: 2016-06-28T10:30:44.000Z
      Author: N/A
      Highlights:
      Supporting both of these features requires only accurate source maps, not additional information in the AST. Source maps are a separate issue, I think…clearly they are desirable, it’s just a matter of designing your parser to produce one. (I didn’t get this right in the current reference implementations, which don’t store source position for inline elements.)
      ...
      An AST is abstract – in general it represents only the essential structure of the program or document, abstracting from syntactic details in which this is presented. For example, some ASTs omit source code comments altogether. Some ASTs omit details like whether a string was represented with single quotes or double quotes, in languages where both are permitted. So it’s up to us to decide which features must be represented in an AST. It is generally not expected or required that an AST contain enough information to reproduce the source in every detail. If there is a reason to require this in this case, it needs to be articulated and defended.
      ...
      of the AST was to provide a representation of a structured document that could be rendered into a number of different formats, preserving semantics. The current AST meets that goal.
      ...
      What I think now is that the spec
      ...
      ’t say anything about it. The spec should describe the structure that a conforming parser needs to preserve. This structure won’t include, e.g., a distinction between`+` bulleted lists and`-` bulleted lists. But nothing stops you from preserving this structure if you want to.
      ...
      While there is already a parsing strategy as an appendix, I would not mind much having an appendix that describes what kind of information you could store in an AST+source map, but it is highly dependent on the uses cases (you barely need the original semantic+source span for a html only converter, but for a syntax highlighter you definitely need them) so not sure it should be part of a 1.0 spec (just a nice to have)… This could be added later without any impact on the core specs
      ...
      I give my vote to@vitaly: the AST should be lossless but would add there is a lot more to it than that. The AST would have to reflect the elements in the source and not only ones needed for rendering.
      ...
      with a source map. What would the source map look like
      ...
      a reference definition which has no rendering element at all? It may not even be referenced in
      ...
      document but is needed for syntax
      ...
      , completion assistance
      ...
      error annotations.
      ...
      I am the author of a Markdown plugin for JetBrains IDEs with syntax highlighting, completions, annotations and formatting, Markdown Navigator. I learned the hard way that having a lossless AST is not an option but a requirement. For my purposes I forked commonmark-java and made a version, flexmark-java, with a lossless AST that reflects the exact markdown elements in the source and a boat load of options that can tweak the parsing rules. I also included the AST as a testable part of the spec. Lots of bugs in the plugin came from the AST not being what was expected or from missing or incorrect source position information for some part of an element.
      ...
      Without a lossless AST, whoever needs the source representation will have no choice but to at least partially re-parse the source to complete what is missing. Double parsing is a nightmare to implement, test and maintain. I did that when pegdown was the parser used by the plugin and would not want to relive the experience.
      ...
      an inside`-` or`*` or`
      ...
      ` being wrapped to the first character of the line. Now
      ...
      list where none was
      ...
      . Again, trying to catch all this in the editor
      ...
      adding a \ to all such
      ...
      wrap to the
      ...
      . What about block
      ...
      , atx headings,
      ...
      these issues were swept aside
      ...
      edge cases. I
      ...
      edly disagree.
      ...
      ambiguity was approximately right, commonmark spec is choosing
      ...
      be precisely wrong. For example setext headers
      ...
      having minimum of 3`-`, lists not needing a blank line before
      ...
      paragraph, list indentation rules that can result
      ...
      a single list with items indented
      ...
      a few examples.
      ...
      ### Parsing and AST for User Assisting Editor
      ...
      Not only do you want an element parsed and in the AST when it does not render, you also want illegal elements that would be legal with minor changes. For example a link reference without the reference being defined should be in the AST. It is not unusual to create the ref link then define the reference. If the ref link is parsed as text until its reference is defined, then the editor/plugin cannot help the user with an error highlight of the missing reference, nor help him get the reference label right, by using the undefined ref link and ref image references as a list of suggestions for the reference label.
      ...
      Case 2. is a reference link with a reference definition and the user gets different options for the two
      ...
      - option to inline a link
      - validation that the reference is defined
      - validation that the reference is a valid type, as for image references to non-image files
      - completions between`[]` give a list with all the references defined in the document
      - If an inline link after this element is converted to a reference then an empty dummy reference`[]` will be inserted automatically after this element to prevent it from becoming the reference text of the new reference link.
      ...
      as for an inline
      ...
      when a reference definition is
      ...
      being referenced in the document
      ...
      - completion between`
      ...
      show any reference links whose reference is not defined. In other words, if
      ...
      create reference links first then create
      ...
      references you are aided
      ...
      getting the reference text right.
      ...
      Thanks for all this feedback. The spec and reference implementations were certainly designed with rendering in mind, not on-the-fly highlighting, and you’re right that the latter task imposes different requirements. For now, I don’t think the spec should say anything in particular about AST elements; we’ll leave this up to implementations.
      ...
      Let me just say that a lot of thought has gone into the existing rules. There are strong reasons for choosing the current list indentation rules over, say, a one-space indent or three-space indent or four-space indent rule (see section 5.2.1 in the spec for explanation). And there is a strong reason for not requiring blank lines before lists. (This is also articulated in the spec, see after Example 264.) We have recently mitigated the problem of unwanted lists from hard-wrapping by adding the rule that an ordered list item can interrupt a list only when it starts with`1`. You can still get unwanted lists if you hard-wrap a paragraph containing a`-`,`*`, or`+` with space on both sides, but these cases should be pretty rare. If we were designing something afresh, rather than trying to give a rational formulation to an existing markup, I would have preferred the reStructuredText approach, which requires blank lines before lists including sublists, but I think going this way is ruled out by an interest in maintaining backwards compatibility.
```
/answer 4ba4efe4-c1bb-4df0-b9ee-8f908791736f allow
User:

Agent:
Control Request:
```yaml
id: a5c15431-0000-4bc2-91ba-45a548316969
toolName: web-search-exa
arguments:
  query: Eclipse Mylyn WikiText markdown AST SWT StyledText editor incremental parsing large files
  reason: Find SWT-based markdown editor implementations with AST support
```
/answer a5c15431-0000-4bc2-91ba-45a548316969 allow
User:

Agent:
Control Request:
```yaml
id: bdb7347e-ce5e-48c1-a84e-14ed7c003bce
result:
  structuredContent:
    content: |-
      Title: WikiText-Developer-Guide
      URL: https://help.eclipse.org/latest/topic/org.eclipse.mylyn.wikitext.help.ui/help/devguide/WikiText-Developer-Guide.html
      Published: N/A
      Author: N/A
      Highlights:
      The markup parser uses regular expressions to parse wiki markup, converting the markup to XHTML or other outputs depending on the configured`DocumentBuilder`. See also Markup Language Concepts
      ...
      The WikiText JFace viewer extends the standard JFace source viewer. The WikiText JFace viewer uses the markup parser to convert the text to XHTML. It then parses the XHTML and converts it to a standard unicode character sequence and a TextPresentation. It uses standard JFace techniques to apply styles to the rendered text and uses unicode character features such as newlines, tabs and bullet characters to make the text look as it is intended.
      ...
      The following limitations apply to the JFace text-only markup rendering viewer. The limitations can be avoided by using the SWT Browser widget instead.
      ...
      WikiText use within an Eclipse runtime is simple: simply add the desired WikiText bundles as dependencies to your plug-in project. This is the recommended approach for Eclipse plug-ins and for RCP applications.
      ...
      Stand-alone usage of WikiText is also possible. To use WikiText outside of an Eclipse runtime, simply add the WikiText jar files to your classpath. Note that stand-alone usage of WikiText is limited to the use of the markup parser framework and Ant tasks.
      ...
      Add the`org.eclipse.mylyn.wikitext*.jar` to your classpath, and at least one the following language-specific jars:
      ...
      - `org.eclipse.mylyn.wikitext.asciidoc*.jar`
      - `org.eclipse.mylyn.wikitext.confluence*.jar`
      - `org.eclipse.mylyn.wikitext.markdown*.jar`
      - `org.eclipse.mylyn.wikitext.mediawiki*.jar`
      - `org.eclipse.mylyn.wikitext.textile*.jar`
      - `org.eclipse.mylyn.wikitext.tracwiki*.jar`
      - `org.eclipse.mylyn.wikitext.twiki*.jar`

      ---

      Title: Mylyn-WikiText-User-Guide - Getting Started
      URL: https://help.eclipse.org/latest/topic/org.eclipse.mylyn.wikitext.help.ui/help/Getting-Started.html
      Published: N/A
      Author: N/A
      Highlights:
      The WikiText plug-in provides a lightweight markup editor for Eclipse. The editor is registered against all files with the following file extensions:`*.ad`,`*.adoc`,`*.asciidoc`,`*.textile`,`*.tracwiki`,`*.markdown`,`*.md`,`*.mdtext`,`*.mediawiki`,`*.twiki`,`*.confluence`
      ...
      The WikiText editor provides a source view, preview and outline.
      ...
      ‘Source’ is the default editor pane. This is the area for editing markup such as Textile. The source pane provides syntax highlighting that should make it easier to see what the markup means.
      ...
      Standard text editor actions are available here, such as copy/paste and find/replace. A "Preview at [heading]" context menu is provided to open the preview tab at a specific section of the document. Explore the context menu and ‘Edit’ menu to see what actions are available.
      ...
      The editor preview provides a preview of the wiki markup as it is rendered by your default browser after converting the markup to HTML. Though the ‘Source’ syntax highlighting is pretty good, the preview provides a more accurate view of the rendered result.
      ...
      The WikiText editor supports multiple markup languages. The editor makes a best-guess at the markup language from the file extension. To switch markup languages in the editor invoke the context menu Markup Languages and select the language:
      ...
      Selecting Enable validation causes WikiText to validate wiki markup files in your project. This is done as part of the project build process, so it helps to have automatic building enabled (Preferences->Workspace->Build Automatically). Validation is performed on all resources that match a wiki markup file extension. In addition validation includes any files for which the markup language setting was set even if the file doesn’t have a registered wiki markup file extension.

      ---

      Title: Mylyn/WikiText - Eclipsepedia
      URL: https://wiki.eclipse.org/Mylyn/WikiText/
      Published: 2023-09-04T00:00:00.000Z
      Author: N/A
      Highlights:
      Mylyn WikiText provides an extensible framework and tools for parsing, editing and presenting lightweight markup, including:
      ...
      - API a framework and API for handling lightweight markup
      - Ant Tasks for converting markup to other formats
      - Mylyn task editor integration making the task editor markup-aware for displaying and editing markup
      - Editor for wiki files using a source editor paradigm with preview
      - 1.3 Feedback
      ...
      WikiText has parsers for MediaWiki, Textile, Markdown, CommonMark, AsciiDoc, Confluence, Creole, TracWiki and TWiki markup, and can be extended to support other languages. WikiText provides Ant tasks for converting lightweight markup to HTML, Eclipse Help, DocBook, DITA and XSL-FO. WikiText also provides an editor for editing such markup within Eclipse, and integrates with the Mylyn task editor causing it to be markup-aware. WikiText provides API for integrating wiki markup capabilities into Eclipse, RCP, stand-alone and server-side applications.

      ---

      Title: WikiText-Developer-Guide - Using The WikiText Parser
      URL: https://help.eclipse.org/latest/topic/org.eclipse.mylyn.wikitext.help.ui/help/devguide/Using-The-WikiText-Parser.html
      Published: N/A
      Author: N/A
      Highlights:
      WikiText provides a parser framework for parsing lightweight markup languages (wiki markup). The parser framework can output HTML, DITA, DocBook, and Eclipse Help directly, or integrators can extend`DocumentBuilder` to output other formats.
      ...
      Your code may need to handle multiple markup languages or you may wish to eliminate compile-time dependencies on a specific markup language. The`ServiceLocator` can obtain markup languages by name:
      ...
      Other markup languages can be obtained by changing`"Textile"` to another markup language name. Valid markup language names include:
      ...
      - AsciiDoc
      - Confluence
      - Markdown
      - MediaWiki
      - Textile
      - TracWiki
      - TWiki
      ...
      The`MarkupParser` emits content to a`DocumentBuilder` in order to create HTML or other output. To control the output more closely instantiate the`DocumentBuilder` of choice and configure it before it is used.
      ...
      ```
      StringWriter writer = new StringWriter();

      HtmlDocumentBuilder builder = new HtmlDocumentBuilder(writer);
      // avoid the <html> and <body> tags
      builder.setEmitAsDocument(false);

      MarkupParser parser = new MarkupParser(new TextileLanguage());
      parser.setBuilder(builder);
      parser.parse(markupContent);

      String htmlContent = writer.toString();
      ...
      CSS stylesheets can be embedded directly or referenced within your HTML:
      ...
      builder.addCssStylesheet("styles/test.css");

      // Add a
      ...
      /css">
      ...
      builder.addCssStylesheet(new File("/
      ...
      /file.css"));

      ---

      Title: Mylyn FAQ - WikiText
      URL: https://help.eclipse.org/latest/topic/org.eclipse.mylyn.help.ui/Mylyn/FAQ/WikiText.html
      Published: N/A
      Author: N/A
      Highlights:
      WikiText is a set of plug-ins for Eclipse that provide lightweight markup (wiki) parsing, editing and display capabilities to the Eclipse platform and Mylyn. WikiText provides a parser for wiki markup and converts the markup to HTML, Docbook, DITA, or Eclipse Help format, either via the API or by using Ant tasks. WikiText also provides UI components (such as an editor) integrating with Eclipse and the Mylyn task editor.
      ...
      WikiText extends the Mylyn task editor to be markup-aware. Comments and description text is formatted according to the configured markup language. The description and comment editors are aware of markup and provide content assist, markup help and preview.
      ...
      WikiText can create HTML, Eclipse Help, DITA, DocBook and XSL-FO from wiki markup. Using the WikiText APIs you can also extend WikiText to create other output formats. DITA, DocBook and XSL-FO can all be used to create PDF. More information is available in the WikiText User Guide.
      ...
      WikiText can parse the following markup languages:
      ...
      - Confluence
      - MediaWiki
      - Markdown
      - Textile
      - TracWiki
      - TWiki
      ...
      The preview tab is not shown if the SWT browser is not configured correctly. See The SWT FAQ for details.

      ---

      Title: org.eclipse.mylyn.wikitext.markdown.core/src/org/eclipse/mylyn/wikitext/markdown/core/MarkdownLanguage.java - mylyn/org.eclipse.mylyn.docs - Git at Google
      URL: https://eclipse.googlesource.com/mylyn/org.eclipse.mylyn.docs/+/176a2ae17c5b04d07c16dfe061420c44678edc4e/org.eclipse.mylyn.wikitext.markdown.core/src/org/eclipse/mylyn/wikitext/markdown/core/MarkdownLanguage.java
      Published: N/A
      Author: N/A
      Highlights:
      | import org.eclipse.mylyn.wikitext.core.parser.markup.AbstractMarkupLanguage; |
      ...
      .eclipse.mylyn.wik
      ...
      parser.markup.Block; |
      ...
      | /** |
      | * A markup language implementing Markdown syntax. http://daringfireball.net/projects/markdown/syntax |
      | * |
      | * @author Stefan Seelmann |
      | * @since 1.8 |
      | */ |
      | public class MarkdownLanguage extends AbstractMarkupLanguage { |
      | public MarkdownLanguage() { |
      | setName("Markdown"); //$NON-NLS-1$ |
      | } |
      ...
      | @Override |
      | protected void addStandardTokens(PatternBasedSyntax tokenSyntax) { |
      | // HTML entities are preserved |
      | tokenSyntax.add(new PreserverHtmlEntityToken()); |
      | // two or more spaces at end of line force a line break |
      | tokenSyntax.add(new PatternLineBreakReplacementToken("( {2,})$")); //$NON-NLS-1$ |
      | } |
      ...
      | @Override |
      | protected void addStandardPhraseModifiers(PatternBasedSyntax phraseModifierSyntax) { |
      ...
      | // inline HTML |
      | phraseModifierSyntax.add(new HtmlEndTagPhraseModifier()); |
      | phraseModifierSyntax.add(new HtmlStartTagPhraseModifier()); |
      ...
      | @Override |
      | protected void addStandardBlocks(List blocks, List paragraphBreakingBlocks) { |
      | blocks.add(new CodeBlock()); |
      | blocks.add(new HorizontalRuleBlock()); |
      | blocks.add(new HeadingBlock()); |
      | blocks.add(new InlineHtmlBlock()); |
      | blocks.add(new QuoteBlock()); |
      | } |
      ...
      | @Override |
      | protected Block createParagraphBlock() { |
      | ParagraphBlock paragraphBlock = new ParagraphBlock(); |
      | UnderlinedHeadingBlock headingBlock = new UnderlinedHeadingBlock(); |
      | ReadAheadDispatcher readAheadBlock = new ReadAheadDispatcher(headingBlock, paragraphBlock); |
      | return readAheadBlock; |
      | } |
      | } |

      ---

      Title: WikiText-Developer-Guide - Markup Languages
      URL: https://help.eclipse.org/latest/topic/org.eclipse.mylyn.wikitext.help.ui/help/devguide/Markup-Languages.html
      Published: N/A
      Author: N/A
      Highlights:
      Markup languages are the core concept that WikiText uses to define a parser for specific wiki markup. WikiText provides facilities for adding new markup languages or extending an existing one. All markup languages in WikiText extend the`org.eclipse.mylyn.wikitext.parser.markup.MarkupLanguage` class.
      ...
      You may wish to augment an existing markup language syntax with your own extensions. With WikiText this is possible by subclassing an existing markup language.
      ...
      MarkupLanguages that extend others may declare this hierarchy in the`org.eclipse.mylyn.wikitext.ui.markupLanguage` extension point by using the`extends` attribute. Declaring the hierarchy is optional, and allows for the markup language to inherit code completion, validation and help content from the markup language being extended.
      ...
      WikiText is designed to be extended to support new markup languages. To add a markup language take the following steps:
      ...
      If it’s to be used in Eclipse, register your markup language using the`org.eclipse.mylyn.wikitext.ui.markupLanguage` extension point. Other extension points that may be of interest to you:
      ...
      - `org.eclipse.mylyn.wikitext.ui.markupValidationRule` provide markup validation to detect common problems in markup
      - `org.eclipse.team.core.fileTypes` ensure that team providers know that your file type is text
      - `org.eclipse.core.contenttype.contentTypes` declare your markup language content type
      ...
      You’re most likely to be successful if you use one of the existing WikiText markup language plug-ins as an example. A good starting point is the`org.eclipse.mylyn.wikitext.textile` plug-in.
      ...
      To have a full featured UI for your markup language in Eclipse there are several additional extension points to be aware of:
      ...
      - `org.eclipse.mylyn.tasks.ui.taskEditorExtensions` make your markup language contribute to the Mylyn task editor
      - `org.eclipse.mylyn.wikitext.ui.contentAssist` a means of having content-assist for your markup language
      - `org.eclipse.mylyn.wikitext.ui.cheatSheet` a way of associating help content for your markup language
      ...
      For more information on these and other UI functions, take a look at the`org.eclipse.mylyn.wikitext.textile.ui` plug-in.
      ...
      Every`MarkupLanguage` declares its syntax in terms of blocks, phrases, and replacement tokens. Though it is possible to create a markup language implementation that doesn’t use these concepts, these are the building blocks of all markup languages implemented within WikiText.
      ...
      1. Feel free to post questions to the Mylyn newsgroup.
      2. If in doubt, always use an existing WikiText markup language as an example.
      3. Make extensive use of brief test cases that test for markup language syntax. WikiText has over 430 JUnit tests which we’ve found to be invaluable in verifying the expected behavior. Take a look at`TextileLanguageTest` for an example of how to write tests for your markup language.
      ...
      4. When implementing a markup language, always ensure that there’s one ‘catch-all’ block, usually the paragraph block. It should be last in the list of blocks declared by your language.
      ...
      5. The order that blocks, phrases and replacement tokens are declared by your markup language affect the markup syntax.

      ---

      Title: Mylyn-WikiText-User-Guide - Task Editor Integration
      URL: https://help.eclipse.org/latest/topic/org.eclipse.mylyn.wikitext.help.ui/help/Task-Editor-Integration.html
      Published: N/A
      Author: N/A
      Highlights:
      WikiText extends Mylyn to provide a markup-aware task editor. With WikiText installed, Mylyn can render wiki markup as intended, provide markup-specific syntax highlighting, content-assist, validation, and a cheat-sheet for wiki markup syntax.
      ...
      To use the WikiText extension to the Mylyn task editor you may need to configure your Mylyn task repository. To do so, open the Mylyn Task Repositories view (Window -> Show View -> Other... -> Mylyn -> Task Repositories). Select the repository that you wish to configure and then select Properties from the context menu.
      ...
      In the Editor section select the markup language of choice. Note that you may need to expand this section to see available choices. To disable WikiText extensions to the Mylyn task editor select Plain Text.
      ...
      The appearance of rendered markup in the task editor can be altered in the Eclipse preferences. Open Preferences -> General -> Editors -> Text Editors -> WikiText -> Appearance and alter the appearance using CSS styles. See Preferences for more details.
      ...
      We recommend using markup with task repositories such as Bugzilla. Markup makes content more readable and Mylyn can make it look good within Eclipse.
      ...
      It should be noted that some markup languages such as WikiMedia and Textile were originally designed for wikis, not bug reports or task descriptions. Some markup language constructs of these languages are not suitable for use with task repositories, and are altered by WikiText when used with the Mylyn task editor. Below is a list of language features are altered when used with the Mylyn task editor:
      ...
      formatted text where
      ...
      a space character has
      ...
      where it was not
      ...
      source code into bug comments
      ...
      - Support for HTML tags has been disabled to allow for pasting HTML source code into bug comments and descriptions.
      - Footnote references are preprocessed and only matched if a corresponding footnote exists in the content.
      ...
      The following language constructs are enabled for all markup languages:
      ...
      - Java stack trace detection
      - Eclipse-specific: content following a line starting with -- Error Details --

      ---

      Title: org.eclipse.mylyn.wikitext.markdown 4.7.0.v20250326-1000
      URL: https://download.eclipse.org/staging/2025-06/buildInfo/archive/download.eclipse.org/staging/2025-06/index/org.eclipse.mylyn.wikitext.markdown_4.7.0.v20250326-1000.html
      Published: N/A
      Author: N/A
      Highlights:
      org.eclipse.mylyn.wikitext.markdown 4.7.0.v20250326-1000
      ...
      ## Mylyn WikiText Markdown org.eclipse.mylyn.wikitext.markdown4.7.0.v20250326-1000
      ...
      Eclipse Mylyn
      ...
      ### Content Metadata ▷

       org.eclipse.mylyn.wikitext.markdown (org.eclipse.update.install.sources=true) Bundle-SymbolicName: org.eclipse.mylyn.wikitext.markdown Bundle-Version: 4.7.0.v20250326-1000

      ---

      Title: org.eclipse.mylyn.wikitext.feature.source.feature.jar 4.7.0.v20250513-1805
      URL: https://download.eclipse.org/staging/2025-06/buildInfo/archive/download.eclipse.org/staging/2025-06/index/org.eclipse.mylyn.wikitext.feature.source.feature.jar_4.7.0.v20250513-1805.html
      Published: N/A
      Author: N/A
      Highlights:
      Provides an editor for lightweight markup (wiki text) files supporting AsciiDoc, Confluence, Markdown, MediaWiki, Textile, TracWiki and TWiki. Extends the Mylyn task editor to create a markup-aware editor.
```
/answer bdb7347e-ce5e-48c1-a84e-14ed7c003bce allow
User:

Agent:
Ja, es gibt gleich mehrere brauchbare Java-Implementierungen – die Frage ist, welche am besten zu deinem Anwendungsfall (großer Files + präzises Region-Tracking für Spellcheck/Highlighting in SWT) passt.

## 1. **commonmark-java** – solide Basis

Der "offizielle" Java-Port des CommonMark-Referenzparsers.

- Reiner CommonMark-Standard + optionale Extensions (Tables, Strikethrough, Autolink, Footnotes, …)
- Seit einiger Zeit mit **Source-Span-Tracking**:
```java
Parser parser = Parser.builder()
    .includeSourceSpans(IncludeSourceSpans.BLOCKS_AND_INLINES)
    .build();
Node doc = parser.parse(source);
SourceSpan span = someNode.getSourceSpans().get(0);
span.getLineIndex();   // Zeile
span.getColumnIndex(); // Spalte
span.getInputIndex();  // Absolute Offset
span.getLength();
```
- **Nachteil**: Source-Spans sind pro Node meist nur Start/Ende, nicht auf Lexem-Ebene für jedes Detail (z. B. die Sternchen einer `**bold**`-Markierung separat).
- Klein, schnell, gut gepflegt, viele Extensions als separate Artefakte – für dich vermutlich der pragmatischste Einstieg.

## 2. **flexmark-java** – die "Rolls-Royce"-Variante für Editoren

Ein Fork von commonmark-java, der **explizit für Editor-Anwendungsfälle** (JetBrains "Markdown Navigator" Plugin) entwickelt wurde – genau dein Szenario.

Warum das für dich interessant ist:
- **Lossless AST**: Jedes Zeichen des Quelltexts ist einem Knoten zugeordnet, auch syntaktisch "unwichtige" Teile (Sternchen, Leerzeichen, ungültige Referenzen etc.) – wichtig für Syntax-Highlighting, wo du auch Markup-Zeichen einfärben willst.
- **Source-Position bis auf Lexem-Ebene**, nicht nur Node-Start/Ende – z. B. bekommst du für `**bold**` separate Segmente für die öffnenden/schließenden Sternchen und den Textinhalt.
- Sehr granulare Extensions (Tables, Task-Lists, Wiki-Links, Footnotes, Fenced Code, Jekyll-Frontmatter, Typographic, …)
- `BasedSequence` – ein spezielles CharSequence, das Teilsequenzen mit Rückverweis auf die Originalposition erlaubt → sehr praktisch, wenn du inkrementell auf Regionen im Originaltext zugreifen willst, ohne den ganzen Text zu kopieren.
- Wird explizit auch für JetBrains PSI-Tree-ähnliche Strukturen genutzt – also für "living AST" in einem Editor-Kontext, nicht nur einmaliges Rendering.

Nachteil: Größer, komplexer API, Weiterentwicklung mittlerweile langsamer als bei commonmark-java.

## 3. **Eclipse Mylyn WikiText**

Falls dein SWT-Editor ohnehin im Eclipse/RCP-Ökosystem läuft:
- `org.eclipse.mylyn.wikitext.markdown` – Markdown-Parser als Teil eines größeren Frameworks für Wiki-Markup
- Bringt bereits einen fertigen **JFace-Source-Editor mit Syntax-Highlighting, Outline und Preview** mit (`org.eclipse.mylyn.wikitext.editor`)
- Architektur ist aber eher "Regex-Block/Phrase-Parser → DocumentBuilder", kein klassischer, node-fokussierter AST wie MDAST – für feingranulares Region-Tracking (z. B. jedes einzelne Wort für Spellcheck) weniger geeignet als flexmark.
- Guter Startpunkt, wenn du sowieso in Eclipse-Infrastruktur bist und schnell ein "gutes genug" Ergebnis willst.

## Empfehlung für deinen Use-Case

Für **große Dateien + präzises Region-Tracking** würde ich zu **flexmark-java** raten:

1. **Große Dateien**: `BasedSequence` vermeidet unnötiges String-Kopieren, arbeitet mit Index-Bereichen auf dem Original-Buffer – das ist genau das Pattern, das du für SWT `StyledText`/`IDocument`-Regionen brauchst (Offsets direkt weiterverwendbar für `StyleRange`).
2. **Spellcheck**: Da der AST lossless ist, kannst du gezielt nur `Text`-Node-Bereiche an den Spellchecker geben und Code-Blöcke, URLs, Markup-Zeichen etc. ausschließen.
3. **Syntax-Highlighting**: Die feingranularen Source-Spans (bis zu einzelnen Markup-Zeichen) lassen sich 1:1 in SWT `StyleRange`-Listen übersetzen.
4. **Inkrementelles Parsen**: flexmark selbst macht kein inkrementelles Reparsing, aber durch die granulare Node-Struktur und Sequenz-basierte API kannst du selbst eine Änderungs-Diff-Strategie bauen (z. B. nur betroffene Blöcke neu parsen).

Für sehr große Dateien solltest du zusätzlich überlegen, das Parsing **blockweise/lazy** zu triggern (z. B. nur sichtbaren Viewport + Puffer neu parsen) – das musst du bei allen drei Bibliotheken selbst bauen, keine bietet das "out of the box".

Soll ich dir ein konkretes Beispiel zeigen, wie man flexmark-java mit SWT `StyledText` verbindet (Node → StyleRange-Mapping)?

User:
Hm, klingt beides ungeeignet. Ich denke ich schreibe eine eigene bIbliothek.
Agent:
[xy.ai.req:ClaudeCode:acbec191-94bf-4821-8897-6d922422d54e]
User:
