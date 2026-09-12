package xy.ai.workbench;

public enum CacheMode {
	Default, Disabled, Minutes_5, Minutes_30 , Hours_1;

	public static final CacheMode[] ClaudeCode = new CacheMode[] { CacheMode.Minutes_5, CacheMode.Default, CacheMode.Disabled,
			CacheMode.Hours_1 };
	public static final CacheMode[] ChatGPT = new CacheMode[] { CacheMode.Disabled,
			CacheMode.Minutes_30 };
	// deepseek/chatgpt is at least 30 minutes
}
