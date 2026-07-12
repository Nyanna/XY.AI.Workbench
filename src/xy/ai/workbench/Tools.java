package xy.ai.workbench;

public class Tools {
	private static String toolList = //
			"ask-user," // top
					+ "read,list,change,insert,write,replace-chars,replace-lines," // Read and Edit tools
					+ "bash,python,markdown,markdown-format," // Dynamic code tools
					+ "web-search-exa,web-fetch-exa," // research cpabilities
					+ "context7-libraries,context7-documentation," // specialised research
					+ "openalex-search,openalex-semantic-search,openalex-work," 
					+ "agt-python,agt-markdown,agt-web-research,agt-github-research," // subagents
					+ "github-get-file,github-get-tree,github-search-code,github-search-commits," // special github
					+ "github-search-repos,github-issue-read,github-list-issues,github-search-issues";
	public static String[] ALL = toolList.split(",");
}
