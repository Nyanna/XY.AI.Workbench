package xy.ai.workbench;

public class Tools {
	private static String toolList = //
			"ask-user," // top
					+ "file-stats,read-file,list," // retrieve
					+ "write,replace,replace-block,replace-lines,insert," // edit tools
					+ "python-ast," // python ast
					+ "bash,python," // scripting
					+ "markdown,markdown-format," // Dynamic code tools
					+ "colgrep,web-search-exa,web-fetch-exa," // research cpabilities
					+ "context7-libraries,context7-documentation," // specialised research
					+ "openalex-search,openalex-semantic-search,openalex-work," 
					+ "replace-chars," // seldom used
					+ "agt-python,agt-markdown,agt-web-research,agt-github-research," // subagents
					+ "github-get-file,github-get-tree,github-search-code,github-search-commits," // special github
					+ "github-search-repos,github-issue-read,github-list-issues,github-search-issues";
	public static String[] ALL = toolList.split(",");
}
