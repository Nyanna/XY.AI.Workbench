package xy.ai.workbench.connector.harness;

import java.util.List;

/**
 * Resolves the non-generic include kinds ({@code contextprompt}, {@code files},
 * {@code file}, {@code search}) recognized by {@link SessionProcessor}, keeping
 * the processor itself free of any host-specific (e.g. Eclipse) retrieval
 * logic.
 */
public interface IIncludeAdapter {

	/**
	 * {@code [include contextprompt](dir)}: the context-prompt text found in
	 * {@code dir}, or {@code null}.
	 */
	String contextPrompt(String dir);

	/**
	 * {@code [include files](selected|dir)}: the resolved entries, in the order
	 * they should appear.
	 */
	List<Entry> files(String arg);

	/**
	 * {@code [include file](path)}: the content of a single file, or {@code null}.
	 */
	String file(String path);

	/**
	 * {@code [include search](files)}: the files matched by the current search, as
	 * entries.
	 */
	List<Entry> searchFiles();

	/**
	 * {@code [include search](matches)}: the typed matches of the current search.
	 */
	List<Match> searchMatches();

	/**
	 * A single resolved file: {@code id} (path/name, used as the {@link ToolResult}
	 * id) and its raw content.
	 */
	public class Entry {
		public final String id;
		public final String content;

		public Entry(String id, String content) {
			this.id = id;
			this.content = content;
		}
	}

	/** A single typed search match. */
	public class Match {
		public final String file;
		public final int line;
		public final String text;

		public Match(String file, int line, String text) {
			this.file = file;
			this.line = line;
			this.text = text;
		}
	}
}
