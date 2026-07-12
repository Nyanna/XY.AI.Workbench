package xy.ai.workbench.views.explorer;

import java.util.regex.Pattern;

/**
 * A single parsed line of a ".gitignore" file, compiled into two regular
 * expressions:
 * <ul>
 * <li>{@code exactPattern} matches when the checked path itself is the
 * ignored entry.</li>
 * <li>{@code nestedPattern} matches when the checked path is located
 * <i>inside</i> an ignored directory.</li>
 * </ul>
 */
final class GitIgnoreRule {

	private final Pattern exactPattern;
	private final Pattern nestedPattern;
	private final boolean negate;
	private final boolean dirOnly;

	private GitIgnoreRule(Pattern exactPattern, Pattern nestedPattern, boolean negate, boolean dirOnly) {
		this.exactPattern = exactPattern;
		this.nestedPattern = nestedPattern;
		this.negate = negate;
		this.dirOnly = dirOnly;
	}

	boolean isNegate() {
		return negate;
	}

	/**
	 * @param relativePath path of the checked resource, relative to the
	 *                      directory that contains the ".gitignore" file,
	 *                      using '/' as separator
	 * @param isDirectory   whether the checked resource itself is a folder
	 */
	boolean matches(String relativePath, boolean isDirectory) {
		if (exactPattern.matcher(relativePath).matches())
			return !dirOnly || isDirectory;
		return nestedPattern.matcher(relativePath).matches();
	}

	static GitIgnoreRule parse(String rawLine) {
		if (rawLine == null)
			return null;
		String line = rawLine.stripTrailing();
		if (line.isEmpty() || line.startsWith("#"))
			return null;

		boolean negate = false;
		if (line.startsWith("!")) {
			negate = true;
			line = line.substring(1);
		}
		line = line.replace("\\ ", " ").replace("\\!", "!").replace("\\#", "#");
		if (line.isEmpty())
			return null;

		boolean dirOnly = line.endsWith("/");
		if (dirOnly)
			line = line.substring(0, line.length() - 1);
		if (line.isEmpty())
			return null;

		boolean anchored = line.startsWith("/");
		String pattern = anchored ? line.substring(1) : line;
		if (!anchored && pattern.indexOf('/') >= 0)
			anchored = true;
		if (pattern.isEmpty())
			return null;

		String regex = toRegex(pattern);
		String prefix = anchored ? "" : "(?:.*/)?";
		Pattern exact = Pattern.compile("^" + prefix + regex + "$");
		Pattern nested = Pattern.compile("^" + prefix + regex + "/.*$");
		return new GitIgnoreRule(exact, nested, negate, dirOnly);
	}

	private static String toRegex(String pattern) {
		StringBuilder regex = new StringBuilder();
		int n = pattern.length();
		for (int i = 0; i < n; i++) {
			char c = pattern.charAt(i);
			if (c == '*') {
				if (i + 1 < n && pattern.charAt(i + 1) == '*') {
					boolean slashBefore = i == 0 || pattern.charAt(i - 1) == '/';
					boolean slashAfter = i + 2 < n && pattern.charAt(i + 2) == '/';
					if (slashBefore && slashAfter) {
						regex.append("(?:.*/)?");
						i += 2; // consumes "**", trailing '/' is skipped by the loop increment
						continue;
					}
					regex.append(".*");
					i++; // consumes the second '*'
					continue;
				}
				regex.append("[^/]*");
			} else if (c == '?') {
				regex.append("[^/]");
			} else if (c == '/') {
				regex.append('/');
			} else if ("\\.^$|()[]{}+".indexOf(c) >= 0) {
				regex.append('\\').append(c);
			} else {
				regex.append(c);
			}
		}
		return regex.toString();
	}
}
