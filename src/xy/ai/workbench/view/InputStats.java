package xy.ai.workbench.view;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;

import xy.ai.conscious.api.IActiveEditor;
import xy.ai.conscious.core.ConfigManager;
import xy.ai.conscious.core.InputMode;
import xy.ai.workbench.ActiveEditorListener;

public class InputStats {
	private int[] inputStats = new int[InputMode.values().length];
	private List<Consumer<int[]>> inputStatObs = new ArrayList<>();
	private final ConfigManager cfg;
	private final IActiveEditor editorListener;

	public InputStats(ConfigManager cfg, ActiveEditorListener editorListener) {
		this.cfg = cfg;
		this.editorListener = editorListener;
		editorListener.addInputObserver(i -> updateInputStat(i));
		cfg.addInputModeObs(i -> updateInputStat(i));
		cfg.addEnabledToolsObs(t -> updateInputStat(InputMode.Tools), false);
	}

	public void initializeInputs() {
		for (var mode : InputMode.values())
			updateInputStat(mode);
	}

	public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {
		inputStatObs.add(obs);
		if (initialize)
			obs.accept(inputStats);
	}

	private String getInput(InputMode mode) {
		switch (mode) {
		case SystemPrompt:
			StringBuffer systemPrompt = new StringBuffer();
			Arrays.stream(cfg.getSystemPrompt()).filter(e -> !e.startsWith("#"))
					.forEach(e -> systemPrompt.append("* ").append(e).append(".\n"));
			String freeText = cfg.getFreeText();
			if (freeText != null && !freeText.isBlank())
				systemPrompt.append(".\n").append(cfg.getFreeText()).append(".\n");
			String prompttext = systemPrompt.toString();
			return prompttext.length() > 0 && !prompttext.isBlank() ? prompttext : null;
		case Selection:
			return IActiveEditor.removeCommentLines(editorListener.getSelection());
		case Converter:
			return IActiveEditor.removeCommentLines(editorListener.getFileContent());
		case Tools:
			throw new UnsupportedOperationException();
		}
		return null;
	}

	private void updateInputStat(InputMode mode) {
		if (mode == InputMode.Tools) {
			String[] tools = cfg.getEnabledTools();
			inputStats[mode.ordinal()] = tools != null ? tools.length : 0;
		} else {
			String input = getInput(mode);
			inputStats[mode.ordinal()] = input != null ? input.length() : -1;
		}
		inputStatObs.forEach(c -> c.accept(inputStats));
	}
}
