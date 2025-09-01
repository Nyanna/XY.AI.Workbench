package xy.ai.workbench;

import com.openai.client.OpenAIClient;
import com.openai.client.okhttp.OpenAIOkHttpClient;
import com.openai.core.http.HttpResponseFor;
import com.openai.models.Reasoning;
import com.openai.models.ReasoningEffort;
import com.openai.models.responses.Response;
import com.openai.models.responses.ResponseCreateParams;
import com.openai.models.responses.ResponseCreateParams.Truncation;
import com.openai.models.responses.ResponseStatus;
import com.openai.models.responses.ResponseUsage;

public class OpenAPIConnector {
	SessionConfig cfg;
	private OpenAIClient client;

	public OpenAPIConnector(SessionConfig cfg) {
		this.cfg = cfg;
	}

	public AIAnswer sendRequest(String input, String systemPrompt) {
		AIAnswer res = new AIAnswer();
		var isBackground = false;
		if (this.client == null)
			this.client = OpenAIOkHttpClient.builder().apiKey(cfg.key).build();

		ResponseCreateParams params = ResponseCreateParams.builder() //
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
				.model(cfg.model) //
				.input(input) //
				.build();
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

		if (resp.usage().isPresent()) {
			ResponseUsage usage = resp.usage().get();
			System.out.println("Usage In: " + usage.inputTokens() + ", Out: " + usage.outputTokens() + ", Total: "
					+ usage.totalTokens() + "");
			res.inputToken = usage.inputTokens();
			res.outputToken = usage.outputTokens();
			res.totalToken = usage.totalTokens();
			if (usage.outputTokensDetails() != null) {
				System.out.println("Out Details Reasoning: " + usage.outputTokensDetails().reasoningTokens() + "");
				res.reasoningToken = usage.outputTokensDetails().reasoningTokens();
			}
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
						System.out.println("Answer: " + answer);
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

		System.out.println("Finished");
		return res;

	}

	public static void main(String[] args) {
		SessionConfig cfg = new SessionConfig();
		new OpenAPIConnector(cfg).sendRequest("Say hello", String.join(", ", cfg.systemPrompt));
	}
}
