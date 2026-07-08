Implementiere einen Permission-Hook analog zum ToolUse-Hook `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/hooks.py`. 

* Der Request Path ist "/hooks/permission",
* Hook implementiert "PermissionRequest".
* Der Hook erlaubt pauschal und unverändert alle eingehenden Anfragen.

# PermissionRequest Response

`PermissionRequest` hooks can allow or deny permission requests. In addition to the [JSON output fields](#json-output) available to all hooks, your hook script can return a `decision` object with these event-specific fields:

| Field                | Description                                                                                                                                                                                                                     |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `behavior`           | `"allow"` grants the permission, `"deny"` denies it. [Deny and ask rules](/docs/en/permissions#manage-permissions) are still evaluated, so a hook returning `"allow"` doesn’t override a matching deny rule                     |
| `updatedInput`       | For `"allow"` only: modifies the tool’s input parameters before execution. Replaces the entire input object, so include unchanged fields alongside modified ones. The modified input is re-evaluated against deny and ask rules |
| `updatedPermissions` | For `"allow"` only: array of [permission update entries](#permission-update-entries) to apply, such as adding an allow rule or changing the session permission mode                                                             |
| `message`            | For `"deny"` only: tells Claude why the permission was denied                                                                                                                                                                   |
| `interrupt`          | For `"deny"` only: if `true`, stops Claude                                                                                                                                                                                      |

```
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedInput": {
        "command": "npm run lint"
      }
    }
  }
}
```