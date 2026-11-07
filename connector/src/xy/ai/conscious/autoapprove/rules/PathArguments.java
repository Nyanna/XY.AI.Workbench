package xy.ai.conscious.autoapprove.rules;

import java.nio.file.InvalidPathException;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Set;

import com.fasterxml.jackson.databind.JsonNode;

/** Helpers to find and evaluate path-like values in tool arguments. */
public class PathArguments {

	private PathArguments() {
	}

	/** Collects all string values (or string array elements) of fields named like {@code fields}, at any depth. */
	static List<String> collect(JsonNode node, Set<String> fields) {
		List<String> out = new ArrayList<>();
		collect(node, fields, out);
		return out;
	}

	@SuppressWarnings("deprecation")
	private static void collect(JsonNode node, Set<String> fields, List<String> out) {
		if (node == null)
			return;
		if (node.isObject()) {
			node.fields().forEachRemaining(e -> {
				if (fields.contains(e.getKey()))
					addValues(e.getValue(), out);
				collect(e.getValue(), fields, out);
			});
		} else if (node.isArray()) {
			node.forEach(child -> collect(child, fields, out));
		}
	}

	private static void addValues(JsonNode value, List<String> out) {
		if (value.isTextual())
			out.add(value.asText());
		else if (value.isArray())
			value.forEach(v -> addValues(v, out));
	}

	/** Resolves against {@code cwd} and normalizes; null if the value is not a valid path. */
	static Path resolve(Path cwd, String value) {
		try {
			return cwd.resolve(value).normalize();
		} catch (InvalidPathException e) {
			return null;
		}
	}

	static boolean isRoot(Path path) {
		return path.isAbsolute() && path.getNameCount() == 0;
	}
}
