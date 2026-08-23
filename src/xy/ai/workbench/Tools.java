package xy.ai.workbench;

public class Tools {
	private static String toolList = //
			"ask-user," // top
					+ "file-stats,read-file,list,write,replace-block," // Read and Edit tools
					+ "bash,python,markdown,markdown-format," // Dynamic code tools
					+ "change,insert,replace-chars,replace-lines," // seldom used
					+ "python-ast," // python ast
					+ "colgrep,web-search-exa,web-fetch-exa," // research cpabilities
					+ "context7-libraries,context7-documentation," // specialised research
					+ "openalex-search,openalex-semantic-search,openalex-work," 
					+ "agt-python,agt-markdown,agt-web-research,agt-github-research," // subagents
					+ "github-get-file,github-get-tree,github-search-code,github-search-commits," // special github
					+ "github-search-repos,github-issue-read,github-list-issues,github-search-issues";
	public static String[] ALL = toolList.split(",");
}
