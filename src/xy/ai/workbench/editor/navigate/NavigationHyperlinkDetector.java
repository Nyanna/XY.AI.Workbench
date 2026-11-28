package xy.ai.workbench.editor.navigate;

import java.nio.file.Files;
import java.nio.file.InvalidPathException;
import java.nio.file.Path;
import java.util.function.Supplier;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.ITextViewer;
import org.eclipse.jface.text.Region;
import org.eclipse.jface.text.hyperlink.AbstractHyperlinkDetector;
import org.eclipse.jface.text.hyperlink.IHyperlink;

/**
 * Detects navigable references: file paths (backticked or as include argument) and
 * {@code /answer <session>-<result>-<call> allow|deny|exec} commands, which point to the originating tool call.
 */
public class NavigationHyperlinkDetector extends AbstractHyperlinkDetector {
  private static final Pattern INCLUDE = Pattern.compile("\\[include(?:\\s+(\\w+))?\\]\\(([^)]*)\\)");
  private static final Pattern BACKTICK = Pattern.compile("`([^`]+)`");
  private static final Pattern ANSWER = Pattern.compile("^\\s*/answer\\s+(\\S+)\\s+(?:allow|deny|exec)\\b", Pattern.CASE_INSENSITIVE);
  private static final Pattern CALL_ID = Pattern.compile("([^-\\s]+)-\\d+-(\\d+)");

  private final Supplier<Path> baseDirectory;

  public NavigationHyperlinkDetector(Supplier<Path> baseDirectory) {
    this.baseDirectory = baseDirectory;
  }

  @Override
  public IHyperlink[] detectHyperlinks(ITextViewer viewer, IRegion region, boolean canShowMultipleHyperlinks) {
    IDocument document = viewer.getDocument();
    if (document == null)
      return null;
    try {
      IRegion line = document.getLineInformationOfOffset(region.getOffset());
      String text = document.get(line.getOffset(), line.getLength());
      int caret = region.getOffset() - line.getOffset();
      IHyperlink link = detectAnswer(viewer, text, caret, line.getOffset());
      if (link == null)
        link = detectInclude(text, caret, line.getOffset());
      if (link == null)
        link = detectBacktick(text, caret, line.getOffset());
      return link == null ? null : new IHyperlink[] { link };
    } catch (BadLocationException e) {
      return null;
    }
  }

  private IHyperlink detectAnswer(ITextViewer viewer, String text, int caret, int lineOffset) {
    Matcher answer = ANSWER.matcher(text);
    if (!answer.find() || !contains(answer.start(1), answer.end(1), caret))
      return null;
    Matcher id = CALL_ID.matcher(answer.group(1));
    if (!id.matches())
      return null;
    String callId = id.group(1) + "-" + id.group(2) + "-0";
    return new ToolCallHyperlink(viewer, new Region(lineOffset + answer.start(1), answer.end(1) - answer.start(1)), callId);
  }

  private IHyperlink detectInclude(String text, int caret, int lineOffset) {
    Matcher include = INCLUDE.matcher(text);
    while (include.find()) {
      if ("search".equals(include.group(1)))
        continue;
      int start = include.start(2);
      int end = include.end(2);
      while (start < end && isPadding(text.charAt(start)))
        start++;
      while (end > start && isPadding(text.charAt(end - 1)))
        end--;
      if (contains(start, end, caret))
        return fileLink(text.substring(start, end), lineOffset + start, end - start);
    }
    return null;
  }

  private IHyperlink detectBacktick(String text, int caret, int lineOffset) {
    Matcher quoted = BACKTICK.matcher(text);
    while (quoted.find()) {
      if (contains(quoted.start(1), quoted.end(1), caret))
        return fileLink(quoted.group(1), lineOffset + quoted.start(1), quoted.end(1) - quoted.start(1));
    }
    return null;
  }

  private IHyperlink fileLink(String pathText, int offset, int length) {
    Path file = resolve(pathText.strip());
    return file == null ? null : new FileHyperlink(new Region(offset, length), file);
  }

  private Path resolve(String pathText) {
    if (pathText.isEmpty())
      return null;
    try {
      Path path = Path.of(pathText);
      if (!path.isAbsolute()) {
        Path base = baseDirectory.get();
        if (base == null)
          return null;
        path = base.resolve(path);
      }
      path = path.normalize();
      return Files.isRegularFile(path) ? path : null;
    } catch (InvalidPathException e) {
      return null;
    }
  }

  private static boolean isPadding(char c) {
    return c == '`' || Character.isWhitespace(c);
  }

  private static boolean contains(int start, int end, int offset) {
    return offset >= start && offset <= end;
  }
}
