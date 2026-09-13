Implementiere für alle Connectoren Tool Unterstützung

- beim Setzen eines Modells in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java` soll deren `xy.ai.workbench.Model.Capabilities.tools` durch die über MCP verfügbaren Tools via `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPClient.java` befüllt werden
	- Die Einträge sollen auf Basis der Reihenfolge in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Tools.java` (nicht enthaltende angefügt) sortiert sein
- In allen Implementierungen von `IAIConnector` sollen die aktivierten Tools als weiterer Parameter in `createRequest` reingereicht werden
	- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/AdaptingConnector.java` nur durchgereicht
	- `MCPClient` Parameter ergänzen aber nicht verarbeiten
	- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java` wird die Toolliste breits korrekt verarbeitet.
	- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claude/ClaudeConnector.java`, `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/deepseek/DeepSeekConnector.java`, `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/google/GeminiConnector.java`, `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/openai/OpenAIConnector.java` wird die eingegebene Liste mittels `MCPClient` in ein JSON Toolschema aufgelöst und dem Request providerkonform als Function angefügt(Nicht MCP sondern Tool im Harness). 
- Die vom Modell in Rückgabe enthaltenen Calls werden analog der Verwendung von `MCPClient` als Mardown YAML Script Blöcke mit in der Antwort zurückgegeben
		

## Tool-Definition nach API

### Anthropic

```java
import com.anthropic.core.JsonValue;
import com.anthropic.models.messages.ContentBlockParam;
// ...
import com.anthropic.models.messages.Tool;
import com.anthropic.models.messages.Tool.InputSchema;
import com.anthropic.models.messages.ToolChoiceAuto;
import com.anthropic.models.messages.ToolResultBlockParam;
import com.anthropic.models.messages.ToolUseBlock;
// ...

void main() {
    AnthropicClient client = AnthropicOkHttpClient.fromEnv();

    Tool weatherTool = Tool.builder()
        .name("get_weather")
        .description("Get the current weather for a given location.")
        .inputSchema(InputSchema.builder()
            .properties(JsonValue.from(Map.of(
                "location", Map.of(
                    "type", "string",
                    "description", "City and state, e.g. San Francisco, CA"
                )
            )))
            .required(List.of("location"))
            .build())
        .build();

    // Ask for at most one tool call per turn.
    ToolChoiceAuto toolChoice = ToolChoiceAuto.builder()
        .disableParallelToolUse(true)
        .build();

    String userPrompt = "What's the weather in San Francisco?";

    // Claude replies with a tool_use block naming the tool and its arguments.
    Message response = client.messages().create(MessageCreateParams.builder()
        .model(Model.CLAUDE_OPUS_5)
        .maxTokens(1024L)
        .addTool(weatherTool)
        .toolChoice(toolChoice)
        .addUserMessage(userPrompt)
        .build());
    ToolUseBlock toolUse = response.content().stream()
        .flatMap(block -> block.toolUse().stream())
        .findFirst()
        .orElseThrow();
    IO.println("Claude called " + toolUse.name() + " with " + toolUse._input());

    // Run the tool, then send the result back in a tool_result block.
    String weather = "15 degrees Celsius, partly cloudy";
    Message followup = client.messages().create(MessageCreateParams.builder()
        .model(Model.CLAUDE_OPUS_5)
        .maxTokens(1024L)
        .addTool(weatherTool)
        .toolChoice(toolChoice)
        .addUserMessage(userPrompt)
        .addMessage(response)
        .addUserMessageOfBlockParams(List.of(ContentBlockParam.ofToolResult(
            ToolResultBlockParam.builder()
                .toolUseId(toolUse.id())
                .content(weather)
                .build())))
        .build());

    // Claude uses the result to answer the original question.
    followup.content().stream()
        .flatMap(block -> block.text().stream())
        .forEach(textBlock -> IO.println(textBlock.text()));
}
```

### Google Gemini

json

```java
    import com.google.genai.Client;
import com.google.genai.gaos.models.interactions.CreateModelInteraction;
import com.google.genai.gaos.models.interactions.Function;
import com.google.genai.gaos.models.interactions.FunctionCallStep;
import com.google.genai.gaos.models.interactions.Interaction;
import com.google.genai.gaos.models.interactions.InteractionsInput;
import com.google.genai.gaos.models.interactions.Model;
import com.google.genai.gaos.models.interactions.Step;
import com.google.genai.gaos.models.operations.CreateInteractionRequestBody;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

Client client = new Client();

Map<String, Object> attendeesProp = new HashMap<>();
attendeesProp.put("type", "array");
Map<String, Object> itemsMap = new HashMap<>(); itemsMap.put("type", "string"); attendeesProp.put("items", itemsMap);

Map<String, Object> dateProp = new HashMap<>();
dateProp.put("type", "string");
dateProp.put("description", "Date (e.g., \"2024-07-29\")");

Map<String, Object> timeProp = new HashMap<>();
timeProp.put("type", "string");
timeProp.put("description", "Time (e.g., \"15:00\")");

Map<String, Object> topicProp = new HashMap<>();
topicProp.put("type", "string");
topicProp.put("description", "The meeting topic.");

Map<String, Object> properties = new HashMap<>();
properties.put("attendees", attendeesProp);
properties.put("date", dateProp);
properties.put("time", timeProp);
properties.put("topic", topicProp);

Map<String, Object> parameters = new HashMap<>();
parameters.put("type", "object");
parameters.put("properties", properties);
parameters.put("required", Arrays.asList("attendees", "date", "time", "topic"));

Function scheduleMeetingFunction =
    Function.builder()
        .name("schedule_meeting")
        .description("Schedules a meeting with specified attendees at a given time and date.")
        .parameters(parameters)
        .build();

CreateModelInteraction params =
    CreateModelInteraction.builder()
        .model(Model.of("gemini-3.6-flash"))
        .input(InteractionsInput.of("Schedule a meeting with Bob and Alice for 03/27/2025 at 10:00 AM about Q3 planning."))
        .tools(Arrays.asList(scheduleMeetingFunction))
        .build();

Interaction interaction =
    client.interactions.create(CreateInteractionRequestBody.of(params)).interaction().get();

if (interaction.steps().isPresent()) {
  for (Step step : interaction.steps().get()) {
    if (step instanceof FunctionCallStep) {
      FunctionCallStep functionCall = (FunctionCallStep) step;
      System.out.println("Function to call: " + functionCall.name().orElse(""));
      System.out.println("Arguments: " + functionCall.arguments().orElse(null));
    }
  }
}
```

Im Request-Body unter `tools`, Response enthält `functionCall` Object mit Name + Argumenten.

### OpenAI

```java
import com.openai.client.OpenAIClient;
import com.openai.client.okhttp.OpenAIOkHttpClient;
import com.openai.core.JsonValue;
import com.openai.models.responses.FunctionTool;
import com.openai.models.responses.ResponseCreateParams;
import com.openai.models.responses.ResponseInputItem;
import java.util.List;
import java.util.Map;

FunctionTool horoscope =
    FunctionTool.builder()
        .name("get_horoscope")
        .description("Get today's horoscope for an astrological sign.")
        .parameters(
            FunctionTool.Parameters.builder()
                .putAdditionalProperty("type", JsonValue.from("object"))
                .putAdditionalProperty(
                    "properties",
                    JsonValue.from(
                        Map.of(
                            "sign",
                            Map.of(
                                "type", "string",
                                "description",
                                    "An astrological sign like Taurus or Aquarius"))))
                .putAdditionalProperty("required", JsonValue.from(List.of("sign")))
                .putAdditionalProperty("additionalProperties", JsonValue.from(false))
                .build())
        .strict(true)
        .build();

var firstResponse =
    client
        .responses()
        .create(
            ResponseCreateParams.builder()
                .model("gpt-6-astra")
                .input("What is my horoscope? I am an Aquarius.")
                .addTool(horoscope)
                .build());

var functionCall =
    firstResponse.output().stream()
        .flatMap(item -> item.functionCall().stream())
        .filter(call -> call.name().equals("get_horoscope"))
        .findFirst()
        .orElseThrow(() -> new IllegalStateException("The model did not call get_horoscope"));

record HoroscopeArguments(String sign) {}

String sign = functionCall.arguments(HoroscopeArguments.class).sign();
ResponseCreateParams followUp =
    ResponseCreateParams.builder()
        .model("gpt-6-astra")
        .instructions("Respond only with a horoscope generated by a tool.")
        .previousResponseId(firstResponse.id())
        .inputOfResponse(
            List.of(
                ResponseInputItem.ofFunctionCallOutput(
                    ResponseInputItem.FunctionCallOutput.builder()
                        .callId(functionCall.callId())
                        .output(sign + ": Embrace an unexpected opportunity today.")
                        .build())))
        .addTool(horoscope)
        .build();

client.responses().create(followUp).output().stream()
    .flatMap(item -> item.message().stream())
    .flatMap(message -> message.content().stream())
    .flatMap(content -> content.outputText().stream())
    .forEach(text -> System.out.println(text.text()));
```