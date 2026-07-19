package xy.ai.workbench.connectors.claudecode;

import java.io.BufferedReader;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.io.OutputStreamWriter;
import java.io.PrintWriter;
import java.io.Writer;
import java.nio.charset.StandardCharsets;
import java.util.Objects;
import java.util.zip.CRC32;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.core.io.JsonStringEncoder;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

/**
 * Central utility for JSON parsing/building and for stream handling used by the
 * Claude Code connector.
 *
 * <p>
 * The connector repeatedly wraps and unwraps JSON documents as it moves data
 * between the CLI's stream-json transport, the MCPC control endpoint and the
 * Eclipse UI. Doing this by hand ({@link JsonNode#toString()},
 * {@code new ObjectMapper()} per class, platform-default stream charsets, ...)
 * is the root cause of the observed escaping defects, where a value such as the
 * regex {@code [\s\S]} is re-serialised into {@code [\\s\\S]} because a JSON
 * <em>string literal</em> was rendered instead of its logical text.
 *
 * <p>
 * All connector code should therefore go through this class:
 * <ul>
 * <li>{@link #mapper()} — one shared, consistently configured mapper.</li>
 * <li>{@link #plainText(JsonNode)} — the logical value of a node
 * <em>without</em> JSON quoting/escaping (the correct thing to show in the UI
 * or embed into a text container).</li>
 * <li>{@link #compact(JsonNode)} / {@link #pretty(JsonNode)} — a valid JSON
 * document when the structure itself must be preserved.</li>
 * <li>{@link #escape(String)} / {@link #unescape(String)} — round-trippable
 * JSON string escaping when raw text is embedded into a hand-built JSON
 * document.</li>
 * <li>{@link #newReader(InputStream)} / {@link #newWriter(OutputStream)} —
 * UTF-8 stream wrappers so bytes on the wire and characters in memory never
 * disagree.</li>
 * </ul>
 */
public final class JsonUtil {

	private static final ObjectMapper MAPPER = new ObjectMapper();

	private JsonUtil() {
	}

	/** The single shared, consistently configured {@link ObjectMapper}. */
	public static ObjectMapper mapper() {
		return MAPPER;
	}

	/**
	 * Parses {@code json} into a tree. Unlike ad-hoc {@code readTree} calls this
	 * never returns {@code null} and never swallows the parse error: callers get a
	 * checked {@link JsonProcessingException} they must handle (log/skip) instead
	 * of silently losing the line.
	 *
	 * @throws NullPointerException    if {@code json} is {@code null}
	 * @throws JsonProcessingException if {@code json} is not well-formed JSON
	 */
	public static JsonNode readTree(String json) throws JsonProcessingException {
		Objects.requireNonNull(json, "json to parse must not be null");
		JsonNode node = MAPPER.readTree(json);
		if (node == null)
			throw new IllegalArgumentException("JSON parsed to a null tree: " + abbreviate(json));
		return node;
	}

	/**
	 * Returns the logical text of {@code node} <em>without</em> JSON
	 * quoting/escaping. This is what must be shown to the user or embedded into a
	 * non-JSON text container.
	 *
	 * <ul>
	 * <li>A missing or explicit-null node yields the empty string.</li>
	 * <li>A value node (string/number/boolean) yields its plain text — e.g. the
	 * string node {@code "[\s\S]"} yields {@code [\s\S]}, <em>not</em>
	 * {@code "[\\s\\S]"}.</li>
	 * <li>An object/array node yields pretty-printed JSON, since the structure
	 * itself carries the meaning.</li>
	 * </ul>
	 */
	public static String plainText(JsonNode node) {
		if (node == null || node.isMissingNode() || node.isNull())
			return "";
		if (node.isValueNode())
			return node.asText();
		return pretty(node);
	}

	/**
	 * Compact single-line JSON for {@code node} (empty string for {@code null}).
	 */
	public static String compact(JsonNode node) {
		if (node == null || node.isMissingNode())
			return "";
		try {
			return MAPPER.writeValueAsString(node);
		} catch (JsonProcessingException e) {
			return node.toString();
		}
	}

	/** Pretty-printed JSON for {@code node} (empty string for {@code null}). */
	public static String pretty(JsonNode node) {
		if (node == null || node.isMissingNode())
			return "";
		try {
			return MAPPER.writerWithDefaultPrettyPrinter().writeValueAsString(node);
		} catch (JsonProcessingException e) {
			return node.toString();
		}
	}

	/** Serialises any value to a compact JSON document. */
	public static String write(Object value) {
		try {
			return MAPPER.writeValueAsString(value);
		} catch (JsonProcessingException e) {
			throw new IllegalStateException("Failed to serialise value to JSON", e);
		}
	}

	/**
	 * Escapes {@code raw} for safe embedding as the <em>content</em> of a JSON
	 * string literal (no surrounding quotes). Use this whenever raw text is placed
	 * into a hand-built JSON document; it guarantees exactly one level of escaping.
	 */
	public static String escape(String raw) {
		if (raw == null)
			return "";
		return new String(JsonStringEncoder.getInstance().quoteAsString(raw));
	}

	/**
	 * Inverse of {@link #escape(String)}: interprets JSON escape sequences in
	 * {@code escaped} and returns the logical text.
	 *
	 * @throws IllegalArgumentException if {@code escaped} is not a valid JSON
	 *                                  string body
	 */
	public static String unescape(String escaped) {
		if (escaped == null)
			return "";
		try {
			return MAPPER.readValue('"' + escaped + '"', String.class);
		} catch (JsonProcessingException e) {
			throw new IllegalArgumentException("Not a valid JSON string body: " + abbreviate(escaped), e);
		}
	}

	/** UTF-8 buffered reader for a process/socket input stream. */
	public static BufferedReader newReader(InputStream in) {
		Objects.requireNonNull(in, "input stream must not be null");
		return new BufferedReader(new InputStreamReader(in, StandardCharsets.UTF_8));
	}

	/** UTF-8 print writer for a process/socket output stream. */
	public static PrintWriter newWriter(OutputStream out) {
		Objects.requireNonNull(out, "output stream must not be null");
		return new PrintWriter(new OutputStreamWriter(out, StandardCharsets.UTF_8));
	}

	/** UTF-8 writer for a file output stream. */
	public static Writer newWriter(OutputStream out, boolean autoFlushPrintWriter) {
		Objects.requireNonNull(out, "output stream must not be null");
		Writer w = new OutputStreamWriter(out, StandardCharsets.UTF_8);
		return autoFlushPrintWriter ? new PrintWriter(w, true) : w;
	}

	public static String abbreviate(String s) {
		if (s == null)
			return "null";
		final int max = 100;
		if (s.length() <= max)
			return s;

		CRC32 crc = new CRC32();
		crc.update(s.getBytes());
		return s.substring(0, max) + "…(" + s.length() + " chars total, " + crc.getValue() + ")";
	}
}
