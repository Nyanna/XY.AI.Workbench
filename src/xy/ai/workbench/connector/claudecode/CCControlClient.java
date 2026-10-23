package xy.ai.workbench.connector.claudecode;

import org.eclipse.core.runtime.CoreException;
import org.eclipse.core.runtime.NullProgressMonitor;
import org.eclipse.debug.core.DebugPlugin;
import org.eclipse.debug.core.ILaunch;
import org.eclipse.debug.core.ILaunchConfiguration;
import org.eclipse.debug.core.ILaunchConfigurationType;
import org.eclipse.debug.core.ILaunchConfigurationWorkingCopy;
import org.eclipse.debug.core.ILaunchManager;
import java.io.IOException;
import java.net.URI;
import java.net.URLEncoder;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpRequest.BodyPublishers;
import java.net.http.HttpResponse;
import java.net.http.HttpResponse.BodyHandlers;
import java.nio.charset.StandardCharsets;
import java.time.Duration;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import xy.ai.workbench.EditorInterface;
import xy.ai.workbench.LOG;
import xy.ai.workbench.commands.AnswerCommand;

/**
 * Minimal HTTP client for the MCPC human-in-the-loop tool-control endpoint
 * ({@code POST /control/tool}). Replaces the standalone {@code control.sh}
 * client: this class is used directly from the connector's retrieval loop to
 * poll for pending tool-call requests/results and to submit approval,
 * rejection, or modification decisions.
 *
 * <p>
 * Request body: {@code {"approvals":[...]}} (may be empty for a plain poll).
 * Response body: {@code {"pending":[...]}}.
 */
public class CCControlClient {
  private static final String CONTROL_URL = "http://localhost:9093/control/tool";
  private static final Duration TIMEOUT = Duration.ofSeconds(5);
  private static final String MCPC_LAUNCH_NAME = "MCPC";
  private final ObjectMapper mapper = JsonUtil.mapper();
  private YamlRenderer yaml = new YamlRenderer();
  private final HttpClient http = HttpClient.newBuilder().connectTimeout(TIMEOUT).build();
  private boolean launchAttempted;

  public void checkControlEndpoint(CCResponse resp, String sessionId) {
    JsonNode pending = poll(sessionId);
    if (pending.isEmpty())
      return;
    StringBuilder res = new StringBuilder();
    ProtocolParser.appendEvents(resp.events, res);
    JsonNode first = pending.get(0);
    res.append(String.format("%s\n```yaml\n%s\n```\n%s %s allow", EditorInterface.CONTROL_REQUEST, toYaml(first), AnswerCommand.CMD_ANSWER, first.path("id").asText()));
    resp.resultText = res.toString();
    resp.showStats = false;
  }

  public String toYaml(JsonNode node) {
    return yaml.toYaml(node);
  }

  public JsonNode fromYaml(String text) throws JsonProcessingException {
    return yaml.readTree(text);
  }

  public boolean isMCPCAvailable() {
    if (ping())
      return true;
    synchronized (this) {
      if (launchAttempted)
        return false;
      launchAttempted = true;
    }
    return startMcpcLaunch();
  }

  private boolean ping() {
    try {
      return poll(null) != null;
    } catch (IllegalStateException e) {
      return false;
    }
  }

  private boolean startMcpcLaunch() {
    try {
      ILaunchManager lm = DebugPlugin.getDefault().getLaunchManager();
      for (ILaunch l : lm.getLaunches()) {
        ILaunchConfiguration c = l.getLaunchConfiguration();
        if (c != null && MCPC_LAUNCH_NAME.equals(c.getName()) && !l.isTerminated())
          return false;
      }
      ILaunchConfiguration cfg = null;
      for (ILaunchConfiguration c : lm.getLaunchConfigurations()) {
        if (MCPC_LAUNCH_NAME.equals(c.getName())) {
          cfg = c;
          break;
        }
      }
      if (cfg == null) {
        ILaunchConfigurationType type = lm.getLaunchConfigurationType("org.eclipse.ui.externaltools.ProgramLaunchConfigurationType");
        ILaunchConfigurationWorkingCopy wc = type.newInstance(null, MCPC_LAUNCH_NAME);
        String dir = "/home/user/xyan/xy.ai.workbench/mcpc";
        wc.setAttribute("org.eclipse.ui.externaltools.ATTR_LOCATION", "/usr/bin/bash");
        wc.setAttribute("org.eclipse.ui.externaltools.ATTR_TOOL_ARGUMENTS", "-lc \"" + dir + "/start.sh\"");
        wc.setAttribute("org.eclipse.ui.externaltools.ATTR_WORKING_DIRECTORY", dir);
        wc.setAttribute("org.eclipse.ui.externaltools.ATTR_LAUNCH_CONFIGURATION_BUILD_SCOPE", "${none}");
        wc.setAttribute("org.eclipse.debug.core.environmentVariables", java.util.Map.of("TERM", "xterm-256color"));
        cfg = wc.doSave();
      }
      cfg.launch(ILaunchManager.RUN_MODE, new NullProgressMonitor());
      long end = System.currentTimeMillis() + 3000;
      while (System.currentTimeMillis() < end) {
        if (ping())
          return true;
        Thread.sleep(250);
      }
    } catch (InterruptedException e) {
      Thread.currentThread().interrupt();
    } catch (CoreException e) {
      LOG.error("Failed to start MCPC run configuration", e);
    }
    return false;
  }

  private ArrayNode poll(String sessionId) {
    return post(mapper.createObjectNode(), sessionId);
  }

  public ArrayNode approve(String id) {
    return approve(id, null);
  }

  public ArrayNode approve(String id, String hint) {
    return submit(approvalNode(id, null, null, null, hint == null || hint.isBlank() ? null : hint, false));
  }

  public ArrayNode exec(String id, String hint) {
    return submit(approvalNode(id, null, null, null, hint == null || hint.isBlank() ? null : hint, true));
  }

  public ArrayNode deny(String id, String reason) {
    return submit(approvalNode(id, null, null, reason == null ? "" : reason, null, false));
  }

  public boolean submitEdit(String yamlBlock) {
    if (yamlBlock == null || yamlBlock.isBlank())
      return false;
    JsonNode edited;
    try {
      edited = fromYaml(yamlBlock);
    } catch (Exception e) {
      throw new IllegalArgumentException("YAML Error", e);
    }
    if (edited == null || !edited.isObject() || !edited.hasNonNull("id"))
      return false;
    submit((ObjectNode) edited);
    return true;
  }

  private ObjectNode approvalNode(String id, JsonNode arguments, JsonNode result, String rejectReason, String hint, boolean exec) {
    ObjectNode approval = mapper.createObjectNode();
    approval.put("id", id);
    if (arguments != null)
      approval.set("arguments", arguments);
    if (result != null)
      approval.set("result", result);
    if (rejectReason != null) {
      approval.put("rejected", true);
      approval.put("reason", rejectReason);
    }
    if (hint != null)
      approval.put("hint", hint);
    if (exec)
      approval.put("exec", true);
    return approval;
  }

  private ArrayNode submit(ObjectNode approval) {
    ObjectNode body = mapper.createObjectNode();
    body.putArray("approvals").add(approval);
    return post(body, null);
  }

  private ArrayNode post(ObjectNode body, String sessionId) {
    try {
      String json = JsonUtil.write(body);
      String url = sessionId == null ? CONTROL_URL : CONTROL_URL + "?session=" + URLEncoder.encode(sessionId, StandardCharsets.UTF_8);
      HttpRequest request = HttpRequest.newBuilder().uri(URI.create(url)).timeout(TIMEOUT).header("Content-Type", "application/json").POST(BodyPublishers.ofString(json)).build();
      HttpResponse<String> response = http.send(request, BodyHandlers.ofString());
      if (response.statusCode() / 100 != 2) {
        LOG.error("control endpoint returned status " + response.statusCode());
        return mapper.createArrayNode();
      }
      String responseBody = response.body();
      if (responseBody == null || responseBody.isBlank()) {
        LOG.error("control endpoint returned an empty body");
        return mapper.createArrayNode();
      }
      JsonNode root = JsonUtil.readTree(responseBody);
      JsonNode pending = root.path("pending");
      return pending.isArray() ? (ArrayNode) pending : mapper.createArrayNode();
    } catch (IOException | InterruptedException e) {
      LOG.error("Control endpoint unreachable", e);
      if (e instanceof InterruptedException)
        Thread.currentThread().interrupt();
      throw new IllegalStateException("Error on control endpoint", e);
    }
  }
}
