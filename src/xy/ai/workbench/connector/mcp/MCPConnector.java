package xy.ai.workbench.connector.mcp;

import java.util.List;
import java.util.UUID;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.jobs.Job;

import com.fasterxml.jackson.databind.JsonNode;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.connector.IAIConnector;
import xy.ai.workbench.models.AIAnswer;

public class MCPConnector implements IAIConnector<MCPRequest, MCPResponse> {
	private final MCPClient client = new MCPClient();
	private final MCPControlClient control = new MCPControlClient();

	@SuppressWarnings("unused")
	private final ConfigManager cfg;

	public MCPConnector(ConfigManager cfg) {
		this.cfg = cfg;
	}

	@Override
	public KeyPattern getSupportedKeyPattern() {
		return KeyPattern.Misc;
	}

	@Override
	public MCPRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,
			IProgressMonitor mon) {
		return new MCPRequest(UUID.randomUUID().toString(), systemPrompt, tools, preprocess(inputs));
	}

	@Override
	public MCPResponse executeRequest(MCPRequest req, IProgressMonitor mon, Job job) {
		switch (req.cmd.type) {
		case Exit:
			client.close();
			return new MCPResponse(req.id, "MCP connection closed");
		case Tool: {
			JsonNode tool = client.findTool(req.cmd.parameter);
			if (tool == null)
				throw new IllegalArgumentException("Unknown tool: " + req.cmd.parameter);
			return new MCPResponse(req.id, control.renderSchema(tool));
		}
		case Prompt: {
			String block = control.extractYamlBlock(req.cmd.parameter);
			if (block == null)
				throw new IllegalArgumentException(
						"No ```yaml tool call block found. Use \"/tool <id>\" to get a template.");
			JsonNode call = control.parseYaml(block);
			String name = call.path("tool").asText(null);
			if (name == null || name.isBlank())
				throw new IllegalArgumentException("Tool call is missing the 'tool' field");
			JsonNode tool = client.findTool(name);
			JsonNode arguments = control.fillReason(tool, call.path("arguments"));
			JsonNode result = client.callTool(name, arguments);
			return new MCPResponse(req.id, control.prettyResult(result));
		}
		default:
			throw new UnsupportedOperationException("Unsupported command: " + req.cmd.type);
		}
	}

	@Override
	public AIAnswer convertResponse(MCPResponse resp, IProgressMonitor mon) {
		AIAnswer answer = new AIAnswer(resp.id);
		answer.answer = resp.resultText;
		return answer;
	}

	private MCPCommand preprocess(List<String> inputs) {
		StringBuilder merged = null;
		for (String input : inputs) {
			String clean = input != null ? input.strip() : "";
			if (clean.isBlank())
				continue;
			if ("/exit".equalsIgnoreCase(clean))
				return new MCPCommand(MCPCommandType.Exit, "");
			if (clean.matches("(?i)/tool\\s+\\S+"))
				return new MCPCommand(MCPCommandType.Tool, clean.split("\\s+", 2)[1].strip());
			if (merged == null)
				merged = new StringBuilder();
			else
				merged.append("\n");
			merged.append(clean);
		}
		if (merged == null)
			throw new IllegalStateException("No commands in inputs");
		return new MCPCommand(MCPCommandType.Prompt, merged.toString());
	}
}
