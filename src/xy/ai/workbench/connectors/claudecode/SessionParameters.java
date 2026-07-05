package xy.ai.workbench.connectors.claudecode;

import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.List;

import xy.ai.workbench.AgentProfile;
import xy.ai.workbench.Model;
import xy.ai.workbench.Reasoning;

public class SessionParameters {
	private static final String SCRIPT = System.getProperty("user.home")
			+ "/xyan/xy.ai.workbench/claude-code/claude-session.sh";

	/** Deterministic hash of the session parameters. Immutable. */
	public final Path cwd;
	public final Model model;
	public final Reasoning reasoning;
	public final AgentProfile agentProfile;
	public final String cliProfile;
	private String hash;
	private String title;

	public SessionParameters(Path cwd, Model model, Reasoning reasoning, AgentProfile agentProfile, String cliProfile) {
		this.cwd = cwd;
		this.model = model;
		this.reasoning = reasoning;
		this.agentProfile = agentProfile;
		this.cliProfile = cliProfile;
		if (cwd == null)
			throw new IllegalStateException("Work directory not set");
	}

	public List<String> buildBaseCommand() {
		List<String> cmd = new ArrayList<>();
		cmd.add(SCRIPT);
		cmd.add(agentProfile != null ? agentProfile.name : ""); // Agent definition
		cmd.add("--profile");
		cmd.add(cliProfile);
		cmd.add("--verbose");
		cmd.add("--include-hook-events");
		cmd.add("--include-partial-messages");
		cmd.add("--input-format");
		cmd.add("stream-json");
		cmd.add("--output-format");
		cmd.add("stream-json");
		cmd.add("--replay-user-messages");
		cmd.add("--model");
		cmd.add(model.apiName);
		if (Reasoning.Disabled != reasoning) {
			cmd.add("--effort");
			cmd.add(reasoning.name().toLowerCase());
		}
		cmd.add("--dangerously-skip-permissions"); // as long there is no permission prompt handling implemented
		return cmd;
	}

	public String getHash() {
		if (hash == null)
			hash = computeHash();
		return hash;
	}

	private String computeHash() {
		String input = cwd.toString() + "|" + model.apiName + "|" + reasoning.name() + "|" + agentProfile.name + "|"
				+ cliProfile;
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
