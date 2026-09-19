package xy.ai.workbench.connector.harness;

import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.List;

import xy.ai.workbench.connector.harness.PromptInputHandler.PromptArguments;

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
		try {
			MessageDigest md = MessageDigest.getInstance("MD5");
			byte[] bytes = md.digest(input.getBytes(StandardCharsets.UTF_8));
			StringBuilder sb = new StringBuilder();
			for (byte b : bytes)
				sb.append(String.format("%02x", b));
			return sb.substring(0, 8);
		} catch (NoSuchAlgorithmException e) {
			// Stable fallback (no external dependency)
			long h = 0;
			for (char c : input.toCharArray())
				h = h * 31L + c;
			return String.format("%08x", h & 0xFFFFFFFFL);
		}
	}
}
