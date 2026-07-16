package xy.ai.workbench;

public enum CacheMode {
	Default, Disabled, Minutes_5, Hours_1;

	public static final CacheMode[] ClaudeCode = new CacheMode[] { CacheMode.Default, CacheMode.Disabled,
			CacheMode.Hours_1, CacheMode.Minutes_5 };
}
