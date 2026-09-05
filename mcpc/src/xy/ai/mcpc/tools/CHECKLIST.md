# Tools Checkliste

Verify for new tools MCP part.

* Input-schema is present and complete and conform to MCP specification
* Output-schema is present and complete and conform to MCP specification
* Schema was checked for consistency because this will break clients
* Result uses structuredContent, only error output uses content
* AI Instructions and descriptions should be short, clear, and distinct
* Check input and output for technical limits; reduce input and output as much as semantically meaningfull
* Never reflect input paramaeters, not even in error output
* On MCP passthrough always reduce limits like results on pages
* Tools should be as hallucination tolerant as possible and guess and correct/anticipate input

For Python module direct call:

* Implementation conforms to the other tools
* Docstrings should match the MCP scheme
* Parameter validation like min/max should be implemented in code not just by scheme