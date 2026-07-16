package xy.ai.workbench;

public enum Reasoning {
	minimal, low, medium, high, xhigh, max, Budget, Disabled, Unlimited;

	public static final Reasoning[] ClaudeCode = new Reasoning[] { Reasoning.Disabled, Reasoning.low, Reasoning.medium,
			Reasoning.high, Reasoning.xhigh, Reasoning.max };

	public static final Reasoning[] Budgets = new Reasoning[] { Reasoning.Unlimited, Reasoning.Budget,
			Reasoning.Disabled };
	public static final Reasoning[] OpenAI = new Reasoning[] { Reasoning.high, Reasoning.medium, Reasoning.low,
			Reasoning.minimal };
}