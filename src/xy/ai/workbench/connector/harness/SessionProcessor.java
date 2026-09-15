package xy.ai.workbench.connector.harness;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.NullNode;

import xy.ai.workbench.EditorInterface;
import xy.ai.workbench.connector.claudecode.YamlRenderer;

/**
 * Deterministic, symmetric text &lt;-&gt; message-list translator shared by
 * all connectors. Recursively resolves {@code [include](path)} directives
 * (removed as a line-level separator; the target is parsed independently
 * and spliced in at that point) and turns the resulting markdown into an
 * ordered sequence of {@link SessionCallbacks} invocations.
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
 * When {@link #SessionProcessor(boolean) disabled}, the whole input is
 * passed through {@link SessionCallbacks#message(Role, String)} verbatim -
 * the previous, simple behaviour.
 */
public final class SessionProcessor {

	private static final Pattern INCLUDE_LINE = Pattern.compile("^(\\s*\\[include\\]\\([^)]+\\)\\s*)+$");
	private static final Pattern INCLUDE_TAG = Pattern.compile("\\[include\\]\\(([^)]+)\\)");

	private final boolean enabled;
	private final YamlRenderer yaml = new YamlRenderer();

	public SessionProcessor() {
		this(true);
	}

	public SessionProcessor(boolean enabled) {
		this.enabled = enabled;
	}

	/** Processes each list entry as an independent root document (matches the {@code inputs} lists connectors build requests from). */
	public <M> List<M> process(List<String> inputs, SessionCallbacks<M> callbacks) {
		List<M> out = new ArrayList<>();
		if (inputs == null)
			return out;
		if (!enabled) {
			for (String input : inputs)
				if (input != null && !input.isBlank())
					out.add(callbacks.message(Role.User, input));
			return out;
		}
		for (String input : inputs)
			if (input != null && !input.isBlank())
				new Run<>(callbacks, out).run(input, List.of());
		return out;
	}

	/** Processes a single root document, optionally anchored at {@code rootPath} for include-cycle detection against itself. */
	public <M> List<M> process(String input, Path rootPath, SessionCallbacks<M> callbacks) {
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

	public <M> List<M> process(String input, SessionCallbacks<M> callbacks) {
		return process(input, null, callbacks);
	}

	/** One recursive parse: a stateful pass producing callback-built messages in document order. */
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

				if (INCLUDE_LINE.matcher(line).matches()) {
					flush();
					Matcher m = INCLUDE_TAG.matcher(line);
					while (m.find())
						include(m.group(1).strip(), chain);
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
					|| s.equals(EditorInterface.TOOLUSE) || s.equals(EditorInterface.TOOLRESULT);
		}

		private int consumeReasoning(String[] lines, int start) {
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
				out.add(callbacks.reasoning(role, text));
			return i;
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

		private int consumeToolCall(String[] lines, int start) {
			int[] range = fenceRange(lines, start);
			if (range == null)
				return start;
			JsonNode node = readYaml(lines, range);
			out.add(callbacks.toolCall(
					new ToolCall(node.path("id").asText(""), node.path("tool").asText(""), node.path("arguments"))));
			return range[1] + 1;
		}

		private int consumeToolResult(String[] lines, int start) {
			int[] range = fenceRange(lines, start);
			if (range == null)
				return start;
			JsonNode node = readYaml(lines, range);
			out.add(callbacks.toolResult(new ToolResult(node.path("id").asText(""), node.path("result").asText(""))));
			return range[1] + 1;
		}

		/** Finds the fenced ``` block immediately following a marker line; returns [startLine, endLine] (both fence delimiters), or null. */
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

		private void include(String pathText, List<Path> chain) {
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
	}
}
