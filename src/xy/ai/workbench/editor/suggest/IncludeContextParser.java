package xy.ai.workbench.editor.suggest;

import java.util.regex.Matcher;
import java.util.regex.Pattern;
import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IRegion;
import xy.ai.workbench.editor.suggest.IncludeContext.Part;

/**
 * Locates the caret within the include syntax. Include tags are only recognised
 * on lines that consist of include tags only, as the include processor does.
 */
class IncludeContextParser {

  private static final String KEYWORD = "include";
  private static final Pattern COMPLETE_TAGS = Pattern.compile("(?:\\s*\\[include(?:\\s+\\w+)?\\]\\([^)]*\\))*\\s*");
  private static final Pattern ARGUMENT = Pattern.compile("\\[include(?:\\s+(\\w+))?\\]\\(([^)]*)");
  private static final Pattern HEADER = Pattern.compile("\\[(\\S*)(\\s+(\\w*))?");

  IncludeContext parse(IDocument document, int offset) {
    try {
      IRegion line = document.getLineInformationOfOffset(offset);
      String before = document.get(line.getOffset(), offset - line.getOffset());
      String rest = document.get(offset, line.getOffset() + line.getLength() - offset);
      return parse(before, rest, line.getOffset());
    } catch (BadLocationException e) {
      return null;
    }
  }

  IncludeContext parse(String before, String rest, int lineOffset) {
    Matcher head = COMPLETE_TAGS.matcher(before);
    head.lookingAt();
    int tagStart = head.end();
    String tag = before.substring(tagStart);
    if (!tag.startsWith("["))
      return null;

    Matcher argument = ARGUMENT.matcher(tag);
    if (argument.matches())
      return new IncludeContext(Part.ARGUMENT, argument.group(1), argument.group(2), lineOffset + tagStart + argument.start(2), rest);

    Matcher header = HEADER.matcher(tag);
    if (!header.matches())
      return null;
    String word = header.group(1);
    if (header.group(2) == null)
      return KEYWORD.startsWith(word) ? new IncludeContext(Part.TAG, null, word, lineOffset + tagStart + 1, rest) : null;
    if (!KEYWORD.equals(word))
      return null;
    return new IncludeContext(Part.KIND, null, header.group(3), lineOffset + before.length() - header.group(3).length(), rest);
  }
}
