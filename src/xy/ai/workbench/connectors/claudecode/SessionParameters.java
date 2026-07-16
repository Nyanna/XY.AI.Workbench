package xy.ai.workbench.connectors.claudecode;

import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

import xy.ai.workbench.AgentProfile;
import xy.ai.workbench.CacheMode;
import xy.ai.workbench.Model;
import xy.ai.workbench.Reasoning;

public class SessionParameters {
	private static final String SCRIPT = System.getProperty("user.home")
			+ "/xyan/xy.ai.workbench/claude-code/claude-session.sh";
	private static final String COMMAND = "claude";

	/** Deterministic hash of the session parameters. Immutable. */
	public final Path cwd;
	public final String systemPrompt;
	public final List<String> tools;
	public final Model model;
	public final Reasoning reasoning;
	public final AgentProfile agentProfile;
	public final String cliProfile;
	public final String filePath;
	public final CacheMode cacheMode;
	private String hash;
	private String title;

	public SessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,
			AgentProfile agentProfile, String cliProfile, CacheMode cacheMode) {
		this(cwd, systemPrompt, tools, model, reasoning, agentProfile, cliProfile, cacheMode, null);
	}

	public SessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,
			AgentProfile agentProfile, String cliProfile, CacheMode cacheMode, String filePath) {
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
		this.cliProfile = cliProfile;
		this.filePath = filePath;
		this.cacheMode = cacheMode;
	}

	public String getFilePath() {
		return filePath;
	}

	public List<String> buildBaseCommand() {
		List<String> cmd = new ArrayList<>();
		if (AgentProfile.MCPC.equals(agentProfile)) {
			cmd.add(COMMAND);
			cmd.add("--system-prompt");
			cmd.add(systemPrompt);
			cmd.add("--tools");
			cmd.add(""); // restrict builtin tools
			// evil: breaks STDIN handling
//			cmd.add("--debug");
//			cmd.add("mcp");
			cmd.add("--settings");
			cmd.add("""
					{
						"hooks": {
							"PreToolUse": [
								{
									"hooks": [
										{
											"type": "http",
											"url":"http://localhost:9093/hooks/tool",
											"headers":{
											   "X-MCPC-SESSION-ID":"$MCPC_SESSION_ID"
											},
											"allowedEnvVars":[
											   "MCPC_SESSION_ID"
											],
											"timeout": 86400
										}
									]
								}
							],
							"PermissionRequest": [
								{
									"hooks": [
										{
											"type": "http",
											"url":"http://localhost:9093/hooks/permission",
											"headers":{
											   "X-MCPC-SESSION-ID":"$MCPC_SESSION_ID"
											},
											"allowedEnvVars":[
											   "MCPC_SESSION_ID"
											],
											"timeout": 86400
										}
									]
								}
							]
						}
					}
										""".replace("\t", "").replace("\n", "").strip());
			cmd.add("--mcp-config");
			cmd.add("""
					{
						"mcpServers": {
							"mcpc": {
								"type": "ws",
								"url": "http://localhost:9094/mcp",
								"timeout": 86400000,
								"alwaysLoad": true,
								"headers": {
									"X-MCPC-SESSION-ID": "${MCPC_SESSION_ID}",
									"X-MCPC-TOOLS": "${MCPC_TOOLS}",
									"X-MCPC-CC-PROFILE": "${MCPC_CC_PROFILE}"
								}
							}
						}
					}
										""".replace("\t", "").replace("\n", "").strip());
		} else {
			cmd.add(SCRIPT);
			cmd.add(agentProfile != null ? agentProfile.name : ""); // Agent definition
			cmd.add("--profile");
			cmd.add(cliProfile);
		}

		cmd.add("--verbose");
		// replaced by MCPC
		// cmd.add("--include-hook-events");
		cmd.add("--include-partial-messages");
		cmd.add("--input-format");
		cmd.add("stream-json");
		cmd.add("--output-format");
		cmd.add("stream-json");
		// replaced by self mirror input
		// cmd.add("--replay-user-messages");
		cmd.add("--model");
		cmd.add(model.apiName);
		if (Reasoning.Disabled != reasoning) {
			cmd.add("--effort");
			cmd.add(reasoning.name().toLowerCase());
		}
		cmd.add("--dangerously-skip-permissions"); // as long there is no permission prompt handling implemented
		return cmd;
	}

	public void buildEvironment(ProcessBuilder pb) {
		pb.directory(cwd.toFile());
		if (AgentProfile.MCPC.equals(agentProfile)) {
			if (!tools.isEmpty())
				pb.environment().put("MCPC_TOOLS", String.join(",", tools));
			else
				pb.environment().put("MCPC_TOOLS", "None");
			pb.environment().put("MCPC_CC_PROFILE", cliProfile);
			pb.environment().put("CLAUDE_CONFIG_DIR", System.getProperty("user.home") + "/.claude-" + cliProfile);

			pb.environment().put("CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS", "1");
			pb.environment().put("CLAUDE_CODE_DISABLE_AGENT_VIEW", "1");
			pb.environment().put("CLAUDE_CODE_DISABLE_BACKGROUND_TASKS", "1");
			pb.environment().put("CLAUDE_CODE_DISABLE_BUNDLED_SKILLS", "1");
			pb.environment().put("CLAUDE_CODE_DISABLE_CLAUDE_MDS", "1");
			pb.environment().put("CLAUDE_CODE_DISABLE_CRON", "1");
			pb.environment().put("CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS", "1");
			pb.environment().put("CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS", "1");
			pb.environment().put("CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC", "1");
			pb.environment().put("CLAUDE_CODE_DISABLE_POLICY_SKILLS", "1");
			pb.environment().put("CLAUDE_CODE_DISABLE_WORKFLOWS", "1");
			pb.environment().put("CLAUDE_CODE_ENABLE_AWAY_SUMMARY", "0");
			pb.environment().put("CLAUDE_CODE_ENABLE_BACKGROUND_PLUGIN_REFRESH", "1");
			pb.environment().put("CLAUDE_CODE_FORK_SUBAGENT", "0");
			pb.environment().put("CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY", "1"); // number of parralel read tools
			pb.environment().put("ENABLE_TOOL_SEARCH", "false");
		}

		if (cacheMode != null)
			switch (cacheMode) {
			case Disabled:
				pb.environment().put("DISABLE_PROMPT_CACHING", "1");
				break;
			case Minutes_5:
				pb.environment().put("FORCE_PROMPT_CACHING_5M", "1");
				break;
			case Hours_1:
				pb.environment().put("ENABLE_PROMPT_CACHING_1H", "1");
			case Default:
			default:
			}
		pb.environment().put("CLAUDE_CODE_DISABLE_SPELLCHECK", "true");
		pb.environment().put("CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT", "0");
		pb.environment().put("MCP_TOOL_TIMEOUT", "86400000");
		pb.environment().put("CLAUDE_ENABLE_STREAM_WATCHDOG", "0");
		pb.environment().put("CLAUDE_ENABLE_BYTE_WATCHDOG", "0");
		pb.environment().put("CLAUDE_STREAM_IDLE_TIMEOUT_MS", "86400000");
		pb.environment().put("API_FORCE_IDLE_TIMEOUT", "0");
		pb.environment().put("MCP_TIMEOUT", "86400000");

		if (Reasoning.Disabled == reasoning) {
			pb.environment().put("CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING", "1");
			pb.environment().put("MAX_THINKING_TOKENS", "0");
		}
		pb.environment().put("CLAUDE_CODE_DISABLE_ADVISOR_TOOL", "1");
	}

	public String getHash() {
		if (hash == null)
			hash = computeHash();
		return hash;
	}

	private String computeHash() {
		String input = systemPrompt.toString() + "|" + String.join(",", tools) + "|" + cwd.toString() + "|"
				+ model.apiName + "|" + reasoning.name() + "|" + (agentProfile != null ? agentProfile.name : "") + "|"
				+ cliProfile + "|" + (filePath != null ? filePath : "");
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

	public void setTitle(String title) {
		if (this.title == null)
			this.title = title;
	}

	public String getTitle() {
		return title;
	}
}
