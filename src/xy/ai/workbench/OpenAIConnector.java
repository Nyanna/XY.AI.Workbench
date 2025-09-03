package xy.ai.workbench;

import java.util.ArrayList;
import java.util.List;

import com.openai.client.OpenAIClient;
import com.openai.client.okhttp.OpenAIOkHttpClient;
import com.openai.core.http.HttpResponseFor;
import com.openai.models.Reasoning;
import com.openai.models.ReasoningEffort;
import com.openai.models.responses.Response;
import com.openai.models.responses.Response.Instructions;
import com.openai.models.responses.ResponseCreateParams;
import com.openai.models.responses.ResponseCreateParams.Builder;
import com.openai.models.responses.ResponseCreateParams.Truncation;
import com.openai.models.responses.ResponseInputItem;
import com.openai.models.responses.ResponseInputText;
import com.openai.models.responses.ResponseStatus;
import com.openai.models.responses.ResponseUsage;

public class OpenAIConnector {
	SessionConfig cfg;
	private OpenAIClient client;

	public OpenAIConnector(SessionConfig cfg) {
		this.cfg = cfg;
	}

	public ResponseCreateParams createRequest(String input, String systemPrompt, List<String> tools) {
		boolean isBackground = false;
		if (this.client == null)
			this.client = OpenAIOkHttpClient.builder().apiKey(cfg.key).build();

		Builder builder = ResponseCreateParams.builder() //
				.maxOutputTokens(cfg.maxOutputTokens)
				// .temperature(cfg.temperature) // Not supported
				// .topP(cfg.topP) // Not Supported
				.truncation(Truncation.DISABLED) //
				.maxToolCalls(1)//
				.background(isBackground)//
				.instructions(systemPrompt)//
				.reasoning( //
						Reasoning.builder()//
								.effort(ReasoningEffort.MINIMAL) //
								.summary(Reasoning.Summary.AUTO)//
								.build())
				.model(cfg.model); //
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
		return params;
	}

	public Response executeRequest(ResponseCreateParams params) {
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
		return resp;
	}

	public AIAnswer convertResponse(Response resp) {

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

		if (resp.error().isPresent()) {
			var error = resp.error().get();
			System.out.println("Error: " + error.code() + " " + error.message());
		}
		if (resp.incompleteDetails().isPresent()) {
			var incomplete = resp.incompleteDetails().get();
			System.out.println("Incomplete: " + incomplete.reason().get().toString() + " ");
		}

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
}
