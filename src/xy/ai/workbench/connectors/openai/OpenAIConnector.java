package xy.ai.workbench.connectors.openai;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonMappingException;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.node.TextNode;
import com.openai.client.OpenAIClient;
import com.openai.client.okhttp.OpenAIOkHttpClient;
import com.openai.core.ObjectMappers;
import com.openai.core.http.HttpResponseFor;
import com.openai.models.ChatModel;
import com.openai.models.Reasoning;
import com.openai.models.ReasoningEffort;
import com.openai.models.responses.Response;
import com.openai.models.responses.Response.Instructions;
import com.openai.models.responses.ResponseCreateParams;
import com.openai.models.responses.ResponseCreateParams.Builder;
import com.openai.models.responses.ResponseCreateParams.Truncation;
import com.openai.models.responses.ResponseError;
import com.openai.models.responses.ResponseInputItem;
import com.openai.models.responses.ResponseInputText;
import com.openai.models.responses.ResponseStatus;
import com.openai.models.responses.ResponseUsage;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Model.KeyPattern;
import xy.ai.workbench.connectors.IAIConnector;
import xy.ai.workbench.models.AIAnswer;
import xy.ai.workbench.models.IModelRequest;
import xy.ai.workbench.models.IModelResponse;

public class OpenAIConnector implements IAIConnector {
	private ConfigManager cfg;
	private OpenAIClient client;

	public OpenAIConnector(ConfigManager cfg) {
		this.cfg = cfg;
		cfg.addKeyObs(k -> {
			if (KeyPattern.OpenAI.matches(k))
				this.client = OpenAIOkHttpClient.builder().apiKey(cfg.getKeys()).build();
		}, true);
	}

	@Override
	public IModelRequest createRequest(String input, String systemPrompt, List<String> tools) {
		boolean isBackground = false;

		Builder builder = ResponseCreateParams.builder() //
				.maxOutputTokens(cfg.getMaxOutputTokens())
				.safetyIdentifier(new Random().nextInt(Integer.MAX_VALUE) + "").truncation(Truncation.DISABLED) //
				.maxToolCalls(1)//
				.background(isBackground)//
				.instructions(systemPrompt)//
				.reasoning( //
						Reasoning.builder()//
								.effort(ReasoningEffort.of(cfg.getReasoning().name())) //
								.summary(Reasoning.Summary.AUTO)//
								.build())
				.model(ChatModel.of(cfg.getModel().connectorName)); //

		if (cfg.getCapabilities().isSupportTemperature())
			builder = builder.temperature(cfg.getTemperature());

		if (cfg.getCapabilities().isSupportTopP())
			builder = builder.topP(cfg.getTopP());

		if (input != null && !input.isBlank()) {
			List<ResponseInputItem> inputs = new ArrayList<ResponseInputItem>();
			ResponseInputText inputText = ResponseInputText.builder() //
					.text(input) //
					.build();
			ResponseInputItem inputItem = ResponseInputItem.ofMessage(ResponseInputItem.Message.builder() //
					.role(ResponseInputItem.Message.Role.USER)//
					.addContent(inputText).build());
			inputs.add(inputItem);
			builder = builder.inputOfResponse(inputs);
		}

		if (tools != null && !tools.isEmpty())
			builder = appendTools(builder, tools);

		ResponseCreateParams params = builder.build();
		return new OpenAIModelRequest(params);
	}

	@Override
	public IModelResponse executeRequest(IModelRequest request) {
		ResponseCreateParams params = ((OpenAIModelRequest) request).reqquest;
		boolean isBackground = params.background().orElse(Boolean.FALSE);

		HttpResponseFor<Response> rwResponse = client.responses().withRawResponse().create(params);
//		int statusCode = rwResponse.statusCode();
//		Headers headers = rwResponse.headers();
		Response resp = rwResponse.parse();

		// for background mode
		if (isBackground && resp.status().isPresent()) {
			ResponseStatus status;
			do {
				try {
					Thread.sleep(500);
				} catch (InterruptedException e) {
				}
				status = resp.status().get();
			} while (status.equals(ResponseStatus.QUEUED) || status.equals(ResponseStatus.IN_PROGRESS));
		}
		return new OpenAIModelResponse(resp);
	}

	@Override
	public AIAnswer convertResponse(IModelResponse response) {
		Response resp = ((OpenAIModelResponse) response).response;

		AIAnswer res = new AIAnswer();
		if (resp.usage().isPresent()) {
			ResponseUsage usage = resp.usage().get();
			res.inputToken = usage.inputTokens();
			res.outputToken = usage.outputTokens();
			res.totalToken = usage.totalTokens();
			if (usage.outputTokensDetails() != null)
				res.reasoningToken = usage.outputTokensDetails().reasoningTokens();
		}

		if (resp.instructions().isPresent()) {
			Instructions ins = resp.instructions().get();
			res.instructions = ins.asString();
		}

		if (resp.incompleteDetails().isPresent()) {
			var incomplete = resp.incompleteDetails().get();
			System.out.println("Incomplete: " + incomplete.reason().get().toString() + " ");
		}
		if (resp.error().isPresent()) {
			ResponseError error = resp.error().get();
			res.answer = error.code() + ": " + error.message();
			System.out.println("Error: " + error.code() + " " + error.message());

		} else
			for (var out : resp.output()) {
				if (out.isMessage()) {
					var msg = out.message().get();
					for (var cnt : msg.content()) {
						if (cnt.isOutputText()) {
							String answer = cnt.asOutputText().text();
							// System.out.println("Answer: " + answer);
							res.answer += answer;
						} else if (cnt.isRefusal()) {
							System.out.println("Refusal: " + cnt.asRefusal().refusal());
						}
					}
				} else if (out.isReasoning()) {
					for (var cnt : out.asReasoning().summary()) {
						System.out.println("Reasoning summary: " + cnt.text());
					}
					if (out.asReasoning().content().isPresent())
						for (var cnt : out.asReasoning().content().get()) {
							System.out.println("Reasoning content: " + cnt.text());
						}
				} else {
					System.out.println("Other output!");
				}
			}
		return res;
	}

	private Builder appendTools(Builder builder, List<String> tools) {
		List<ResponseInputItem> inputs = new ArrayList<ResponseInputItem>();

		StringBuffer out = new StringBuffer("~~~\nTool output:\n");
		tools.forEach(t -> out.append(t));
		out.append("\n~~~");

		ResponseInputText inputFile = ResponseInputText.builder() //
				.text(out.toString()) //
				.build();
		ResponseInputItem inputItem = ResponseInputItem.ofMessage(ResponseInputItem.Message.builder() //
				.role(ResponseInputItem.Message.Role.DEVELOPER)//
				.addContent(inputFile).build());
		inputs.add(inputItem);
		return builder.inputOfResponse(inputs);
	}

	@Override
	public AIAnswer convertToAnswer(String bodyJson) {

		try {
			ObjectNode tree = (ObjectNode) ObjectMappers.jsonMapper().readTree(bodyJson);
			tree.get("error"); // errors
			TextNode id = (TextNode) tree.get("id");
			ObjectNode response = (ObjectNode) tree.get("response");
			// IntNode statusCode = (IntNode) response.get("status_code");
			ObjectNode body = (ObjectNode) response.get("body");

			String cbodyJson = ObjectMappers.jsonMapper().writeValueAsString(body);
			AIAnswer answer = convertToAnswer1(cbodyJson);
			answer.id = id.asText();
			return answer;

		} catch (JsonProcessingException e) {
			throw new IllegalStateException(e);
		}
	}

	private AIAnswer convertToAnswer1(String bodyJson) throws JsonMappingException, JsonProcessingException {
		Response resp = ObjectMappers.jsonMapper().readerFor(Response.class).readValue(bodyJson);

		AIAnswer answer = convertResponse(new OpenAIModelResponse(resp));
		return answer;
	}
}
