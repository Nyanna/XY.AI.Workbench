package xy.ai.workbench.connector;

import java.nio.file.Path;
import java.util.Collections;
import java.util.List;

import com.google.api.client.util.Objects;

import xy.ai.workbench.AgentProfile;
import xy.ai.workbench.CacheMode;
import xy.ai.workbench.Model;
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.connector.harness.FrozenConfig;
import xy.ai.workbench.tools.Hash;

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
		this.systemPrompt = systemPrompt;
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

	public boolean equalsConfig(FrozenConfig cfg, Path cwd, String filePath, List<String> tools) {
		return Objects.equal(cwd, this.cwd) //
				&& Objects.equal(cfg.systemPrompt, systemPrompt) //
				&& Objects.equal(tools, this.tools) //
				&& Objects.equal(cfg.model, model) //
				&& Objects.equal(cfg.reasoning, reasoning) //
				&& Objects.equal(cfg.profile, agentProfile) //
				&& Objects.equal(filePath, this.filePath) //
				&& Objects.equal(cfg.cacheMode, cacheMode);
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
		return Hash.hash(input);
	}

}
