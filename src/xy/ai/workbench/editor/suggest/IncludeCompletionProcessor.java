package xy.ai.workbench.editor.suggest;

import java.io.File;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.function.Supplier;
import org.eclipse.jface.text.ITextViewer;
import org.eclipse.jface.text.contentassist.CompletionProposal;
import org.eclipse.jface.text.contentassist.ICompletionProposal;
import org.eclipse.jface.text.contentassist.IContentAssistProcessor;
import org.eclipse.jface.text.contentassist.IContextInformation;
import org.eclipse.jface.text.contentassist.IContextInformationValidator;
import xy.ai.workbench.editor.suggest.IncludeSyntax.Argument;

public class IncludeCompletionProcessor implements IContentAssistProcessor {
  private static final char[] ACTIVATION = { ' ', '(', '/', '\\' };
  private final IncludeContextParser parser = new IncludeContextParser();
  private final IncludeSyntax syntax = new IncludeSyntax();
  private final PathSuggestions paths = new PathSuggestions();
  private final Supplier<Path> baseDirectory;

  /**
   * @param baseDirectory directory that relative paths are resolved against, may return {@code null}
   */
  public IncludeCompletionProcessor(Supplier<Path> baseDirectory) {
    this.baseDirectory = baseDirectory;
  }

  @Override
  public ICompletionProposal[] computeCompletionProposals(ITextViewer viewer, int offset) {
    IncludeContext context = parser.parse(viewer.getDocument(), offset);
    if (context == null)
      return null;
    List<ICompletionProposal> proposals = switch(context.part()) {
      case TAG ->
        tagProposals(context, offset);
      case KIND ->
        kindProposals(context, offset);
      case ARGUMENT ->
        argumentProposals(context, offset);
    };
    return proposals.isEmpty() ? null : proposals.toArray(new ICompletionProposal[0]);
  }

  private List<ICompletionProposal> tagProposals(IncludeContext context, int offset) {
    List<ICompletionProposal> result = new ArrayList<>();
    result.add(tag(context, offset, null));
    for (String kind : syntax.kinds()) result.add(tag(context, offset, kind));
    return result;
  }

  private ICompletionProposal tag(IncludeContext context, int offset, String kind) {
    String text = "include" + (kind == null ? "" : " " + kind) + "](" + closing(context.rest());
    String display = kind == null ? "include (path)" : "include " + kind;
    return new CompletionProposal(text, context.start(), offset - context.start(), text.indexOf('(') + 1, null, display, null, null);
  }

  private List<ICompletionProposal> kindProposals(IncludeContext context, int offset) {
    List<ICompletionProposal> result = new ArrayList<>();
    boolean closed = context.rest().startsWith("]");
    for (String kind : syntax.kinds()) {
      if (!kind.startsWith(context.prefix()))
        continue;
      String text = closed ? kind : kind + "](" + closing(context.rest());
      int cursor = closed ? text.length() : text.indexOf('(') + 1;
      result.add(new CompletionProposal(text, context.start(), offset - context.start(), cursor, null, kind, null, null));
    }
    return result;
  }

  private List<ICompletionProposal> argumentProposals(IncludeContext context, int offset) {
    String kind = context.kind();
    Argument argument = syntax.argument(kind);
    return argument == Argument.FIXED ? fixedProposals(context, offset) : pathProposals(context, offset, argument == Argument.DIRECTORY);
  }

  private List<ICompletionProposal> fixedProposals(IncludeContext context, int offset) {
    List<ICompletionProposal> result = new ArrayList<>();
    String typed = context.prefix().strip();
    int start = offset - context.prefix().length();
    for (String value : syntax.fixedValues(context.kind())) {
      if (!value.startsWith(typed))
        continue;
      String text = value + closing(context.rest());
      result.add(new CompletionProposal(text, start, context.prefix().length(), text.length(), null, value, null, null));
    }
    return result;
  }

  private List<ICompletionProposal> pathProposals(IncludeContext context, int offset, boolean directoriesOnly) {
    int skipped = leadingNoise(context.prefix());
    String typed = context.prefix().substring(skipped);
    int start = context.start() + skipped;
    List<ICompletionProposal> result = new ArrayList<>();
    for (PathSuggestions.Entry entry : paths.list(typed, baseDirectory.get(), directoriesOnly)) {
      String text = entry.path().toString();
      String display = entry.name();
      int cursorOffset;
      if (entry.directory()) {
        text += File.separator;
        display += File.separator;
        cursorOffset = text.length();
      } else {
        text += closing(context.rest());
        cursorOffset = text.length();
      }
      result.add(new CompletionProposal(text, start, offset - start, cursorOffset, null, display, null, entry.path().toString()));
    }
    return result;
  }

  /**
   * Whitespace and one backtick, which {@code IncludeArgs.path} strips as well.
   */
  private int leadingNoise(String prefix) {
    int i = 0;
    while (i < prefix.length() && Character.isWhitespace(prefix.charAt(i))) i++;
    if (i < prefix.length() && prefix.charAt(i) == '`')
      i++;
    return i;
  }

  private String closing(String rest) {
    return rest.startsWith(")") ? "" : ")";
  }

  @Override
  public IContextInformation[] computeContextInformation(ITextViewer viewer, int offset) {
    return null;
  }

  @Override
  public char[] getCompletionProposalAutoActivationCharacters() {
    return ACTIVATION;
  }

  @Override
  public char[] getContextInformationAutoActivationCharacters() {
    return null;
  }

  @Override
  public String getErrorMessage() {
    return null;
  }

  @Override
  public IContextInformationValidator getContextInformationValidator() {
    return null;
  }
}
