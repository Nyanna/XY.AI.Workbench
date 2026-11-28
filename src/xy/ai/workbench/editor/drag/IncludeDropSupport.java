package xy.ai.workbench.editor.drag;

import java.util.ArrayList;
import java.util.List;
import org.eclipse.jface.text.BadLocationException;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.ITextViewerExtension5;
import org.eclipse.jface.text.TextUtilities;
import org.eclipse.jface.text.source.ISourceViewer;
import org.eclipse.swt.custom.StyledText;
import org.eclipse.swt.dnd.DND;
import org.eclipse.swt.dnd.DropTarget;
import org.eclipse.swt.dnd.DropTargetAdapter;
import org.eclipse.swt.dnd.DropTargetEvent;
import org.eclipse.swt.dnd.DropTargetListener;
import org.eclipse.swt.dnd.FileTransfer;
import org.eclipse.swt.dnd.Transfer;
import org.eclipse.swt.dnd.TransferData;
import org.eclipse.swt.graphics.Point;
import org.eclipse.swt.widgets.Listener;
import org.eclipse.swt.widgets.TypedListener;

/**
 * Accepts files and directories dropped on a source viewer and inserts one include tag per entry,
 * each on its own line. Other drops (e.g. text) are still handled by the existing listeners.
 */
public final class IncludeDropSupport {

  private static final FileTransfer FILES = FileTransfer.getInstance();

  private final ISourceViewer viewer;
  private final StyledText widget;

  private IncludeDropSupport(ISourceViewer viewer, StyledText widget) {
    this.viewer = viewer;
    this.widget = widget;
  }

  public static void install(ISourceViewer viewer) {
    if (viewer != null && viewer.getTextWidget() != null)
      new IncludeDropSupport(viewer, viewer.getTextWidget()).attach();
  }

  private void attach() {
    DropTarget target = widget.getData(DND.DROP_TARGET_KEY) instanceof DropTarget existing ? existing : new DropTarget(widget, DND.DROP_COPY | DND.DROP_MOVE);

    List<Transfer> transfers = new ArrayList<>(List.of(target.getTransfer()));
    if (!transfers.contains(FILES))
      transfers.add(FILES);
    target.setTransfer(transfers.toArray(Transfer[]::new));

    // The default text drop handler cannot cope with file data, so it is only called for non-file drops.
    List<DropTargetListener> previous = new ArrayList<>();
    for (Listener listener : target.getListeners(DND.Drop))
      if (listener instanceof TypedListener typed && typed.getEventListener() instanceof DropTargetListener drop)
        previous.add(drop);
    previous.forEach(target::removeDropListener);
    target.addDropListener(new Dispatcher(previous));
  }

  private final class Dispatcher extends DropTargetAdapter {
    private final List<DropTargetListener> previous;

    Dispatcher(List<DropTargetListener> previous) {
      this.previous = previous;
    }

    @Override
    public void dragEnter(DropTargetEvent event) {
      accept(event);
    }

    @Override
    public void dragOperationChanged(DropTargetEvent event) {
      accept(event);
    }

    @Override
    public void dragOver(DropTargetEvent event) {
      if (FILES.isSupportedType(event.currentDataType))
        event.detail = DND.DROP_COPY;
    }

    @Override
    public void drop(DropTargetEvent event) {
      if (FILES.isSupportedType(event.currentDataType)) {
        if (event.data instanceof String[] paths)
          insert(IncludeTags.of(paths), offsetAt(event));
        return;
      }
      for (DropTargetListener listener : previous)
        listener.drop(event);
    }

    private void accept(DropTargetEvent event) {
      for (TransferData type : event.dataTypes) {
        if (FILES.isSupportedType(type)) {
          event.currentDataType = type;
          event.detail = DND.DROP_COPY;
          return;
        }
      }
    }
  }

  private int offsetAt(DropTargetEvent event) {
    Point point = widget.getDisplay().map(null, widget, event.x, event.y);
    int offset;
    try {
      offset = widget.getOffsetAtPoint(point);
    } catch (IllegalArgumentException e) {
      offset = -1;
    }
    return offset < 0 ? widget.getCaretOffset() : offset;
  }

  private void insert(List<String> tags, int widgetOffset) {
    IDocument document = viewer.getDocument();
    if (document == null || tags.isEmpty())
      return;
    int offset = viewer instanceof ITextViewerExtension5 ext ? ext.widgetOffset2ModelOffset(widgetOffset) : widgetOffset;
    if (offset < 0)
      return;
    try {
      String delimiter = TextUtilities.getDefaultLineDelimiter(document);
      IRegion line = document.getLineInformationOfOffset(offset);
      boolean textBefore = !document.get(line.getOffset(), offset - line.getOffset()).isBlank();
      boolean textAfter = !document.get(offset, line.getOffset() + line.getLength() - offset).isBlank();
      String text = (textBefore ? delimiter : "") + String.join(delimiter, tags) + (textAfter ? delimiter : "");
      document.replace(offset, 0, text);
      int caret = offset + text.length();
      viewer.setSelectedRange(caret, 0);
      viewer.revealRange(caret, 0);
      widget.setFocus();
    } catch (BadLocationException e) {
      // drop position vanished while the document changed, nothing to insert
    }
  }
}
