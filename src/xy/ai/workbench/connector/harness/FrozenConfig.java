package xy.ai.workbench.connector.harness;

import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Arrays;
import java.util.List;

import xy.ai.workbench.AgentProfile;
import xy.ai.workbench.CacheMode;
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.InputMode;
import xy.ai.workbench.Model;
import xy.ai.workbench.OutputMode;
import xy.ai.workbench.Reasoning;

/**
 * Immutable snapshot of the {@link ConfigManager} state, taken once when a
 * {@link Prompt} is frozen. The system prompt is rendered eagerly so that later
 * changes to the live config do not affect already-frozen sessions.
 */
public class FrozenConfig {

	public Model model;
	public String keys;
	public final AgentProfile profile;
	public final Reasoning reasoning;
	public final List<String> tools;
	public final String systemPrompt;
	public final Double topP;
	public final Double temperature;
	public final Long maxOutputTokens;
	public final Integer reasoningBudget;
	public final CacheMode cacheMode;
	public final OutputMode outputMode;

	private String hash;

	private FrozenConfig(String keys, Model model, AgentProfile profile, Reasoning reasoning, List<String> tools,
			String systemPrompt, Double topP, Double temperature, Long maxOutputTokens, Integer reasoningBudget,
			CacheMode cacheMode, OutputMode outputMode) {
		this.keys = keys;
		this.model = model;
		this.profile = profile;
		this.reasoning = reasoning;
		this.tools = tools;
		this.systemPrompt = systemPrompt;
		this.topP = topP;
		this.temperature = temperature;
		this.maxOutputTokens = maxOutputTokens;
		this.reasoningBudget = reasoningBudget;
		this.cacheMode = cacheMode;
		this.outputMode = outputMode;
	}

	/**
	 * Reads all relevant values from {@code cfg} once and renders the system
	 * prompt.
	 */
	public static FrozenConfig from(ConfigManager cfg) {
		String systemPrompt = null;
		if (cfg.isInputEnabled(InputMode.SystemPrompt)) {
			String[] rawPrompt = cfg.getSystemPrompt();
			StringBuilder sb = new StringBuilder();
			Arrays.stream(rawPrompt != null ? rawPrompt : new String[0]).filter(e -> !e.startsWith("#"))
					.forEach(e -> sb.append("- ").append(e).append("\n"));
			String freeText = cfg.getFreeText();
			if (freeText != null && !freeText.isBlank())
				sb.append("\n- ").append(freeText).append("\n");
			systemPrompt = sb.toString();
		}

		String[] rawTools = cfg.isInputEnabled(InputMode.Tools) ? cfg.getTools() : null;
		List<String> tools = List.of(rawTools != null ? rawTools : new String[0]);

		String keys = cfg.getKeys();
		for (String key : keys.split(","))
			if (cfg.getModel().cap.acceptsKey(key))
				keys = key;

		return new FrozenConfig(keys, cfg.getModel(), cfg.getProfile(), cfg.getReasoning(), tools, systemPrompt,
				cfg.getTopP(), cfg.getTemperature(), cfg.getMaxOutputTokens(), cfg.getReasoningBudget(),
				cfg.getCacheMode(), cfg.getOuputMode());
	}

	/**
	 * Deterministic hash over exactly: model, profile, reasoning, tools,
	 * systemPrompt, topP, temperature, maxOutputTokens, reasoningBudget.
	 */
	public String getHash() {
		if (hash == null)
			hash = computeHash();
		return hash;
	}

	private String computeHash() {
		String input = (model != null ? model.apiName : "") + "|" + (profile != null ? profile.name : "") + "|"
				+ (reasoning != null ? reasoning.name() : "") + "|" + String.join(",", tools) + "|" + systemPrompt + "|"
				+ topP + "|" + temperature + "|" + maxOutputTokens + "|" + reasoningBudget;
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
