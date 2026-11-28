package xy.ai.workbench.editor.navigate;

import java.util.regex.Matcher;
import java.util.regex.Pattern;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.ITextViewer;
import org.eclipse.jface.text.hyperlink.IHyperlink;
import org.eclipse.swt.widgets.Display;

class ToolCallHyperlink implements IHyperlink {
  private final ITextViewer viewer;
  private final IRegion region;
  private final String callId;

  ToolCallHyperlink(ITextViewer viewer, IRegion region, String callId) {
    this.viewer = viewer;
    this.region = region;
    this.callId = callId;
  }

  @Override
  public IRegion getHyperlinkRegion() {
    return region;
  }

  @Override
  public String getTypeLabel() {
    return "Go to Tool Call";
  }

  @Override
  public String getHyperlinkText() {
    return "Go to tool call " + callId;
  }

  @Override
  public void open() {
    if (viewer.getDocument() == null)
      return;
    Pattern idLine = Pattern.compile("^[ \\t]*(?:-[ \\t]+)?id:[ \\t]*[\"']?(" + Pattern.quote(callId) + ")[\"']?[ \\t]*$", Pattern.MULTILINE);
    Matcher match = idLine.matcher(viewer.getDocument().get());
    if (!match.find()) {
      Display.getDefault().beep();
      return;
    }
    viewer.setSelectedRange(match.start(1), callId.length());
    viewer.revealRange(match.start(1), callId.length());
  }
}
