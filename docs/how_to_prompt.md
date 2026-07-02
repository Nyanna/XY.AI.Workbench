# How-To Prompt

* Best prompt pattern is a structure soft-prompt
* Spell corrections increases quality
* Use code blocks with shell name for prompts: ```bash
* Use absolute file path with `/path/x.md`
* Use external prompt injection single @/path/to/prompt
* Always use a plan stage for big changes, if context explodes or crash
* Complete a task withing 1h/5m cuz Prompt prefix caching (pro/api)
	* caching works just up to 20 blocks/steps back
* Code änderung immer über proccess Feature Analye -> Plan -> Ausführung (jeweiligen Kontexte klein und isoliert halten)
* For exports use /copy and copy outputfile
* Set Thinking Effort based on exspected input/output and total estimated token cost