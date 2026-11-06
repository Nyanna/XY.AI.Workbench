package xy.ai.conscious.connector.mcp;

import java.util.UUID;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;

import xy.ai.conscious.api.IExecutor.IProgress;
import xy.ai.conscious.commands.CallCommand;
import xy.ai.conscious.commands.CallEditCommand;
import xy.ai.conscious.commands.Command;
import xy.ai.conscious.commands.ExitCommand;
import xy.ai.conscious.commands.ToolCommand;
import xy.ai.conscious.connector.IAIConnector;
import xy.ai.conscious.connector.harness.Prompt;
import xy.ai.conscious.core.IEditorInterface;
import xy.ai.conscious.model.AIAnswer;
import xy.ai.conscious.model.Model.KeyPattern;

public class MCPConnector implements IAIConnector<MCPRequest, MCPResponse> {
	private final MCPClient client;
	private final MCPControlClient control = new MCPControlClient();

	public MCPConnector(MCPClient client) {
		this.client = client;
	}

	@Override
	public KeyPattern getSupportedKeyPattern() {
		return KeyPattern.Misc;
	}

	@Override
	public MCPRequest createRequest(Prompt prompt, IProgress mon) {
		Command command = prompt.arg.command;
		if (!(command instanceof ExitCommand) && !(command instanceof ToolCommand) && !(command instanceof CallCommand)
				&& !(command instanceof CallEditCommand))
			throw new IllegalArgumentException(String.format("No command detected. [%s]",
					command == null ? "none" : command.getClass().getSimpleName()));
		return new MCPRequest(UUID.randomUUID().toString(), prompt.config.systemPrompt, prompt.config.tools, command,
				prompt.arg.yaml);
	}

	@Override
	public MCPResponse executeRequest(MCPRequest req, IProgress mon) {
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
		return new MCPResponse(id, IEditorInterface.TOOLRESULT + "\n" + control.prettyToolResult(toolCallId, payload));
	}

	@Override
	public AIAnswer convertResponse(MCPResponse resp, IProgress mon) {
		AIAnswer answer = new AIAnswer(resp.id);
		answer.answer = resp.resultText;
		answer.showStats = false;
		return answer;
	}

}
