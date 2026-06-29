Plugin needs a Github authorized PAT MCP configuration.

```JSON

      "mcpServers": {
        "github": {
          "type": "http",
          "url": "https://api.githubcopilot.com/mcp",
          "headers": {
            "Authorization": "Bearer github_pat_*"
          }
        }
      }
```