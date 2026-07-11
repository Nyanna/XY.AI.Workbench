Die Implementierung in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/ClaudeCodeJsonParser.java` verarbeitet den Fehlertext nicht. Dieser muss gejoined und zurückgegeben werden.


```json
{
  "type": "result",
  "subtype": "error_during_execution",
  "duration_ms": 0,
  "duration_api_ms": 0,
  "is_error": true,
  "num_turns": 0,
  "stop_reason": null,
  "session_id": "51281d65-c289-43c6-97d7-82a6b38168e1",
  "total_cost_usd": 0,
  "usage": {
    "input_tokens": 0,
    "cache_creation_input_tokens": 0,
    "cache_read_input_tokens": 0,
    "output_tokens": 0,
    "server_tool_use": {
      "web_search_requests": 0,
      "web_fetch_requests": 0
    },
    "service_tier": "standard",
    "cache_creation": {
      "ephemeral_1h_input_tokens": 0,
      "ephemeral_5m_input_tokens": 0
    },
    "inference_geo": "",
    "iterations": [],
    "speed": "standard"
  },
  "modelUsage": {},
  "permission_denials": [],
  "uuid": "b7916448-0590-4111-85be-22c914d911f9",
  "errors": [
    "No conversation found with session ID: 51281d65-c289-43c6-97d7-82a6b38168e1"
  ]
}
```