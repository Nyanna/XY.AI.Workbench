package xy.ai.conscious.commands.include;

class IncludeArgs {

  /** Removes surrounding whitespace and a leading and trailing backtick, e.g. {@code `/home/user/ToolResult.java`}. */
  static String path(String arg) {
    String s = arg.strip();
    if (s.startsWith("`"))
      s = s.substring(1);
    if (s.endsWith("`"))
      s = s.substring(0, s.length() - 1);
    return s.strip();
  }
}
