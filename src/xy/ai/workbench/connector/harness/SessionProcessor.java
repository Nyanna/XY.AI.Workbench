package xy.ai.workbench.connector.harness;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.node.TextNode;
import com.fasterxml.jackson.databind.node.NullNode;

import xy.ai.workbench.EditorInterface;
import xy.ai.workbench.commands.Command;
import xy.ai.workbench.commands.CommandRegistry;
import xy.ai.workbench.connector.claudecode.YamlRenderer;
import xy.ai.workbench.models.AIAnswer;

/**
 * Deterministic, symmetric text &lt;-&gt; message-list translator shared by all
 * connectors. Recursively resolves {@code [include](path)} directives (removed
 * as a line-level separator; the target is parsed independently and spliced in
 * at that point) and turns the resulting markdown into an ordered sequence of
 * {@link SessionCallbacks} invocations.
 *
 * <p>
 * Besides the generic {@code [include](path)} form, a handful of typed include
 * kinds delegate to an injected {@link IIncludeAdapter}, keeping this class
 * free of any host-specific (e.g. Eclipse) retrieval logic:
 * {@code [include contextprompt](dir)}, {@code [include files](selected|dir)},
 * {@code [include file](path)} and {@code [include search](files|matches)}.
 *
 * <p>
 * Recognized markers, each starting a line of their own
 * ({@link EditorInterface#USER}/{@link EditorInterface#AGENT} switch the
 * current role, {@link EditorInterface#THINKING}/{@link EditorInterface#TEXT}
 * start a plain multi-line block, {@link EditorInterface#TOOLUSE}/
 * {@link EditorInterface#TOOLRESULT} are followed by a fenced YAML block).
 * Unmarked paragraphs become a plain {@link SessionCallbacks#message}.
 *
 * <p>
 * A single instance is shared by all connectors (see {@code Activator}); it is
 * stateless besides the injected {@link IIncludeAdapter}.
 */
public class SessionProcessor {

	private static final Pattern INCLUDE_LINE = Pattern.compile("^(\\s*\\[include(?:\\s+\\w+)?\\]\\([^)]+\\)\\s*)+$");
	private static final Pattern INCLUDE_TAG = Pattern.compile("\\[include(?:\\s+(\\w+))?\\]\\(([^)]+)\\)");

	private static final ObjectMapper JSON = new ObjectMapper();
	private final YamlRenderer yaml = new YamlRenderer();
	private IIncludeAdapter adapter;

	public void setAdapter(IIncludeAdapter adapter) {
		this.adapter = adapter;
	}

	/**
	 * Processes each list entry as an independent root document (matches the
	 * {@code inputs} lists connectors build requests from).
	 */
	public <M> List<M> process(List<String> inputs, boolean enabled, SessionCallbacks<M> callbacks) {
		List<M> out = new ArrayList<>();
		if (inputs == null)
			return out;
		if (!enabled) {
			String joined = inputs.stream().filter(i -> i != null && !i.isBlank())
					.collect(java.util.stream.Collectors.joining("\n"));
			if (!joined.isBlank())
				out.add(callbacks.message(Role.User, joined));
			return out;
		}
		for (String input : inputs)
			if (input != null && !input.isBlank())
				new Run<>(callbacks, out).run(input, List.of());
		return out;
	}

	/**
	 * Processes a single root document, optionally anchored at {@code rootPath} for
	 * include-cycle detection against itself.
	 */
	public <M> List<M> process(String input, boolean enabled, Path rootPath, SessionCallbacks<M> callbacks) {
		List<M> out = new ArrayList<>();
		if (!enabled) {
			if (input != null && !input.isBlank())
				out.add(callbacks.message(Role.User, input));
			return out;
		}
		List<Path> chain = rootPath == null ? List.of() : List.of(rootPath.toAbsolutePath().normalize());
		new Run<>(callbacks, out).run(input, chain);
		return out;
	}

	/**
	 * One recursive parse: a stateful pass producing callback-built messages in
	 * document order.
	 */
	private final class Run<M> {
		private final SessionCallbacks<M> callbacks;
		private final List<M> out;
		private Role role = Role.User;
		private StringBuilder buffer = new StringBuilder();

		Run(SessionCallbacks<M> callbacks, List<M> out) {
			this.callbacks = callbacks;
			this.out = out;
		}

		void run(String text, List<Path> chain) {
			String[] lines = text == null ? new String[0] : text.split("\n", -1);
			int i = 0;
			while (i < lines.length) {
				String line = lines[i];
				String stripped = line.strip();

				Command command = CommandRegistry.detect(line);
				if (command != null) {
					switch (command.processorAction()) {
					case IGNORE_BLOCK: {
						flush();
						int[] range = fenceRange(lines, i + 1);
						i = range != null ? range[1] + 1 : i + 1;
						continue;
					}
					case TRANSFORM: {
						flush();
						String rest = command.parameter(2);
						if (rest != null && !rest.isBlank()) {
							if (buffer.length() > 0)
								buffer.append("\n");
							buffer.append(rest);
						}
						i++;
						continue;
					}
					case REMOVE: {
						flush();
						i++;
						continue;
					}
					default:
						break;
					}
				}

				if (INCLUDE_LINE.matcher(line).matches()) {
					flush();
					Matcher m = INCLUDE_TAG.matcher(line);
					while (m.find())
						include(m.group(1), m.group(2).strip(), chain);
					i++;
					continue;
				}
				if (stripped.equals(EditorInterface.USER)) {
					flush();
					role = Role.User;
					i++;
					continue;
				}
				if (stripped.equals(EditorInterface.AGENT)) {
					flush();
					role = Role.Agent;
					i++;
					continue;
				}
				if (stripped.equals(EditorInterface.THINKING)) {
					flush();
					i = consumeReasoning(lines, i + 1);
					continue;
				}
				if (stripped.equals(EditorInterface.TEXT)) {
					flush();
					i = consumeText(lines, i + 1);
					continue;
				}
				if (stripped.equals(EditorInterface.TOOLUSE)) {
					flush();
					i = consumeToolCall(lines, i + 1);
					continue;
				}
				if (stripped.equals(EditorInterface.TOOLRESULT)) {
					flush();
					i = consumeToolResult(lines, i + 1);
					continue;
				}
				if (stripped.startsWith(AIAnswer.RESULT)) {
					flush();
					i++;
					continue;
				}

				if (buffer.length() > 0)
					buffer.append("\n");
				buffer.append(line);
				i++;
			}
			flush();
		}

		private void flush() {
			String text = buffer.toString().strip();
			buffer = new StringBuilder();
			if (!text.isEmpty())
				out.add(callbacks.message(role, text));
		}

		private boolean isMarkerLine(String line) {
			String s = line.strip();
			return INCLUDE_LINE.matcher(line).matches() //
					|| s.equals(EditorInterface.USER) || s.equals(EditorInterface.AGENT) //
					|| s.equals(EditorInterface.THINKING) || s.equals(EditorInterface.TEXT) //
					|| s.equals(EditorInterface.TOOLUSE) || s.equals(EditorInterface.TOOLRESULT) //
					|| CommandRegistry.detect(s) != null;
		}

		/**
		 * A reasoning block is delimited only by its own markers - repeated Thinking:
		 * lines continue it, Thinking Meta: mandatorily ends it - so its end is never
		 * inferred from an unrelated marker.
		 */
		private int consumeReasoning(String[] lines, int start) {
			List<String> texts = new ArrayList<>();
			String meta = null;
			int i = start;
			while (i < lines.length) {
				StringBuilder body = new StringBuilder();
				while (i < lines.length) {
					String stripped = lines[i].strip();
					if (stripped.equals(EditorInterface.THINKING) || stripped.startsWith(EditorInterface.THINKING_META))
						break;
					if (body.length() > 0)
						body.append("\n");
					body.append(lines[i]);
					i++;
				}
				texts.add(body.toString().strip());
				if (i >= lines.length)
					break; // malformed: missing the mandatory Thinking Meta terminator
				String stripped = lines[i].strip();
				i++;
				if (stripped.startsWith(EditorInterface.THINKING_META)) {
					meta = stripped.substring(EditorInterface.THINKING_META.length()).strip();
					break;
				}
				// else: another Thinking: line starts the next text entry
			}
			if (meta != null)
				meta = substituteReasoningPlaceholders(meta, texts);
			boolean hasText = texts.stream().anyMatch(t -> !t.isEmpty());
			if (hasText || meta != null)
				out.add(callbacks.reasoning(role, texts, meta));
			return i;
		}

		/**
		 * Parses {@code meta} as JSON, walks it generically (no schema knowledge
		 * needed) and replaces every string node whose value exactly matches
		 * {@link SessionRenderer#reasoningPlaceholder(int)} with the corresponding
		 * entry of {@code texts}, then serializes the result back to a string.
		 */
		private String substituteReasoningPlaceholders(String meta, List<String> texts) {
			try {
				JsonNode node = JSON.readTree(meta);
				substituteInPlace(node, texts);
				return node.toString();
			} catch (Exception e) {
				throw new IllegalStateException("Invalid Thinking Meta JSON: " + e.getMessage(), e);
			}
		}

		private void substituteInPlace(JsonNode node, List<String> texts) {
			if (node.isObject()) {
				ObjectNode obj = (ObjectNode) node;
				for (@SuppressWarnings("deprecation")
				Iterator<Map.Entry<String, JsonNode>> it = obj.fields(); it.hasNext();) {
					Map.Entry<String, JsonNode> entry = it.next();
					int idx = placeholderIndex(entry.getValue(), texts);
					if (idx >= 0)
						obj.put(entry.getKey(), texts.get(idx));
					else
						substituteInPlace(entry.getValue(), texts);
				}
			} else if (node.isArray()) {
				ArrayNode arr = (ArrayNode) node;
				for (int j = 0; j < arr.size(); j++) {
					int idx = placeholderIndex(arr.get(j), texts);
					if (idx >= 0)
						arr.set(j, TextNode.valueOf(texts.get(idx)));
					else
						substituteInPlace(arr.get(j), texts);
				}
			}
		}

		private int placeholderIndex(JsonNode value, List<String> texts) {
			if (!value.isTextual())
				return -1;
			for (int idx = 0; idx < texts.size(); idx++)
				if (SessionRenderer.reasoningPlaceholder(idx).equals(value.asText()))
					return idx;
			return -1;
		}

		private int consumeText(String[] lines, int start) {
			StringBuilder body = new StringBuilder();
			int i = start;
			while (i < lines.length && !isMarkerLine(lines[i])) {
				if (body.length() > 0)
					body.append("\n");
				body.append(lines[i]);
				i++;
			}
			String text = body.toString().strip();
			if (!text.isEmpty())
				out.add(callbacks.message(role, text));
			return i;
		}

		private JsonNode reasonLast(JsonNode arguments) {
			if (!(arguments instanceof ObjectNode) || !arguments.has("reason"))
				return arguments;
			ObjectNode src = (ObjectNode) arguments;
			ObjectNode ordered = JsonNodeFactory.instance.objectNode();
			JsonNode reason = src.get("reason");
			Iterator<Map.Entry<String, JsonNode>> fields = src.fields();
			while (fields.hasNext()) {
				Map.Entry<String, JsonNode> entry = fields.next();
				if (!"reason".equals(entry.getKey()))
					ordered.set(entry.getKey(), entry.getValue());
			}
			ordered.set("reason", reason);
			return ordered;
		}

		private int consumeToolCall(String[] lines, int start) {
			int[] range = fenceRange(lines, start);
			if (range == null)
				return start;
			JsonNode node = readYaml(lines, range);
			out.add(callbacks.toolCall(new ToolCall(node.path("id").asText(""), node.path("tool").asText(""),
					reasonLast(node.path("arguments")))));
			return range[1] + 1;
		}

		private int consumeToolResult(String[] lines, int start) {
			int[] range = fenceRange(lines, start);
			if (range == null)
				return start;
			JsonNode node = "json".equalsIgnoreCase(fenceSpecifier(lines, range)) ? readJson(lines, range)
					: readYaml(lines, range);
			out.add(callbacks.toolResult(new ToolResult(node.path("id").asText(""), compact(node.path("result")))));
			return range[1] + 1;
		}

		private String compact(JsonNode result) {
			if (result.isTextual())
				return result.asText("");
			if (result.isMissingNode() || result.isNull())
				return "";
			try {
				return JSON.writeValueAsString(result);
			} catch (Exception e) {
				return result.toString();
			}
		}

		/**
		 * Finds the fenced ``` block immediately following a marker line; returns
		 * [startLine, endLine] (both fence delimiters), or null.
		 */
		private int[] fenceRange(String[] lines, int from) {
			int i = from;
			while (i < lines.length && lines[i].isBlank())
				i++;
			if (i >= lines.length || !lines[i].strip().startsWith("```"))
				return null;
			int start = i;
			i++;
			while (i < lines.length && !lines[i].strip().equals("```"))
				i++;
			if (i >= lines.length)
				return null;
			return new int[] { start, i };
		}

		private JsonNode readYaml(String[] lines, int[] range) {
			StringBuilder body = new StringBuilder();
			for (int j = range[0] + 1; j < range[1]; j++)
				body.append(lines[j]).append("\n");
			try {
				JsonNode node = yaml.readTree(body.toString());
				return node == null ? NullNode.getInstance() : node;
			} catch (Exception e) {
				throw new IllegalStateException("Invalid YAML block: " + e.getMessage(), e);
			}
		}

		private String fenceSpecifier(String[] lines, int[] range) {
			String head = lines[range[0]].strip();
			return head.length() > 3 ? head.substring(3).strip() : "";
		}

		private JsonNode readJson(String[] lines, int[] range) {
			StringBuilder body = new StringBuilder();
			for (int j = range[0] + 1; j < range[1]; j++)
				body.append(lines[j]).append("\n");
			try {
				JsonNode node = JSON.readTree(body.toString());
				return node == null ? NullNode.getInstance() : node;
			} catch (Exception e) {
				throw new IllegalStateException("Invalid JSON block: " + e.getMessage(), e);
			}
		}

		private void include(String kind, String arg, List<Path> chain) {
			if (kind == null) {
				includePath(arg, chain);
				return;
			}
			switch (kind) {
			case "contextprompt":
				includeContextPrompt(arg);
				return;
			case "files":
				includeFiles(arg);
				return;
			case "file":
				includeFile(arg);
				return;
			case "search":
				includeSearch(arg);
				return;
			default:
				throw new IllegalStateException("Unknown include kind: " + kind);
			}
		}

		/**
		 * Generic {@code [include](path)}: parses the target file as a nested,
		 * independent document.
		 */
		private void includePath(String pathText, List<Path> chain) {
			Path path = Paths.get(pathText).toAbsolutePath().normalize();
			if (chain.contains(path))
				throw new IncludeCycleException(path.toString());
			String content;
			try {
				content = Files.readString(path);
			} catch (IOException e) {
				throw new IllegalStateException("Could not read include target: " + path, e);
			}
			List<Path> nested = new ArrayList<>(chain);
			nested.add(path);
			new Run<>(callbacks, out).run(content, nested);
		}

		private void includeContextPrompt(String dir) {
			String text = requireAdapter().contextPrompt(dir);
			if (text != null && !text.isBlank())
				out.add(callbacks.message(Role.User, text.strip()));
		}

		private void includeFiles(String arg) {
			for (IIncludeAdapter.Entry entry : requireAdapter().files(arg))
				out.add(callbacks.toolResult(new ToolResult(entry.id, entry.content)));
		}

		private void includeFile(String path) {
			String content = requireAdapter().file(path);
			if (content != null)
				out.add(callbacks.toolResult(new ToolResult(path, content)));
		}

		private void includeSearch(String arg) {
			IIncludeAdapter a = requireAdapter();
			if ("files".equals(arg)) {
				for (IIncludeAdapter.Entry entry : a.searchFiles())
					out.add(callbacks.toolResult(new ToolResult(entry.id, entry.content)));
			} else if ("matches".equals(arg)) {
				List<IIncludeAdapter.Match> matches = a.searchMatches();
				out.add(callbacks.toolResult(new ToolResult("search-matches", renderMatches(matches))));
			} else
				throw new IllegalStateException("Unknown search include argument: " + arg);
		}

		private String renderMatches(List<IIncludeAdapter.Match> matches) {
			ArrayNode arr = JsonNodeFactory.instance.arrayNode();
			for (IIncludeAdapter.Match match : matches) {
				com.fasterxml.jackson.databind.node.ObjectNode node = arr.addObject();
				node.put("file", match.file);
				node.put("line", match.line);
				node.put("text", match.text);
			}
			return yaml.toYaml(arr);
		}

		private IIncludeAdapter requireAdapter() {
			if (adapter == null)
				throw new IllegalStateException("No IIncludeAdapter configured for this SessionProcessor");
			return adapter;
		}
	}
}
