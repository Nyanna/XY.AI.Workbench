package xy.ai.workbench;

import org.eclipse.ui.IMemento;

public class MementoConverter {

	public static void saveConfig(IMemento memento, SessionConfig cfg) {
		var m = memento.createChild("cfg");

		if (cfg.key != null)
			m.putString("key", cfg.key);
		if (cfg.maxOutputTokens != null)
			m.putString("maxOutputTokens", String.valueOf(cfg.maxOutputTokens));
		if (cfg.temperature != null)
			m.putString("temperature", String.valueOf(cfg.temperature));
		if (cfg.topP != null)
			m.putString("topP", String.valueOf(cfg.topP));
		if (cfg.model != null)
			m.putString("model", cfg.model.name());
		if (cfg.reasoning != null)
			m.putString("reasoning", cfg.reasoning.name());
		if (cfg.reasoningBudget != null)
			m.putInteger("reasoningBudget", cfg.reasoningBudget);
		int spLen = cfg.systemPrompt != null ? cfg.systemPrompt.length : 0;
		m.putInteger("systemPrompt.length", spLen);
		if (spLen > 0) {
			IMemento sp = m.createChild("systemPrompt");
			for (int i = 0; i < spLen; i++) {
				IMemento item = sp.createChild("item");
				item.putInteger("index", i);
				item.putString("value", cfg.systemPrompt[i]);
			}
		}
		if (cfg.ouputMode != null)
			m.putString("outputMode", cfg.ouputMode.name());
		int imLen = cfg.inputModes != null ? cfg.inputModes.length : 0;
		m.putInteger("inputModes.length", imLen);
		if (imLen > 0) {
			IMemento im = m.createChild("inputModes");
			for (int i = 0; i < imLen; i++) {
				IMemento item = im.createChild("item");
				item.putInteger("index", i);
				item.putString("value", Boolean.toString(cfg.inputModes[i]));
			}
		}
	}

	public static void loadConfig(IMemento memento, SessionConfig cfg) {
		if (memento == null)
			return;
		var m = memento.getChild("cfg");
		if (m == null)
			return;

		cfg.key = m.getString("key");
		String maxTok = m.getString("maxOutputTokens");
		cfg.maxOutputTokens = maxTok != null ? Long.valueOf(maxTok) : null;
		String tmp = m.getString("temperature");
		cfg.temperature = tmp != null ? Double.valueOf(tmp) : null;
		String tp = m.getString("topP");
		cfg.topP = tp != null ? Double.valueOf(tp) : null;
		String mdl = m.getString("model");
		cfg.model = mdl != null ? Model.valueOf(mdl) : null;
		String rsn = m.getString("reasoning");
		cfg.reasoning = rsn == null ? cfg.reasoning : Reasoning.valueOf(rsn);
		Integer rsnb = m.getInteger("reasoningBudget");
		cfg.reasoningBudget = rsnb != null ? cfg.reasoningBudget : rsnb;
		Integer spLen = m.getInteger("systemPrompt.length");
		int sLen = spLen != null ? spLen : 0;
		if (sLen > 0) {
			IMemento sp = m.getChild("systemPrompt");
			String[] arr = new String[sLen];
			if (sp != null) {
				IMemento[] items = sp.getChildren("item");
				for (IMemento it : items) {
					Integer idx = it.getInteger("index");
					String val = it.getString("value");
					if (idx != null && idx >= 0 && idx < sLen && idx < arr.length)
						arr[idx] = val;
				}
			}
			cfg.systemPrompt = arr;
		}
		String om = m.getString("outputMode");
		cfg.ouputMode = om != null ? OutputMode.valueOf(om) : null;
		Integer imLen = m.getInteger("inputModes.length");
		int iLen = imLen != null ? imLen : 0;
		if (iLen > 0) {
			IMemento im = m.getChild("inputModes");
			boolean[] arr = new boolean[InputMode.values().length];
			if (im != null) {
				IMemento[] items = im.getChildren("item");
				for (IMemento it : items) {
					Integer idx = it.getInteger("index");
					String val = it.getString("value");
					if (idx != null && idx >= 0 && idx < iLen && idx < arr.length)
						arr[idx] = Boolean.parseBoolean(val);
				}
			}
			cfg.inputModes = arr;
		}
	}
}
