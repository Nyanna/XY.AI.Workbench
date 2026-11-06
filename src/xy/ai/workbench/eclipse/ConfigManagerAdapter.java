package xy.ai.workbench.eclipse;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.Objects;

import org.eclipse.core.resources.IFile;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IMemento;
import org.eclipse.ui.ide.ResourceUtil;

import xy.ai.conscious.api.ext.IFileCache;
import xy.ai.conscious.connector.mcp.MCPClient;
import xy.ai.conscious.core.AgentProfile;
import xy.ai.conscious.core.ConfigManager;
import xy.ai.conscious.core.InputMode;
import xy.ai.conscious.core.LOG;
import xy.ai.conscious.model.CacheMode;
import xy.ai.conscious.model.Model;
import xy.ai.conscious.model.Reasoning;
import xy.ai.conscious.model.Model.KeyPattern;
import xy.ai.conscious.core.OutputMode;
import xy.ai.conscious.core.SessionConfig;
import xy.ai.workbench.ActiveEditorListener;
import xy.ai.workbench.MementoConverter;

public class ConfigManagerAdapter extends ConfigManager{
	private ConfigManager cm;
	private String activeEditorKey;
	private Map<String, SessionConfig> editorConfigs = new HashMap<>();
	private SessionConfig defaultCfg = new SessionConfig();
	

	public ConfigManagerAdapter(ActiveEditorListener editorListener, MCPClient mcpClient, IFileCache cache) {
		super(mcpClient, cache);
		cm = this;
		cfg = defaultCfg;
		editorListener.addTextEditorObserver(e -> activateEditor(e));
	}
	
	public ConfigManager getManager() {
		return cm;
	}

	private void activateEditor(IEditorPart editor) {
		String key = resolveKey(editor);
		if (Objects.equals(activeEditorKey, key))
			return;
		activeEditorKey = key;
		SessionConfig target = key == null ? defaultCfg : editorConfigs.getOrDefault(key, cfg);
		cm.applyConfig(target);
	}

	private static String resolveKey(IEditorPart editor) {
		if (editor == null)
			return null;
		IFile file = ResourceUtil.getFile(editor.getEditorInput());
		return file != null ? file.getFullPath().toString() : null;
	}

	protected void ensureOwnConfig() {
		if (activeEditorKey == null || editorConfigs.containsKey(activeEditorKey))
			return;
		cfg = new SessionConfig(cfg);
		editorConfigs.put(activeEditorKey, cfg);
	}


	public void saveConfig(IMemento memento) {
		MementoConverter.saveConfig(memento, cfg);
	}

	public void loadConfig(IMemento memento) {
		String savedKey = activeEditorKey;
		activeEditorKey = null;
		cfg = defaultCfg;
		try {
			MementoConverter.Snapshot snap = MementoConverter.loadConfig(memento);
			if (snap == null)
				return;

			/*
			 * Order matters: keys must be set (models resolved) before the model itself can
			 * be restored, and the model must be set before the remaining,
			 * capability-dependent settings are applied - otherwise setModel()'s own
			 * defaulting logic would clobber them.
			 */
			if (snap.keys != null)
				cm.setKey(snap.keys);

			if (snap.modelProvider != null) {
				Model resolved = Arrays.stream(cm.getEnabledModels())
						.filter(m -> m.matches(KeyPattern.valueOf(snap.modelProvider), snap.modelDisplayName)).findFirst()
						.orElse(null);
				if (resolved != null)
					cm.setModel(resolved);
				else
					LOG.error("Unable to resolve persisted model " + snap.modelProvider + ":" + snap.modelDisplayName
							+ ", no fallback applied");
			}

			if (snap.temperature != null)
				cm.setTemperature(snap.temperature);
			if (snap.topP != null)
				cm.setTopP(snap.topP);
			if (snap.maxOutputTokens != null)
				cm.setMaxOutputTokens(snap.maxOutputTokens);
			if (snap.reasoningBudget != null)
				cm.setReasoningBudget(snap.reasoningBudget);
			if (snap.reasoning != null)
				cm.setReasoning(Reasoning.valueOf(snap.reasoning));
			if (snap.cacheMode != null)
				cm.setCacheMode(CacheMode.valueOf(snap.cacheMode));
			if (snap.profile != null)
				cm.setProfile(AgentProfile.valueOf(snap.profile));
			if (snap.systemPrompt != null)
				cm.setSystemPrompt(snap.systemPrompt);
			if (snap.freeText != null)
				cm.setSystemFree(snap.freeText);
			if (snap.outputMode != null)
				cm.setOuputMode(OutputMode.valueOf(snap.outputMode));
			if (snap.inputModes != null)
				for (InputMode mode : InputMode.values())
					if (mode.ordinal() < snap.inputModes.length)
						cm.setInputMode(mode, snap.inputModes[mode.ordinal()]);
			if (snap.enabledTools != null)
				cm.setEnabledTools(snap.enabledTools);
		} catch (Exception e) {
			LOG.error(e.getMessage(), e);
			LOG.info("Unable to restore config", e);
		} finally {
			activeEditorKey = savedKey;
			cfg = savedKey == null ? defaultCfg : editorConfigs.getOrDefault(savedKey, defaultCfg);
		}
	}
}
