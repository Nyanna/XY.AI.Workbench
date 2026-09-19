package xy.ai.workbench.connector.mcp;

import java.util.UUID;

import org.eclipse.core.runtime.IProgressMonitor;
import org.eclipse.core.runtime.jobs.Job;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.EditorInterface;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.commands.CallCommand;
import xy.ai.workbench.commands.CallEditCommand;
import xy.ai.workbench.commands.Command;
import xy.ai.workbench.commands.ExitCommand;
import xy.ai.workbench.commands.ToolCommand;
import xy.ai.workbench.connector.IAIConnector;
import xy.ai.workbench.connector.harness.Prompt;
import xy.ai.workbench.models.AIAnswer;

public class MCPConnector implements IAIConnector<MCPRequest, MCPResponse> {
	private final MCPClient client;
	private final MCPControlClient control = new MCPControlClient();

	@SuppressWarnings("unused")
	private final ConfigManager cfg;

	public MCPConnector(ConfigManager cfg, MCPClient client) {
		this.cfg = cfg;
		this.client = client;
	}

	@Override
	public KeyPattern getSupportedKeyPattern() {
		return KeyPattern.Misc;
	}

	@Override
	public MCPRequest createRequest(Prompt prompt, IProgressMonitor mon) {
		Command command = prompt.arg.command;
		if (!(command instanceof ExitCommand) && !(command instanceof ToolCommand) && !(command instanceof CallCommand)
				&& !(command instanceof CallEditCommand))
			throw new IllegalArgumentException(String.format("No command detected. [%s]",
					command == null ? "none" : command.getClass().getSimpleName()));
		return new MCPRequest(UUID.randomUUID().toString(), prompt.config.systemPrompt, prompt.config.tools, command,
				prompt.arg.yaml);
	}

	@Override
	public MCPResponse executeRequest(MCPRequest req, IProgressMonitor mon, Job job) {
		if (req.command instanceof ExitCommand) {
			client.close();
			return new MCPResponse(req.id, "MCP connection closed");
		}
		if (req.command instanceof ToolCommand tc) {
			JsonNode tool = client.findTool(tc.parameter(0));
			if (tool == null)
				throw new IllegalArgumentException("Unknown tool: " + tc.parameter(0));
			return new MCPResponse(req.id, control.renderSchema(tool));
		}
		if (req.command instanceof CallCommand) {
			if (req.yamlBlock == null || req.yamlBlock.isBlank())
				throw new IllegalArgumentException("No preceding ```yaml tool call block found before /call");
			return invokeCall(req.id, control.parseYaml(req.yamlBlock));
		}
		if (req.command instanceof CallEditCommand) {
			if (req.command.parameter(0) == null || req.command.parameter(0).isBlank())
				throw new IllegalArgumentException("No ```yaml tool call block found");
			return invokeCall(req.id, control.parseYaml(req.command.parameter(0)));
		}
		throw new UnsupportedOperationException("Unsupported command: " + req.command.prefix());
	}

	private MCPResponse invokeCall(String id, JsonNode call) {
		String name = call.path("tool").asText(null);
		if (name == null || name.isBlank())
			throw new IllegalArgumentException("Tool call is missing the 'tool' field");
		JsonNode tool = client.findTool(name);
		JsonNode arguments = control.fillReason(tool, call.path("arguments"));
		JsonNode result = client.callTool(name, arguments, call.path("id"));

		ObjectNode payload = ((ObjectNode) result).deepCopy();
		JsonNode toolCallId = payload.remove("id");
		return new MCPResponse(id, EditorInterface.TOOLRESULT + "\n" + control.prettyToolResult(toolCallId, payload));
	}

	@Override
	public AIAnswer convertResponse(MCPResponse resp, IProgressMonitor mon) {
		AIAnswer answer = new AIAnswer(resp.id);
		answer.answer = resp.resultText;
		return answer;
	}

}
