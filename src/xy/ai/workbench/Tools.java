package xy.ai.workbench;

public class Tools {
	private static String toolList = //
			"ask-user,bash,change,insert,markdown,markdown-format,python,read,replace-chars,replace-lines,write,"
					+ "agt-python,agt-markdown,agt-web-research,agt-github-research," //
					+ "web-search-exa,web-fetch-exa," //
					+ "context7-libraries,context7-documentation,"
					+ "github-get-file,github-get-tree,github-search-code,github-search-commits,"
					+ "github-search-repos,github-issue-read,github-list-issues,github-search-issues";
	public static String[] ALL = toolList.split(",");
}
