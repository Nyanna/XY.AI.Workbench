"""GitHub bridge – read-only research tools backed by GitHub's remote MCP server.

Only read-only tools are registered: file/code access, issues, discussions,
pull requests, commits, and project information.
"""


from typing import Any, Callable

from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
from xy.ai.mcpc.tools.mcp.client import McpClient, McpClientError

__all__ = ["GitHubBridge", "GitHubTool", "register_github_tools"]

_CONTENT_OUTPUT: dict[str, Any] = {
    "type": "object",
    "properties": {
        "content": {
            "type": "string",
            "description": "Response content from the GitHub MCP server.",
        },
    },
    "required": ["content"],
}

_RO: dict[str, Any] = {"readOnlyHint": True, "openWorldHint": True}

_GET_FILE_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "owner": {"type": "string", "description": "Repository owner (user or org)."},
        "repo": {"type": "string", "description": "Repository name."},
        "path": {
            "type": "string",
            "description": "Path to the file or directory (omit for root).",
        },
        "ref": {
            "type": "string",
            "description": (
                "Branch, tag, or ref (e.g. refs/heads/main, refs/pull/42/head). "
                "Ignored when sha is provided."
            ),
        },
        "sha": {
            "type": "string",
            "description": "Exact commit SHA; takes precedence over ref.",
        },
    },
    "required": ["owner", "repo"],
}

_GET_TREE_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "owner": {"type": "string", "description": "Repository owner."},
        "repo": {"type": "string", "description": "Repository name."},
        "tree_sha": {
            "type": "string",
            "description": "SHA, branch, or tag to read the tree from (defaults to default branch).",
        },
        "recursive": {
            "type": "boolean",
            "description": "Recurse into sub-trees (default false).",
        },
        "path_filter": {
            "type": "string",
            "description": "Optional path prefix to filter results (e.g. 'src/').",
        },
    },
    "required": ["owner", "repo"],
}

_SEARCH_CODE_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "query": {
            "type": "string",
            "description": (
                "GitHub code search query (max 256 chars). "
                "Qualifiers: repo:owner/repo, org:, language:, path:, "
                "filename:, extension:, in:file|path."
            ),
        },
        "perPage": {
            "type": "integer",
            "description": "Results per page (max 15).",
            "minimum": 1,
            "maximum": 15,
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
    },
    "required": ["query"],
}

_SEARCH_COMMITS_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "query": {
            "type": "string",
            "description": (
                "GitHub commit search query. Scope with repo:owner/repo or org:. "
                "Qualifiers: author:, committer:, author-date:, committer-date:, "
                "merge:true|false, hash:."
            ),
        },
        "sort": {
            "type": "string",
            "description": "Sort by author-date or committer-date (defaults to best match).",
        },
        "order": {"type": "string", "description": "Sort order: asc | desc."},
        "perPage": {
            "type": "integer",
            "description": "Results per page (max 15).",
            "minimum": 1,
            "maximum": 15,
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
    },
    "required": ["query"],
}

_SEARCH_REPOS_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "query": {
            "type": "string",
            "description": (
                "Repository search query. Supports qualifiers: "
                "topic:, language:, stars:>N, user:, org:, is:archived."
            ),
        },
        "sort": {
            "type": "string",
            "description": "Sort by: stars | forks | help-wanted-issues | updated.",
        },
        "order": {"type": "string", "description": "Sort order: asc | desc."},
        "perPage": {
            "type": "integer",
            "description": "Results per page (max 10).",
            "minimum": 1,
            "maximum": 10,
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        "minimal_output": {
            "type": "boolean",
            "description": "Return minimal repository info (default true).",
        },
    },
    "required": ["query"],
}

_ISSUE_READ_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "owner": {"type": "string", "description": "Repository owner."},
        "repo": {"type": "string", "description": "Repository name."},
        "issue_number": {"type": "integer", "description": "Issue number."},
        "method": {
            "type": "string",
            "description": (
                "Read operation to perform:\n"
                "  get – issue body and metadata\n"
                "  get_comments – issue comments\n"
                "  get_sub_issues – child issues\n"
                "  get_parent – parent issue (if this is a sub-issue)\n"
                "  get_labels – labels assigned to the issue"
            ),
            "enum": ["get", "get_comments", "get_sub_issues", "get_parent", "get_labels"],
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        "perPage": {
            "type": "integer",
            "description": "Results per page (max 20).",
            "minimum": 1,
            "maximum": 20,
        },
    },
    "required": ["owner", "repo", "issue_number", "method"],
}

_LIST_ISSUES_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "owner": {"type": "string", "description": "Repository owner."},
        "repo": {"type": "string", "description": "Repository name."},
        "state": {
            "type": "string",
            "description": "Filter by state: open | closed (default: both).",
        },
        "labels": {
            "type": "array",
            "items": {"type": "string"},
            "description": "Filter by label names.",
        },
        "since": {
            "type": "string",
            "description": "Only issues updated after this ISO 8601 timestamp.",
        },
        "perPage": {
            "type": "integer",
            "description": "Results per page (max 15).",
            "minimum": 1,
            "maximum": 15,
        },
        "after": {
            "type": "string",
            "description": "Cursor for pagination (from previous response).",
        },
    },
    "required": ["owner", "repo"],
}

_SEARCH_ISSUES_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "query": {
            "type": "string",
            "description": "Search query using GitHub issues search syntax.",
        },
        "owner": {
            "type": "string",
            "description": "Restrict to this owner (requires repo).",
        },
        "repo": {
            "type": "string",
            "description": "Restrict to this repo (requires owner).",
        },
        "sort": {"type": "string", "description": "Sort field."},
        "order": {"type": "string", "description": "Sort order: asc | desc."},
        "perPage": {
            "type": "integer",
            "description": "Results per page (max 15).",
            "minimum": 1,
            "maximum": 15,
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
    },
    "required": ["query"],
}

_GET_DISCUSSION_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "owner": {"type": "string", "description": "Repository owner."},
        "repo": {"type": "string", "description": "Repository name."},
        "discussionNumber": {"type": "integer", "description": "Discussion number."},
    },
    "required": ["owner", "repo", "discussionNumber"],
}

_GET_DISCUSSION_COMMENTS_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "owner": {"type": "string", "description": "Repository owner."},
        "repo": {"type": "string", "description": "Repository name."},
        "discussionNumber": {"type": "integer", "description": "Discussion number."},
        "includeReplies": {
            "type": "boolean",
            "description": "Include nested replies per comment (up to 100, default false).",
        },
        "perPage": {
            "type": "integer",
            "description": "Results per page (max 20).",
            "minimum": 1,
            "maximum": 20,
        },
        "after": {"type": "string", "description": "Cursor for pagination."},
    },
    "required": ["owner", "repo", "discussionNumber"],
}

_LIST_DISCUSSIONS_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "owner": {"type": "string", "description": "Repository owner or org."},
        "repo": {
            "type": "string",
            "description": "Repository name (omit for org-level discussions).",
        },
        "category": {
            "type": "string",
            "description": "Filter by discussion category ID.",
        },
        "orderBy": {
            "type": "string",
            "description": "Order by field (requires direction).",
        },
        "direction": {"type": "string", "description": "Order direction: ASC | DESC."},
        "perPage": {
            "type": "integer",
            "description": "Results per page (max 20).",
            "minimum": 1,
            "maximum": 20,
        },
        "after": {"type": "string", "description": "Cursor for pagination."},
    },
    "required": ["owner"],
}

_PR_READ_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "owner": {"type": "string", "description": "Repository owner."},
        "repo": {"type": "string", "description": "Repository name."},
        "pullNumber": {"type": "integer", "description": "Pull request number."},
        "method": {
            "type": "string",
            "description": (
                "Data to retrieve:\n"
                "  get – PR body and metadata\n"
                "  get_diff – unified diff\n"
                "  get_status – combined commit status\n"
                "  get_files – changed files\n"
                "  get_commits – commits on the PR\n"
                "  get_review_comments – review threads\n"
                "  get_reviews – review summaries\n"
                "  get_comments – general comments\n"
                "  get_check_runs – CI check runs"
            ),
            "enum": [
                "get",
                "get_diff",
                "get_status",
                "get_files",
                "get_commits",
                "get_review_comments",
                "get_reviews",
                "get_comments",
                "get_check_runs",
            ],
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        "perPage": {
            "type": "integer",
            "description": "Results per page (max 10).",
            "minimum": 1,
            "maximum": 10,
        },
        "after": {
            "type": "string",
            "description": "Cursor for pagination (get_review_comments only).",
        },
    },
    "required": ["owner", "repo", "pullNumber", "method"],
}

_LIST_PRS_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "owner": {"type": "string", "description": "Repository owner."},
        "repo": {"type": "string", "description": "Repository name."},
        "state": {"type": "string", "description": "Filter: open | closed | all."},
        "base": {"type": "string", "description": "Filter by base branch name."},
        "sort": {
            "type": "string",
            "description": "Sort by: created | updated | popularity | long-running.",
        },
        "direction": {"type": "string", "description": "Sort direction: asc | desc."},
        "perPage": {
            "type": "integer",
            "description": "Results per page (max 10).",
            "minimum": 1,
            "maximum": 10,
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
    },
    "required": ["owner", "repo"],
}

_SEARCH_PRS_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "query": {
            "type": "string",
            "description": "Search query using GitHub pull request search syntax.",
        },
        "owner": {
            "type": "string",
            "description": "Restrict to this owner (requires repo).",
        },
        "repo": {
            "type": "string",
            "description": "Restrict to this repo (requires owner).",
        },
        "sort": {"type": "string", "description": "Sort field."},
        "order": {"type": "string", "description": "Sort order: asc | desc."},
        "perPage": {
            "type": "integer",
            "description": "Results per page (max 10).",
            "minimum": 1,
            "maximum": 10,
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
    },
    "required": ["query"],
}

_GET_COMMIT_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "owner": {"type": "string", "description": "Repository owner."},
        "repo": {"type": "string", "description": "Repository name."},
        "sha": {"type": "string", "description": "Commit SHA, branch name, or tag name."},
        "detail": {
            "type": "string",
            "description": (
                "File detail level:\n"
                "  none – omit files entirely\n"
                "  stats – per-file counts (default)\n"
                "  full_patch – includes diff content (can be large)"
            ),
            "enum": ["none", "stats", "full_patch"],
        },
        "perPage": {
            "type": "integer",
            "description": "Results per page (max 10).",
            "minimum": 1,
            "maximum": 10,
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
    },
    "required": ["owner", "repo", "sha"],
}

_LIST_COMMITS_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "owner": {"type": "string", "description": "Repository owner."},
        "repo": {"type": "string", "description": "Repository name."},
        "sha": {
            "type": "string",
            "description": "Branch, tag, or SHA to list commits from (defaults to default branch).",
        },
        "path": {
            "type": "string",
            "description": "Only commits touching this file path.",
        },
        "author": {
            "type": "string",
            "description": "Filter by author username or email.",
        },
        "since": {
            "type": "string",
            "description": "Only commits after this date (ISO 8601: YYYY-MM-DDTHH:MM:SSZ).",
        },
        "until": {
            "type": "string",
            "description": "Only commits before this date (ISO 8601).",
        },
        "perPage": {
            "type": "integer",
            "description": "Results per page (max 10).",
            "minimum": 1,
            "maximum": 10,
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
    },
    "required": ["owner", "repo"],
}

_PROJECTS_GET_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "method": {
            "type": "string",
            "description": (
                "Operation:\n"
                "  get_project – project metadata\n"
                "  get_project_field – a single project field\n"
                "  get_project_item – a single project item\n"
                "  get_project_status_update – a status update"
            ),
            "enum": [
                "get_project",
                "get_project_field",
                "get_project_item",
                "get_project_status_update",
            ],
        },
        "owner": {
            "type": "string",
            "description": "Owner (user or org login).",
        },
        "owner_type": {
            "type": "string",
            "description": "Owner type: user | org (auto-detected if omitted).",
        },
        "project_number": {"type": "integer", "description": "Project number."},
        "field_id": {
            "type": "integer",
            "description": "Field ID (required for get_project_field).",
        },
        "item_id": {
            "type": "integer",
            "description": "Item ID (required for get_project_item).",
        },
        "fields": {
            "type": "array",
            "items": {"type": "string"},
            "description": "Field IDs to include in get_project_item response.",
        },
        "status_update_id": {
            "type": "string",
            "description": "Status update node ID (required for get_project_status_update).",
        },
    },
    "required": ["method"],
}

_PROJECTS_LIST_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "method": {
            "type": "string",
            "description": (
                "Operation:\n"
                "  list_projects – projects for an owner\n"
                "  list_project_fields – fields of a project\n"
                "  list_project_items – items in a project\n"
                "  list_project_status_updates – status updates"
            ),
            "enum": [
                "list_projects",
                "list_project_fields",
                "list_project_items",
                "list_project_status_updates",
            ],
        },
        "owner": {"type": "string", "description": "Owner (user or org login)."},
        "owner_type": {
            "type": "string",
            "description": "Owner type: user | org.",
        },
        "project_number": {
            "type": "integer",
            "description": "Project number (required for fields, items, and status updates).",
        },
        "query": {
            "type": "string",
            "description": (
                "Filter string: for list_projects use title/state filters; "
                "for list_project_items use GitHub project filter syntax."
            ),
        },
        "fields": {
            "type": "array",
            "items": {"type": "string"},
            "description": "Field IDs to include for list_project_items.",
        },
        "per_page": {
            "type": "integer",
            "description": "Results per page (max 20).",
            "minimum": 1,
            "maximum": 20,
        },
        "after": {"type": "string", "description": "Forward pagination cursor."},
        "before": {"type": "string", "description": "Backward pagination cursor."},
    },
    "required": ["method", "owner"],
}


class GitHubBridge(McpBridge):
    """Bridge to the GitHub remote MCP server (read-only)."""

    def build_client(self, config: ServerConfig) -> McpClient:
        pat = config.github_api_pat
        if not pat:
            raise McpClientError(
                "GitHub PAT is not configured (set MCPC_GITHUB_PAT)."
            )
        return McpClient(
            config.github_mcp_url,
            headers={"Authorization": f"Bearer {pat}"},
        )


#: Module-level bridge, built by :func:`register_github_tools`.
_bridge: GitHubBridge | None = None


def _get_bridge() -> GitHubBridge:
    """Return the module-level GitHub bridge configured by :func:`register_github_tools`."""
    if _bridge is None:
        raise McpBridgeError("GitHub tools used before register_github_tools() was called.")
    return _bridge


def github_get_file(
    owner: str, repo: str, path: str | None = None, ref: str | None = None, sha: str | None = None
) -> dict:
    """Read a file or directory listing from a GitHub repository.

    Best for: Fetching source code, configs, and READMEs at any ref or commit.

    Args:
        owner: Repository owner (user or org).
        repo: Repository name.
        path: Path to the file or directory (omit for root).
        ref: Branch, tag, or ref (e.g. refs/heads/main); ignored when sha is given.
        sha: Exact commit SHA; takes precedence over ref.
    """
    return _get_bridge().call("get_file_contents", compact(owner=owner, repo=repo, path=path, ref=ref, sha=sha))


def github_get_tree(
    owner: str,
    repo: str,
    tree_sha: str | None = None,
    recursive: bool | None = None,
    path_filter: str | None = None,
) -> dict:
    """List the file tree of a GitHub repository at a given ref.

    Best for: Understanding project layout before reading individual files.

    Args:
        owner: Repository owner.
        repo: Repository name.
        tree_sha: SHA, branch, or tag to read the tree from (defaults to default branch).
        recursive: Recurse into sub-trees (default false).
        path_filter: Optional path prefix to filter results (e.g. 'src/').
    """
    return _get_bridge().call(
        "get_repository_tree",
        compact(owner=owner, repo=repo, tree_sha=tree_sha, recursive=recursive, path_filter=path_filter),
    )


def github_search_code(query: str, perPage: int | None = None, page: int | None = None) -> dict:
    """Search GitHub code across repositories.

    Best for: Finding specific functions, patterns, or usages across the
    GitHub ecosystem.

    Args:
        query: GitHub code search query (max 256 chars); qualifiers:
            repo:owner/repo, org:, language:, path:, filename:, extension:, in:file|path.
        perPage: Results per page (max 15).
        page: Page number (min 1).
    """
    return _get_bridge().call("search_code", compact(query=query, perPage=perPage, page=page))


def github_search_commits(
    query: str,
    sort: str | None = None,
    order: str | None = None,
    perPage: int | None = None,
    page: int | None = None,
) -> dict:
    """Search commit messages on GitHub.

    Best for: Finding commits by message keyword, author, or date across repositories.

    Args:
        query: GitHub commit search query; scope with repo:owner/repo or org:;
            qualifiers: author:, committer:, author-date:, committer-date:,
            merge:true|false, hash:.
        sort: Sort by author-date or committer-date (defaults to best match).
        order: Sort order: asc | desc.
        perPage: Results per page (max 15).
        page: Page number (min 1).
    """
    return _get_bridge().call(
        "search_commits", compact(query=query, sort=sort, order=order, perPage=perPage, page=page)
    )


def github_search_repos(
    query: str,
    sort: str | None = None,
    order: str | None = None,
    perPage: int | None = None,
    page: int | None = None,
    minimal_output: bool | None = None,
) -> dict:
    """Search GitHub for repositories matching a query.

    Best for: Discovering projects by name, topic, language, or stars.

    Args:
        query: Repository search query; qualifiers: topic:, language:,
            stars:>N, user:, org:, is:archived.
        sort: Sort by: stars | forks | help-wanted-issues | updated.
        order: Sort order: asc | desc.
        perPage: Results per page (max 10).
        page: Page number (min 1).
        minimal_output: Return minimal repository info (default true).
    """
    return _get_bridge().call(
        "search_repositories",
        compact(query=query, sort=sort, order=order, perPage=perPage, page=page, minimal_output=minimal_output),
    )


def github_issue_read(
    owner: str,
    repo: str,
    issue_number: int,
    method: str,
    page: int | None = None,
    perPage: int | None = None,
) -> dict:
    """Read a GitHub issue: body, comments, sub-issues, labels, or parent.

    Args:
        owner: Repository owner.
        repo: Repository name.
        issue_number: Issue number.
        method: One of get | get_comments | get_sub_issues | get_parent | get_labels.
        page: Page number (min 1).
        perPage: Results per page (max 20).
    """
    return _get_bridge().call(
        "issue_read",
        compact(owner=owner, repo=repo, issue_number=issue_number, method=method, page=page, perPage=perPage),
    )


def github_list_issues(
    owner: str,
    repo: str,
    state: str | None = None,
    labels: list[str] | None = None,
    since: str | None = None,
    perPage: int | None = None,
    after: str | None = None,
) -> dict:
    """List issues in a GitHub repository with optional filters.

    Best for: Enumerating open or closed issues, filtering by label or state.

    Args:
        owner: Repository owner.
        repo: Repository name.
        state: Filter by state: open | closed (default: both).
        labels: Filter by label names.
        since: Only issues updated after this ISO 8601 timestamp.
        perPage: Results per page (max 15).
        after: Cursor for pagination (from previous response).
    """
    return _get_bridge().call(
        "list_issues",
        compact(owner=owner, repo=repo, state=state, labels=labels, since=since, perPage=perPage, after=after),
    )


def github_search_issues(
    query: str,
    owner: str | None = None,
    repo: str | None = None,
    sort: str | None = None,
    order: str | None = None,
    perPage: int | None = None,
    page: int | None = None,
) -> dict:
    """Search GitHub issues using GitHub's issue search syntax.

    Best for: Finding issues by keyword, author, label, or state across repositories.

    Args:
        query: Search query using GitHub issues search syntax.
        owner: Restrict to this owner (requires repo).
        repo: Restrict to this repo (requires owner).
        sort: Sort field.
        order: Sort order: asc | desc.
        perPage: Results per page (max 15).
        page: Page number (min 1).
    """
    return _get_bridge().call(
        "search_issues",
        compact(query=query, owner=owner, repo=repo, sort=sort, order=order, perPage=perPage, page=page),
    )


def github_get_discussion(owner: str, repo: str, discussionNumber: int) -> dict:
    """Get the body and metadata of a single GitHub Discussion.

    Best for: Reading a specific community discussion or Q&A thread.

    Args:
        owner: Repository owner.
        repo: Repository name.
        discussionNumber: Discussion number.
    """
    return _get_bridge().call(
        "get_discussion", compact(owner=owner, repo=repo, discussionNumber=discussionNumber)
    )


def github_get_discussion_comments(
    owner: str,
    repo: str,
    discussionNumber: int,
    includeReplies: bool | None = None,
    perPage: int | None = None,
    after: str | None = None,
) -> dict:
    """Get comments for a GitHub Discussion, optionally including nested replies.

    Best for: Reading community feedback, answers, and Q&A responses.

    Args:
        owner: Repository owner.
        repo: Repository name.
        discussionNumber: Discussion number.
        includeReplies: Include nested replies per comment (up to 100, default false).
        perPage: Results per page (max 20).
        after: Cursor for pagination.
    """
    return _get_bridge().call(
        "get_discussion_comments",
        compact(
            owner=owner,
            repo=repo,
            discussionNumber=discussionNumber,
            includeReplies=includeReplies,
            perPage=perPage,
            after=after,
        ),
    )


def github_list_discussions(
    owner: str,
    repo: str | None = None,
    category: str | None = None,
    orderBy: str | None = None,
    direction: str | None = None,
    perPage: int | None = None,
    after: str | None = None,
) -> dict:
    """List GitHub Discussions for a repository or organisation.

    Best for: Browsing community discussions, optionally filtered by category.

    Args:
        owner: Repository owner or org.
        repo: Repository name (omit for org-level discussions).
        category: Filter by discussion category ID.
        orderBy: Order by field (requires direction).
        direction: Order direction: ASC | DESC.
        perPage: Results per page (max 20).
        after: Cursor for pagination.
    """
    return _get_bridge().call(
        "list_discussions",
        compact(
            owner=owner, repo=repo, category=category, orderBy=orderBy, direction=direction,
            perPage=perPage, after=after,
        ),
    )


def github_pr_read(
    owner: str,
    repo: str,
    pullNumber: int,
    method: str,
    page: int | None = None,
    perPage: int | None = None,
    after: str | None = None,
) -> dict:
    """Read details of a GitHub Pull Request: body, diff, files, commits, reviews, or comments.

    Args:
        owner: Repository owner.
        repo: Repository name.
        pullNumber: Pull request number.
        method: One of get | get_diff | get_status | get_files | get_commits |
            get_review_comments | get_reviews | get_comments | get_check_runs.
        page: Page number (min 1).
        perPage: Results per page (max 10).
        after: Cursor for pagination (get_review_comments only).
    """
    return _get_bridge().call(
        "pull_request_read",
        compact(owner=owner, repo=repo, pullNumber=pullNumber, method=method, page=page, perPage=perPage, after=after),
    )


def github_list_prs(
    owner: str,
    repo: str,
    state: str | None = None,
    base: str | None = None,
    sort: str | None = None,
    direction: str | None = None,
    perPage: int | None = None,
    page: int | None = None,
) -> dict:
    """List pull requests in a GitHub repository.

    Best for: Enumerating open or merged PRs with optional state and base-branch filters.

    Args:
        owner: Repository owner.
        repo: Repository name.
        state: Filter: open | closed | all.
        base: Filter by base branch name.
        sort: Sort by: created | updated | popularity | long-running.
        direction: Sort direction: asc | desc.
        perPage: Results per page (max 10).
        page: Page number (min 1).
    """
    return _get_bridge().call(
        "list_pull_requests",
        compact(owner=owner, repo=repo, state=state, base=base, sort=sort, direction=direction,
                perPage=perPage, page=page),
    )


def github_search_prs(
    query: str,
    owner: str | None = None,
    repo: str | None = None,
    sort: str | None = None,
    order: str | None = None,
    perPage: int | None = None,
    page: int | None = None,
) -> dict:
    """Search GitHub pull requests using GitHub's PR search syntax.

    Best for: Finding PRs by keyword, author, state, or label across repositories.

    Args:
        query: Search query using GitHub pull request search syntax.
        owner: Restrict to this owner (requires repo).
        repo: Restrict to this repo (requires owner).
        sort: Sort field.
        order: Sort order: asc | desc.
        perPage: Results per page (max 10).
        page: Page number (min 1).
    """
    return _get_bridge().call(
        "search_pull_requests",
        compact(query=query, owner=owner, repo=repo, sort=sort, order=order, perPage=perPage, page=page),
    )


def github_get_commit(
    owner: str,
    repo: str,
    sha: str,
    detail: str | None = None,
    perPage: int | None = None,
    page: int | None = None,
) -> dict:
    """Get details of a single GitHub commit including changed files.

    Best for: Inspecting what changed in a specific commit.

    Args:
        owner: Repository owner.
        repo: Repository name.
        sha: Commit SHA, branch name, or tag name.
        detail: File detail level: none | stats (default) | full_patch.
        perPage: Results per page (max 10).
        page: Page number (min 1).
    """
    return _get_bridge().call(
        "get_commit", compact(owner=owner, repo=repo, sha=sha, detail=detail, perPage=perPage, page=page)
    )


def github_list_commits(
    owner: str,
    repo: str,
    sha: str | None = None,
    path: str | None = None,
    author: str | None = None,
    since: str | None = None,
    until: str | None = None,
    perPage: int | None = None,
    page: int | None = None,
) -> dict:
    """List commits in a GitHub repository, optionally filtered by author, path, or date.

    Best for: Reviewing recent history or changes to a specific file.

    Args:
        owner: Repository owner.
        repo: Repository name.
        sha: Branch, tag, or SHA to list commits from (defaults to default branch).
        path: Only commits touching this file path.
        author: Filter by author username or email.
        since: Only commits after this date (ISO 8601).
        until: Only commits before this date (ISO 8601).
        perPage: Results per page (max 10).
        page: Page number (min 1).
    """
    return _get_bridge().call(
        "list_commits",
        compact(owner=owner, repo=repo, sha=sha, path=path, author=author, since=since, until=until,
                perPage=perPage, page=page),
    )


def github_projects_get(
    method: str,
    owner: str | None = None,
    owner_type: str | None = None,
    project_number: int | None = None,
    field_id: int | None = None,
    item_id: int | None = None,
    fields: list[str] | None = None,
    status_update_id: str | None = None,
) -> dict:
    """Get details of a GitHub Project or one of its fields, items, or status updates.

    Args:
        method: One of get_project | get_project_field | get_project_item |
            get_project_status_update.
        owner: Owner (user or org login).
        owner_type: Owner type: user | org (auto-detected if omitted).
        project_number: Project number.
        field_id: Field ID (required for get_project_field).
        item_id: Item ID (required for get_project_item).
        fields: Field IDs to include in get_project_item response.
        status_update_id: Status update node ID (required for get_project_status_update).
    """
    return _get_bridge().call(
        "projects_get",
        compact(
            method=method, owner=owner, owner_type=owner_type, project_number=project_number,
            field_id=field_id, item_id=item_id, fields=fields, status_update_id=status_update_id,
        ),
    )


def github_projects_list(
    method: str,
    owner: str,
    owner_type: str | None = None,
    project_number: int | None = None,
    query: str | None = None,
    fields: list[str] | None = None,
    per_page: int | None = None,
    after: str | None = None,
    before: str | None = None,
) -> dict:
    """List GitHub Projects resources: projects, fields, items, or status updates.

    Args:
        method: One of list_projects | list_project_fields | list_project_items |
            list_project_status_updates.
        owner: Owner (user or org login).
        owner_type: Owner type: user | org.
        project_number: Project number (required for fields, items, and status updates).
        query: Filter string (title/state filters for list_projects; GitHub
            project filter syntax for list_project_items).
        fields: Field IDs to include for list_project_items.
        per_page: Results per page (max 20).
        after: Forward pagination cursor.
        before: Backward pagination cursor.
    """
    return _get_bridge().call(
        "projects_list",
        compact(
            method=method, owner=owner, owner_type=owner_type, project_number=project_number,
            query=query, fields=fields, per_page=per_page, after=after, before=before,
        ),
    )


class GitHubTool(ToolDefinition):
    """Generic read-only GitHub tool: forwards ``ctx.arguments`` to a bridge-backed
    core function, translating an :class:`McpBridgeError` into an error
    :class:`ToolResult`. The core function remains the only thing that talks
    to the bridge.
    """

    def __init__(
        self, name: str, title: str, description: str, input_schema: dict[str, Any], core: Callable[..., dict]
    ) -> None:
        self.name = name
        self.title = title
        self.description = description
        self.input_schema = input_schema
        self.output_schema = _CONTENT_OUTPUT
        self.annotations = _RO
        self._core = core

    def handle(self, ctx: ToolContext) -> ToolResult:
        try:
            result = self._core(**ctx.arguments)
        except McpBridgeError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content=result)


_TOOLS: list[tuple[str, str, str, dict[str, Any], Callable[..., dict]]] = [
    (
        "github_get_file",
        "GitHub get file contents",
        "Read a file or directory listing from a GitHub repository.\n\n"
        "Best for: Fetching source code, configs, and READMEs at any ref or commit.",
        _GET_FILE_SCHEMA,
        github_get_file,
    ),
    (
        "github_get_tree",
        "GitHub get repository tree",
        "List the file tree of a GitHub repository at a given ref.\n\n"
        "Best for: Understanding project layout before reading individual files.",
        _GET_TREE_SCHEMA,
        github_get_tree,
    ),
    (
        "github_search_code",
        "GitHub search code",
        "Search GitHub code across repositories.\n\n"
        "Best for: Finding specific functions, patterns, or usages across the GitHub ecosystem.",
        _SEARCH_CODE_SCHEMA,
        github_search_code,
    ),
    (
        "github_search_commits",
        "GitHub search commits",
        "Search commit messages on GitHub.\n\n"
        "Best for: Finding commits by message keyword, author, or date across repositories.",
        _SEARCH_COMMITS_SCHEMA,
        github_search_commits,
    ),
    (
        "github_search_repos",
        "GitHub search repositories",
        "Search GitHub for repositories matching a query.\n\n"
        "Best for: Discovering projects by name, topic, language, or stars.",
        _SEARCH_REPOS_SCHEMA,
        github_search_repos,
    ),
    (
        "github_issue_read",
        "GitHub read issue",
        "Read a GitHub issue: body, comments, sub-issues, labels, or parent.\n\n"
        "method: get | get_comments | get_sub_issues | get_parent | get_labels",
        _ISSUE_READ_SCHEMA,
        github_issue_read,
    ),
    (
        "github_list_issues",
        "GitHub list issues",
        "List issues in a GitHub repository with optional filters.\n\n"
        "Best for: Enumerating open or closed issues, filtering by label or state.",
        _LIST_ISSUES_SCHEMA,
        github_list_issues,
    ),
    (
        "github_search_issues",
        "GitHub search issues",
        "Search GitHub issues using GitHub's issue search syntax.\n\n"
        "Best for: Finding issues by keyword, author, label, or state across repositories.",
        _SEARCH_ISSUES_SCHEMA,
        github_search_issues,
    ),
    (
        "github_get_discussion",
        "GitHub get discussion",
        "Get the body and metadata of a single GitHub Discussion.\n\n"
        "Best for: Reading a specific community discussion or Q&A thread.",
        _GET_DISCUSSION_SCHEMA,
        github_get_discussion,
    ),
    (
        "github_get_discussion_comments",
        "GitHub get discussion comments",
        "Get comments for a GitHub Discussion, optionally including nested replies.\n\n"
        "Best for: Reading community feedback, answers, and Q&A responses.",
        _GET_DISCUSSION_COMMENTS_SCHEMA,
        github_get_discussion_comments,
    ),
    (
        "github_list_discussions",
        "GitHub list discussions",
        "List GitHub Discussions for a repository or organisation.\n\n"
        "Best for: Browsing community discussions, optionally filtered by category.",
        _LIST_DISCUSSIONS_SCHEMA,
        github_list_discussions,
    ),
    (
        "github_pr_read",
        "GitHub read pull request",
        "Read details of a GitHub Pull Request: body, diff, files, commits, "
        "reviews, or comments.\n\n"
        "method: get | get_diff | get_status | get_files | get_commits | "
        "get_review_comments | get_reviews | get_comments | get_check_runs",
        _PR_READ_SCHEMA,
        github_pr_read,
    ),
    (
        "github_list_prs",
        "GitHub list pull requests",
        "List pull requests in a GitHub repository.\n\n"
        "Best for: Enumerating open or merged PRs with optional state and base-branch filters.",
        _LIST_PRS_SCHEMA,
        github_list_prs,
    ),
    (
        "github_search_prs",
        "GitHub search pull requests",
        "Search GitHub pull requests using GitHub's PR search syntax.\n\n"
        "Best for: Finding PRs by keyword, author, state, or label across repositories.",
        _SEARCH_PRS_SCHEMA,
        github_search_prs,
    ),
    (
        "github_get_commit",
        "GitHub get commit",
        "Get details of a single GitHub commit including changed files.\n\n"
        "Best for: Inspecting what changed in a specific commit.",
        _GET_COMMIT_SCHEMA,
        github_get_commit,
    ),
    (
        "github_list_commits",
        "GitHub list commits",
        "List commits in a GitHub repository, optionally filtered by author, path, or date.\n\n"
        "Best for: Reviewing recent history or changes to a specific file.",
        _LIST_COMMITS_SCHEMA,
        github_list_commits,
    ),
    (
        "github_projects_get",
        "GitHub get project",
        "Get details of a GitHub Project or one of its fields, items, or status updates.\n\n"
        "method: get_project | get_project_field | get_project_item | get_project_status_update",
        _PROJECTS_GET_SCHEMA,
        github_projects_get,
    ),
    (
        "github_projects_list",
        "GitHub list projects",
        "List GitHub Projects resources: projects, fields, items, or status updates.\n\n"
        "method: list_projects | list_project_fields | list_project_items | "
        "list_project_status_updates",
        _PROJECTS_LIST_SCHEMA,
        github_projects_list,
    ),
]


def register_github_tools(
    registry: ToolRegistry,
    environment: AppEnvironment,
) -> None:
    """Register read-only GitHub research tools."""
    global _bridge
    _bridge = GitHubBridge(environment.config)
    functions = environment.functions
    for name, title, description, input_schema, core in _TOOLS:
        registry.register(GitHubTool(name, title, description, input_schema, core))
        functions.register(core)
