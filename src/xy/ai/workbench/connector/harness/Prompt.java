package xy.ai.workbench.connector.harness;

import java.util.List;

import xy.ai.workbench.tools.Hash;

/**
 * Immutable, frozen prompt: semantically separated Inputs / Batch / Config /
 * Arguments, plus the activation state for the SessionProcessor.
 */
public class Prompt {
	public final List<String> inputs;
	public final boolean batch;
	public final FrozenConfig config;
	public final PromptArguments arg;

	private String sessionId;

	public Prompt(List<String> inputs, boolean batch, FrozenConfig config, PromptArguments arg) {
		if (config == null)
			throw new IllegalArgumentException("config must not be null");
		if (arg.absoluteFilePath == null || arg.absoluteFilePath.isBlank())
			throw new IllegalArgumentException("absoluteFilePath must not be blank");
		if (arg.project == null)
			throw new IllegalArgumentException("projectPath must not be null");
		this.inputs = inputs != null ? List.copyOf(inputs) : List.of();
		this.batch = batch;
		this.config = config;
		this.arg = arg;
	}

	/**
	 * Deterministic session id: hash(absoluteFilePath, batch, config.getHash()).
	 */
	public String sessionId() {
		if (sessionId == null)
			sessionId = computeSessionId();
		return sessionId;
	}

	private String computeSessionId() {
		String input = arg.absoluteFilePath + "|" + batch + "|" + config.getHash();
		return Hash.hash(input);
	}
}
