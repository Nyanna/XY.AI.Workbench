package xy.ai.workbench;

import java.io.IOException;

import org.eclipse.ui.IMemento;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;

/**
 * (De)serializes {@link SessionConfig} as a single JSON blob instead of mirroring its structure
 * as a tree of {@link IMemento} children/items. This avoids duplicating the config structure and,
 * more importantly, decouples parsing from applying: {@link #loadConfig(IMemento)} only returns a
 * plain data snapshot, letting {@link ConfigManager} decide the (capability-dependent) order in
 * which the values are actually applied.
 */
public class MementoConverter {

	private static final ObjectMapper MAPPER = new ObjectMapper();

	public static void saveConfig(IMemento memento, SessionConfig cfg) {
		Snapshot snap = new Snapshot();
		snap.keys = cfg.keys;
		snap.maxOutputTokens = cfg.maxOutputTokens;
		snap.temperature = cfg.temperature;
		snap.topP = cfg.topP;
		snap.freeText = cfg.freeText;
		if (cfg.model != null) {
			snap.modelProvider = cfg.model.cap.getKeyPattern().name();
			snap.modelApiName = cfg.model.apiName;
		}
		snap.reasoning = cfg.reasoning != null ? cfg.reasoning.name() : null;
		snap.cacheMode = cfg.cacheMode != null ? cfg.cacheMode.name() : null;
		snap.profile = cfg.profile != null ? cfg.profile.name() : null;
		snap.reasoningBudget = cfg.reasoningBudget;
		snap.systemPrompt = cfg.systemPrompt;
		snap.outputMode = cfg.ouputMode != null ? cfg.ouputMode.name() : null;
		snap.inputModes = cfg.inputModes;

		try {
			memento.createChild("cfg").putString("json", MAPPER.writeValueAsString(snap));
		} catch (JsonProcessingException e) {
			LOG.error("Unable to serialize session config", e);
		}
	}

	/** Returns the raw, persisted snapshot - or {@code null} if there is none. Applies nothing. */
	public static Snapshot loadConfig(IMemento memento) {
		if (memento == null)
			return null;
		IMemento m = memento.getChild("cfg");
		String json = m != null ? m.getString("json") : null;
		if (json == null)
			return null;
		try {
			return MAPPER.readValue(json, Snapshot.class);
		} catch (IOException e) {
			LOG.error("Unable to deserialize session config", e);
			return null;
		}
	}

	/** Plain, capability-agnostic data snapshot of a {@link SessionConfig}. */
	public static class Snapshot {
		public String keys;
		public Long maxOutputTokens;
		public Double temperature;
		public Double topP;
		public String freeText;
		public String modelProvider;
		public String modelApiName;
		public String reasoning;
		public String cacheMode;
		public String profile;
		public Integer reasoningBudget;
		public String[] systemPrompt;
		public String outputMode;
		public boolean[] inputModes;
	}
}
