package xy.ai.workbench;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;
import java.util.stream.Collectors;

import org.eclipse.ui.IMemento;

import xy.ai.workbench.Model.Capabilities;

public class ConfigManager {

	private SessionConfig cfg = new SessionConfig();
	private List<Consumer<SessionConfig>> systemPromptObs = new ArrayList<>();
	private List<Consumer<boolean[]>> inputObs = new ArrayList<>();
	private List<Consumer<InputMode>> inputModeObs = new ArrayList<>();
	private List<Consumer<Model>> modelObs = new ArrayList<>();
	private List<Consumer<String>> keyObs = new ArrayList<>();
	private List<Consumer<Long>> outTokenObs = new ArrayList<>();
	private List<Consumer<Integer>> budgetObs = new ArrayList<>();
	private List<Consumer<Reasoning>> reasonObs = new ArrayList<>();

	public void clearObserver() {
		systemPromptObs.clear();
		inputObs.clear();
		inputModeObs.clear();
		keyObs.clear();
		modelObs.clear();
		outTokenObs.clear();
		budgetObs.clear();
		reasonObs.clear();
	}

	public void setKey(String key) {
		cfg.setKey(key);
		keyObs.forEach(c -> c.accept(key));
	}

	public void setMaxOutputTokens(Long maxOutputTokens) {
		maxOutputTokens = (long) getCapabilities().alignOutpuTokens(maxOutputTokens.intValue());
		cfg.setMaxOutputTokens(maxOutputTokens);
		outTokenObs.forEach(c -> c.accept(cfg.maxOutputTokens));
	}

	public void setTemperature(Double temperature) {
		cfg.setTemperature(temperature);
	}

	public void setTopP(Double topP) {
		cfg.setTopP(topP);
	}

	public void setModel(Model model) {
		cfg.setModel(model);

		if (Arrays.asList(getCapabilities().getReasonings()).indexOf(cfg.reasoning) == -1)
			setReasoning(getCapabilities().getReasonings()[0]);
		setMaxOutputTokens((long) getCapabilities().alignOutpuTokens(Integer.MAX_VALUE));
		setReasoningBudget(getCapabilities().alignBudget(Integer.MAX_VALUE));
		modelObs.forEach(c -> c.accept(model));
	}

	public Integer getReasoningBudget() {
		return cfg.getReasoningBudget();
	}

	public void setReasoningBudget(Integer reasoningBudget) {
		reasoningBudget = getCapabilities().alignBudget(reasoningBudget);
		cfg.setReasoningBudget(reasoningBudget);
		budgetObs.forEach(c -> c.accept(cfg.reasoningBudget));
	}

	public void setReasoning(Reasoning reasoning) {
		cfg.setReasoning(reasoning);
		reasonObs.forEach(c -> c.accept(cfg.reasoning));
	}

	public void setSystemPrompt(String[] systemPrompt) {
		cfg.setSystemPrompt(systemPrompt);
		systemPromptObs.forEach(c -> c.accept(cfg));
		inputModeObs.forEach(c -> c.accept(InputMode.Instructions));
	}

	public String getKey() {
		return cfg.getKey();
	}

	public Long getMaxOutputTokens() {
		return cfg.getMaxOutputTokens();
	}

	public Double getTemperature() {
		return cfg.getTemperature();
	}

	public Double getTopP() {
		return cfg.getTopP();
	}

	public Model getModel() {
		return cfg.getModel();
	}

	public Reasoning getReasoning() {
		return cfg.getReasoning();
	}

	public String[] getSystemPrompt() {
		return cfg.getSystemPrompt();
	}

	public void addSystemPromptObs(Consumer<SessionConfig> obs, boolean initialize) {
		systemPromptObs.add(obs);
		if (initialize)
			obs.accept(cfg);
	}

	public void addInputObs(Consumer<boolean[]> obs, boolean initialize) {
		inputObs.add(obs);
		if (initialize)
			obs.accept(cfg.inputModes);
	}

	public void addBudgetObs(Consumer<Integer> obs, boolean initialize) {
		budgetObs.add(obs);
		if (initialize)
			obs.accept(cfg.reasoningBudget);
	}

	public void addReasoningObs(Consumer<Reasoning> obs, boolean initialize) {
		reasonObs.add(obs);
		if (initialize)
			obs.accept(cfg.reasoning);
	}

	public void addKeyObs(Consumer<String> obs, boolean initialize) {
		keyObs.add(obs);
		if (initialize)
			obs.accept(cfg.key);
	}

	public void addModelObs(Consumer<Model> obs, boolean initialize) {
		modelObs.add(obs);
		if (initialize)
			obs.accept(cfg.model);
	}

	public void addOutputTokenObs(Consumer<Long> obs, boolean initialize) {
		outTokenObs.add(obs);
		if (initialize)
			obs.accept(cfg.maxOutputTokens);
	}

	public void addInputModeObs(Consumer<InputMode> obs) {
		inputModeObs.add(obs);
	}

	public OutputMode getOuputMode() {
		return cfg.ouputMode;
	}

	public void setOuputMode(OutputMode ouputMode) {
		cfg.ouputMode = ouputMode;
	}

	public boolean isInputEnabled(InputMode mode) {
		return cfg.isInputEnabled(mode);
	}

	public void setInputMode(InputMode mode, boolean enable) {
		cfg.setInputMode(mode, enable);

		if (InputMode.Selection.equals(mode) && enable) {
			cfg.setInputMode(InputMode.Current_line, false);
			cfg.setInputMode(InputMode.Editor, false);
		} else if (InputMode.Editor.equals(mode) && enable) {
			cfg.setInputMode(InputMode.Current_line, false);
			cfg.setInputMode(InputMode.Selection, false);
		} else if (InputMode.Current_line.equals(mode) && enable) {
			cfg.setInputMode(InputMode.Editor, false);
			cfg.setInputMode(InputMode.Selection, false);
		}

		inputObs.forEach(c -> c.accept(cfg.inputModes));
		inputModeObs.forEach(c -> c.accept(mode));
	}

	public String[] getModels() {
		// TODO based on current key
		String[] options = Arrays.stream(Model.values()).map((m) -> m.name()).collect(Collectors.toList())
				.toArray(new String[0]);
		return options;
	}

	public String[] getReasonings() {
		Reasoning[] reasonings = getCapabilities().getReasonings();
		String[] options = Arrays.stream(reasonings).map((m) -> m.name()).collect(Collectors.toList())
				.toArray(new String[0]);
		return options;
	}

	public void saveConfig(IMemento memento) {
		MementoConverter.saveConfig(memento, cfg);
	}

	public void loadConfig(IMemento memento) {
		MementoConverter.loadConfig(memento, cfg);
	}

	public Capabilities getCapabilities() {
		return cfg.model.cap;
	}
}
