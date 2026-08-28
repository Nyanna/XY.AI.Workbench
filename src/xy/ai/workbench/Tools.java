package xy.ai.workbench;

public class Tools {
	private static String toolList = //
			"ask_user,tools," // top
					+ "file_stats,read_file,list,grep," // retrieve
					+ "write,replace,replace_block,replace_lines,insert," // edit tools
					+ "python_ast," // python ast
					+ "bash,python," // scripting
					+ "markdown,markdown_format," // Dynamic code tools
					+ "colgrep,web_search_exa,web_fetch_exa," // research cpabilities
					+ "context7_libraries,context7_documentation," // specialised research
					+ "openalex_search,openalex_semantic_search,openalex_work," 
					+ "replace_chars," // seldom used
					+ "agt_python,agt_markdown,agt_web_research,agt_github_research," // subagents
					+ "github_get_file,github_get_tree,github_search_code,github_search_commits," // special github
					+ "github_search_repos,github_issue_read,github_list_issues,github_search_issues";
	public static String[] ALL = toolList.split(",");
}
