package xy.ai.workbench.connector;

import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Collections;
import java.util.List;

import xy.ai.workbench.AgentProfile;
import xy.ai.workbench.CacheMode;
import xy.ai.workbench.Model;
import xy.ai.workbench.Reasoning;

public class SessionParameters {

	public final Path cwd;
	public final String systemPrompt;
	public final List<String> tools;
	public final Model model;
	public final Reasoning reasoning;
	public final AgentProfile agentProfile;
	public final String filePath;
	public final CacheMode cacheMode;
	public final Double topP;
	public final Double temperature;
	public final Integer maxOutputTokens;
	private String hash;

	public SessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,
			AgentProfile agentProfile, CacheMode cacheMode, String filePath, Double topP, Double temperature,
			Integer maxOutputTokens) {
		if (cwd == null)
			throw new IllegalStateException("Work directory (cwd) not set");
		if (model == null)
			throw new IllegalArgumentException("Model must not be null");
		if (model.apiName == null || model.apiName.isBlank())
			throw new IllegalArgumentException("Model apiName must not be null or blank");
		if (reasoning == null)
			throw new IllegalArgumentException("Reasoning must not be null");

		this.cwd = cwd;
		this.systemPrompt = systemPrompt != null ? systemPrompt : "";
		this.tools = tools != null ? tools : Collections.emptyList();
		this.model = model;
		this.reasoning = reasoning;
		this.agentProfile = agentProfile;
		this.filePath = filePath;
		this.cacheMode = cacheMode;
		this.topP = topP;
		this.temperature = temperature;
		this.maxOutputTokens = maxOutputTokens;
	}

	public String getHash() {
		if (hash == null)
			hash = computeHash();
		return hash;
	}

	protected String computeHash() {
		String input = String.join(",", tools) + "|" + cwd.toString() + "|" + model.apiName + "|" + reasoning.name()
				+ "|" + (agentProfile != null ? agentProfile.name : "") + "|" + (filePath != null ? filePath : "")
				+ (topP != null ? topP : "") + (temperature != null ? temperature : "")
				+ (maxOutputTokens != null ? maxOutputTokens : "");
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
