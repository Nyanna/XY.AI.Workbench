# Findings

* Thinking cost is a quality indicator for prompts. Better specified prompts => lower thinking costs.
	* Thinking cost measures resistance against the prompt. Too high thinking costs indicate the prompt should be optimized and tuned to the model.
* Parallel agents provide little value if you have to verify the results for hours anyway.
* Make distinctions early and interrupt for important matters in the subsequent context; Control and redirect agents early.
* Input has a large context component; focus as much as possible.
* Agents think in terms of resources, i.e., file system-based.
* When extending or referencing tickets, a patch/diff delta as a small, focused input context is better than having the agent re-understand the requirements.
* Better to incur some extra thinking through iteration and tool interruptions than to load too much into the context; the equation is salience versus tokens.
* Separate understanding => separate contexts.
