---
description: General and default Agent settings
tools: ""
model: Haiku
effort: low
thinking: false
color: cyan
tool_deny:
  redirect:
    ToolSearch: "Using ToolSearch indicates missing instructions in the prompt. Ask the user how to proceed."
    AskUserQuestion: "Do not use this interactive tool. If information is missing, ask one direct and specific question or instruct the user to extend their prompt. Wizard-style questioning is inefficient; if a user cannot answer without guided prompting, it indicates they may not fully understand the problem they are asking to solve."
    "/Task.*/": "Task tools are disabled. Instruct the user and explain why you need this tool."
    "/Cron.*/": "Cron tools are disabled. Instruct the user and explain why you need this tool."
    Workflow: "Workflow is disabled. Background processing is not available. Report this and ask the user for instructions. Explain what you were trying to achieve."
    "Bash(git commit *)": "You are not allowed to commit. Instruct the user to review the changes and commit manually."
    "Bash(git commit)": "You are not allowed to commit. Instruct the user to review the changes and commit manually."
    "Bash(git push *)": "You are not allowed to push. Instruct the user to push manually."
    "Bash(git push)": "You are not allowed to push. Instruct the user to push manually."
    "Bash(git add *)": "You are not allowed to stage files. Instruct the user to review the changes and add files manually."
    "Bash(git add)": "You are not allowed to stage files. Instruct the user to review the changes and add files manually."
    "Bash(git merge *)": "You are not allowed to merge. Instruct the user to perform the merge manually."
    "Bash(git rebase *)": "You are not allowed to modify the repository state. Instruct the user to perform this operation manually."
    "Bash(git reset *)": "You are not allowed to modify the repository state. Instruct the user to perform this operation manually."
    "Bash(git revert *)": "You are not allowed to modify the repository state. Instruct the user to perform this operation manually."
    "Bash(git rm *)": "You are not allowed to remove files from the repository. Instruct the user which files to remove manually."
    "Bash(git mv *)": "You are not allowed to move files. Instruct the user to move the files manually."
    "Bash(git tag *)": "You are not allowed to modify the repository state. Instruct the user to perform this operation manually."
    "Bash(git stash *)": "You are not allowed to modify the repository state. Instruct the user to perform this operation manually."
    "Bash(git cherry-pick *)": "You are not allowed to modify the repository state. Instruct the user to perform this operation manually."
    "Bash(git fetch *)": "You are not allowed to fetch. Instruct the user to fetch manually."
    "Bash(git pull *)": "You are not allowed to pull. Instruct the user to pull manually."
    "Bash(git restore *)": "You are not allowed to modify the repository state. Instruct the user to perform this operation manually."
    "Bash(git switch *)": "You are not allowed to modify the repository state. Instruct the user to perform this operation manually."
    "Bash(git checkout *)": "You are not allowed to modify the repository state. Instruct the user to perform this operation manually."
    "Bash(git branch -d *)": "You are not allowed to modify the repository state. Instruct the user to perform this operation manually."
    "Bash(git branch -D *)": "You are not allowed to modify the repository state. Instruct the user to perform this operation manually."
    "Bash(git branch -m *)": "You are not allowed to modify the repository state. Instruct the user to perform this operation manually."
    "Bash(git clean *)": "You are not allowed to modify the repository state. Instruct the user to perform this operation manually."
    "Bash(curl *)": "Use the 'web-research:web-research' agent instead."
  deny:
    Agent: "The agent you are trying to use is not permitted. Inform the user about this restriction and specify which agent you attempted to use."
---
