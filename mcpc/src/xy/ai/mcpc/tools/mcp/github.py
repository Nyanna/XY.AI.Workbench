"""GitHub bridge – read-only research tools backed by GitHub's remote MCP server.

Only read-only tools are registered: file/code access, issues, discussions,
pull requests, commits, and project information.
"""

from __future__ import annotations

from typing import Any

from ...config import ServerConfig
from ...registry import ToolRegistry
from .bridge import McpBridge
from .client import McpClient, McpClientError

# ---------------------------------------------------------------------------
# Shared helpers
# ---------------------------------------------------------------------------

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

# ---------------------------------------------------------------------------
# File / code tools
# ---------------------------------------------------------------------------

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
            "description": "Results per page (max 100).",
            "minimum": 1,
            "maximum": 100,
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
            "description": "Results per page (max 100).",
            "minimum": 1,
            "maximum": 100,
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
    },
    "required": ["query"],
}

# ---------------------------------------------------------------------------
# Repository tools
# ---------------------------------------------------------------------------

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
            "description": "Results per page (max 100).",
            "minimum": 1,
            "maximum": 100,
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        "minimal_output": {
            "type": "boolean",
            "description": "Return minimal repository info (default true).",
        },
    },
    "required": ["query"],
}

# ---------------------------------------------------------------------------
# Issue tools
# ---------------------------------------------------------------------------

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
            "description": "Results per page (max 100).",
            "minimum": 1,
            "maximum": 100,
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
            "description": "Results per page (max 100).",
            "minimum": 1,
            "maximum": 100,
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
            "description": "Results per page (max 100).",
            "minimum": 1,
            "maximum": 100,
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
    },
    "required": ["query"],
}

# ---------------------------------------------------------------------------
# Discussion tools
# ---------------------------------------------------------------------------

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
            "description": "Results per page (max 100).",
            "minimum": 1,
            "maximum": 100,
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
            "description": "Results per page (max 100).",
            "minimum": 1,
            "maximum": 100,
        },
        "after": {"type": "string", "description": "Cursor for pagination."},
    },
    "required": ["owner"],
}

# ---------------------------------------------------------------------------
# Pull request tools
# ---------------------------------------------------------------------------

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
            "description": "Results per page (max 100).",
            "minimum": 1,
            "maximum": 100,
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
            "description": "Results per page (max 100).",
            "minimum": 1,
            "maximum": 100,
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
            "description": "Results per page (max 100).",
            "minimum": 1,
            "maximum": 100,
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
    },
    "required": ["query"],
}

# ---------------------------------------------------------------------------
# Commit tools
# ---------------------------------------------------------------------------

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
            "description": "Results per page (max 100).",
            "minimum": 1,
            "maximum": 100,
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
            "description": "Results per page (max 100).",
            "minimum": 1,
            "maximum": 100,
        },
        "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
    },
    "required": ["owner", "repo"],
}

# ---------------------------------------------------------------------------
# Project tools
# ---------------------------------------------------------------------------

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
            "description": "Results per page (max 50).",
            "minimum": 1,
            "maximum": 50,
        },
        "after": {"type": "string", "description": "Forward pagination cursor."},
        "before": {"type": "string", "description": "Backward pagination cursor."},
    },
    "required": ["method", "owner"],
}


# ---------------------------------------------------------------------------
# Bridge
# ---------------------------------------------------------------------------

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


def register_github_tools(
    registry: ToolRegistry, bridge: "GitHubBridge | None" = None
) -> None:
    """Register read-only GitHub research tools."""
    bridge = bridge or GitHubBridge()

    # -- File / code ----------------------------------------------------------
    bridge.register_tool(
        registry,
        name="github-get-file",
        remote_tool="get_file_contents",
        title="GitHub get file contents",
        description=(
            "Read a file or directory listing from a GitHub repository.\n\n"
            "Best for: Fetching source code, configs, and READMEs at any ref or commit."
        ),
        input_schema=_GET_FILE_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )
    bridge.register_tool(
        registry,
        name="github-get-tree",
        remote_tool="get_repository_tree",
        title="GitHub get repository tree",
        description=(
            "List the file tree of a GitHub repository at a given ref.\n\n"
            "Best for: Understanding project layout before reading individual files."
        ),
        input_schema=_GET_TREE_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )
    bridge.register_tool(
        registry,
        name="github-search-code",
        remote_tool="search_code",
        title="GitHub search code",
        description=(
            "Search GitHub code across repositories.\n\n"
            "Best for: Finding specific functions, patterns, or usages across the GitHub ecosystem."
        ),
        input_schema=_SEARCH_CODE_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )
    bridge.register_tool(
        registry,
        name="github-search-commits",
        remote_tool="search_commits",
        title="GitHub search commits",
        description=(
            "Search commit messages on GitHub.\n\n"
            "Best for: Finding commits by message keyword, author, or date across repositories."
        ),
        input_schema=_SEARCH_COMMITS_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )

    # -- Repositories ---------------------------------------------------------
    bridge.register_tool(
        registry,
        name="github-search-repos",
        remote_tool="search_repositories",
        title="GitHub search repositories",
        description=(
            "Search GitHub for repositories matching a query.\n\n"
            "Best for: Discovering projects by name, topic, language, or stars."
        ),
        input_schema=_SEARCH_REPOS_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )

    # -- Issues ---------------------------------------------------------------
    bridge.register_tool(
        registry,
        name="github-issue-read",
        remote_tool="issue_read",
        title="GitHub read issue",
        description=(
            "Read a GitHub issue: body, comments, sub-issues, labels, or parent.\n\n"
            "method: get | get_comments | get_sub_issues | get_parent | get_labels"
        ),
        input_schema=_ISSUE_READ_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )
    bridge.register_tool(
        registry,
        name="github-list-issues",
        remote_tool="list_issues",
        title="GitHub list issues",
        description=(
            "List issues in a GitHub repository with optional filters.\n\n"
            "Best for: Enumerating open or closed issues, filtering by label or state."
        ),
        input_schema=_LIST_ISSUES_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )
    bridge.register_tool(
        registry,
        name="github-search-issues",
        remote_tool="search_issues",
        title="GitHub search issues",
        description=(
            "Search GitHub issues using GitHub's issue search syntax.\n\n"
            "Best for: Finding issues by keyword, author, label, or state across repositories."
        ),
        input_schema=_SEARCH_ISSUES_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )

    # -- Discussions ----------------------------------------------------------
    bridge.register_tool(
        registry,
        name="github-get-discussion",
        remote_tool="get_discussion",
        title="GitHub get discussion",
        description=(
            "Get the body and metadata of a single GitHub Discussion.\n\n"
            "Best for: Reading a specific community discussion or Q&A thread."
        ),
        input_schema=_GET_DISCUSSION_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )
    bridge.register_tool(
        registry,
        name="github-get-discussion-comments",
        remote_tool="get_discussion_comments",
        title="GitHub get discussion comments",
        description=(
            "Get comments for a GitHub Discussion, optionally including nested replies.\n\n"
            "Best for: Reading community feedback, answers, and Q&A responses."
        ),
        input_schema=_GET_DISCUSSION_COMMENTS_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )
    bridge.register_tool(
        registry,
        name="github-list-discussions",
        remote_tool="list_discussions",
        title="GitHub list discussions",
        description=(
            "List GitHub Discussions for a repository or organisation.\n\n"
            "Best for: Browsing community discussions, optionally filtered by category."
        ),
        input_schema=_LIST_DISCUSSIONS_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )

    # -- Pull requests --------------------------------------------------------
    bridge.register_tool(
        registry,
        name="github-pr-read",
        remote_tool="pull_request_read",
        title="GitHub read pull request",
        description=(
            "Read details of a GitHub Pull Request: body, diff, files, commits, "
            "reviews, or comments.\n\n"
            "method: get | get_diff | get_status | get_files | get_commits | "
            "get_review_comments | get_reviews | get_comments | get_check_runs"
        ),
        input_schema=_PR_READ_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )
    bridge.register_tool(
        registry,
        name="github-list-prs",
        remote_tool="list_pull_requests",
        title="GitHub list pull requests",
        description=(
            "List pull requests in a GitHub repository.\n\n"
            "Best for: Enumerating open or merged PRs with optional state and base-branch filters."
        ),
        input_schema=_LIST_PRS_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )
    bridge.register_tool(
        registry,
        name="github-search-prs",
        remote_tool="search_pull_requests",
        title="GitHub search pull requests",
        description=(
            "Search GitHub pull requests using GitHub's PR search syntax.\n\n"
            "Best for: Finding PRs by keyword, author, state, or label across repositories."
        ),
        input_schema=_SEARCH_PRS_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )

    # -- Commits --------------------------------------------------------------
    bridge.register_tool(
        registry,
        name="github-get-commit",
        remote_tool="get_commit",
        title="GitHub get commit",
        description=(
            "Get details of a single GitHub commit including changed files.\n\n"
            "Best for: Inspecting what changed in a specific commit."
        ),
        input_schema=_GET_COMMIT_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )
    bridge.register_tool(
        registry,
        name="github-list-commits",
        remote_tool="list_commits",
        title="GitHub list commits",
        description=(
            "List commits in a GitHub repository, optionally filtered by author, path, or date.\n\n"
            "Best for: Reviewing recent history or changes to a specific file."
        ),
        input_schema=_LIST_COMMITS_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )

    # -- Projects -------------------------------------------------------------
    bridge.register_tool(
        registry,
        name="github-projects-get",
        remote_tool="projects_get",
        title="GitHub get project",
        description=(
            "Get details of a GitHub Project or one of its fields, items, or status updates.\n\n"
            "method: get_project | get_project_field | get_project_item | get_project_status_update"
        ),
        input_schema=_PROJECTS_GET_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )
    bridge.register_tool(
        registry,
        name="github-projects-list",
        remote_tool="projects_list",
        title="GitHub list projects",
        description=(
            "List GitHub Projects resources: projects, fields, items, or status updates.\n\n"
            "method: list_projects | list_project_fields | list_project_items | "
            "list_project_status_updates"
        ),
        input_schema=_PROJECTS_LIST_SCHEMA,
        output_schema=_CONTENT_OUTPUT,
        annotations=_RO,
    )
