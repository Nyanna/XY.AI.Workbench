package xy.ai.workbench.connector.claudecode;

import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

import xy.ai.workbench.AgentProfile;
import xy.ai.workbench.CacheMode;
import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model;
import xy.ai.workbench.Reasoning;
import xy.ai.workbench.connector.SessionParameters;
import xy.ai.workbench.connector.harness.FrozenConfig;
import xy.ai.workbench.tools.Hash;

public class ClaudeSessionParameters extends SessionParameters {
	private static final String SCRIPT = System.getProperty("user.home")
			+ "/xyan/xy.ai.workbench/claude-code/claude-session.sh";
	private static final String COMMAND = "claude";
	private String title;
	public final String cliProfile;
	private String hash;

	public ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,
			AgentProfile agentProfile, String cliProfile, CacheMode cacheMode, String filePath) {
		super(cwd, systemPrompt, tools, model, reasoning, agentProfile, cacheMode, filePath, null, null, null);
		this.cliProfile = cliProfile;
	}

	public boolean equals(ConfigManager cfg, Path cwd, String filePath, List<String> tools) {
		FrozenConfig fc = FrozenConfig.from(cfg);
		return fc.keys.contains(cliProfile) //
				&& equalsConfig(fc, cwd, filePath, tools);
	}

	@Override
	public String getHash() {
		if (hash == null)
			hash = computeHash();
		return hash;
	}

	protected String computeHash() {
		String input = cliProfile + "|" + super.computeHash();
		return Hash.hash(input);
	}

	public List<String> buildBaseCommand() {
		List<String> cmd = new ArrayList<>();
		if (AgentProfile.MCPC.equals(agentProfile)) {
			cmd.add(COMMAND);
			cmd.add("--system-prompt");
			cmd.add(systemPrompt != null ? systemPrompt : "");
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
			if (!tools.isEmpty()) {
				if (CacheMode.Disabled.equals(cacheMode))
					throw new IllegalArgumentException("Cache is required for tool loops");
				pb.environment().put("MCPC_TOOLS", String.join(",", tools));
			} else
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
			pb.environment().put("MAX_MCP_OUTPUT_TOKENS", "" + (25000 * 10));
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

	public void setTitle(String title) {
		if (this.title == null)
			this.title = title;
	}

	public String getTitle() {
		return title;
	}
}
