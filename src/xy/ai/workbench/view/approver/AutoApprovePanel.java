package xy.ai.workbench.view.approver;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.EnumSet;
import java.util.Set;
import java.util.function.Consumer;
import org.eclipse.jface.action.Action;
import org.eclipse.jface.action.IAction;
import org.eclipse.jface.resource.JFaceResources;
import org.eclipse.swt.SWT;
import org.eclipse.swt.custom.StyledText;
import org.eclipse.swt.widgets.Display;
import org.eclipse.swt.widgets.Menu;
import org.eclipse.swt.widgets.MenuItem;
import org.eclipse.ui.IActionBars;
import org.eclipse.ui.part.ViewPart;
import org.eclipse.swt.widgets.Composite;
import com.fasterxml.jackson.databind.JsonNode;
import xy.ai.conscious.autoapprove.ApprovalItem;
import xy.ai.conscious.autoapprove.AutoApprover;
import xy.ai.conscious.autoapprove.Decision;
import xy.ai.workbench.Activator;
import org.eclipse.ui.ISharedImages;
import org.eclipse.ui.IWorkbenchPage;
import org.eclipse.ui.PartInitException;
import java.io.File;
import org.eclipse.jface.action.ToolBarManager;
import org.eclipse.swt.graphics.Point;
import org.eclipse.swt.graphics.Rectangle;
import org.eclipse.swt.widgets.ToolBar;
import org.eclipse.swt.widgets.ToolItem;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import org.eclipse.core.resources.IFile;
import org.eclipse.ui.IEditorInput;
import org.eclipse.ui.IEditorPart;
import org.eclipse.ui.IPartListener2;
import org.eclipse.ui.IURIEditorInput;
import org.eclipse.ui.IWorkbenchPart;
import org.eclipse.ui.IWorkbenchPartReference;
import xy.ai.workbench.editor.AISessionEditor;
import xy.ai.workbench.view.PartListener2Adapter;
import org.eclipse.ui.PlatformUI;
import java.util.ArrayList;
import xy.ai.conscious.connector.harness.PromptHandler;
import xy.ai.conscious.model.AIAnswer;
import xy.ai.conscious.model.ModelMessage;
import java.util.List;
import org.eclipse.swt.custom.SashForm;

/**
 * Console like log of the decisions of the auto approver for all sessions.
 */
public class AutoApprovePanel extends ViewPart {
  public static final String ID = "xy.ai.workbench.view.approver.AutoApprovePanel";
  private static final int MAX_LINES = 1000;
  private static final int SESSION_WIDTH = 10;
  private static final int ACTION_WIDTH = 9;
  private static final int TOOL_WIDTH = 16;
  private static final int CHARS_WIDTH = 7;
  private static final int DETAIL_MAX = 200;

  private enum Filter {

    Allow("Allow"), Denied("Denied"), NoAction("No Action"), Message("Model Messages");
    private final String label;

    Filter(String label) {
      this.label = label;
    }
  }

  private record Entry(Filter filter, String sessionId, String line, String detail) {
  }
  private final Deque<Entry> buffer = new ArrayDeque<>();
  private final List<Entry> visible = new ArrayList<>();
  private final Set<Filter> shown = EnumSet.allOf(Filter.class);
  private final Consumer<Decision> observer = this::onDecision;
  private final Consumer<AIAnswer> answerObserver = this::onAnswer;
  private PromptHandler promptHandler;
  private Display display;
  private SashForm sash;
  private StyledText console;
  private StyledText details;
  private AutoApprover approver;
  private boolean showOnChange = true;
  private boolean followEditor;
  private String activePath;
  private final Map<String, String> sessionPaths = new ConcurrentHashMap<>();
  private final IPartListener2 partListener = new PartListener2Adapter() {

    @Override
    public void partActivated(IWorkbenchPartReference ref) {
      String path = pathOf(ref.getPart(false));
      if (path != null && !path.equals(activePath)) {
        activePath = path;
        if (followEditor)
          refreshFollow();
      }
    }
  };

  @Override
  public void createPartControl(Composite parent) {
    display = parent.getDisplay();
    sash = new SashForm(parent, SWT.HORIZONTAL);
    console = new StyledText(sash, SWT.READ_ONLY | SWT.V_SCROLL | SWT.H_SCROLL);
    console.setFont(JFaceResources.getTextFont());
    console.setWordWrap(false);
    console.addCaretListener(e -> showDetails(console.getLineAtOffset(e.caretOffset)));
    details = new StyledText(sash, SWT.READ_ONLY | SWT.V_SCROLL | SWT.H_SCROLL);
    details.setFont(JFaceResources.getTextFont());
    details.setWordWrap(true);
    sash.setWeights(new int[] { 60, 40 });
    promptHandler = Activator.getDefault().promptHandler;
    approver = promptHandler.approver();
    IActionBars bars = getViewSite().getActionBars();
    bars.getToolBarManager().add(createDisableAction());
    bars.getToolBarManager().add(createAutoShowAction());
    bars.getToolBarManager().add(createDetailsAction());
    bars.getToolBarManager().add(createFollowAction());
    bars.getToolBarManager().add(createFilterAction());
    IEditorPart current = getSite().getPage().getActiveEditor();
    activePath = pathOf(current);
    getSite().getPage().addPartListener(partListener);
    bars.updateActionBars();
    approver.addListener(observer);
    promptHandler.addAnswerObs(answerObserver);
  }

  private Action createDetailsAction() {
    Action action = new Action("Show Details", IAction.AS_CHECK_BOX) {

      @Override
      public void run() {
        sash.setMaximizedControl(isChecked() ? null : console);
      }
    };
    action.setToolTipText("Show the details of the line at the cursor");
    action.setImageDescriptor(PlatformUI.getWorkbench().getSharedImages().getImageDescriptor(ISharedImages.IMG_OBJ_FILE));
    action.setChecked(true);
    return action;
  }

  private void showDetails(int line) {
    if (details == null || details.isDisposed())
      return;
    details.setText(line >= 0 && line < visible.size() ? visible.get(line).detail() : "");
  }

  private Action createDisableAction() {
    Action action = new Action("Disable Auto Approver", IAction.AS_CHECK_BOX) {

      @Override
      public void run() {
        approver.setEnabled(!isChecked());
      }
    };
    action.setToolTipText("Disable the processing of the auto approver");
    action.setImageDescriptor(PlatformUI.getWorkbench().getSharedImages().getImageDescriptor(ISharedImages.IMG_ELCL_REMOVE));
    action.setChecked(!approver.isEnabled());
    return action;
  }

  private Action createAutoShowAction() {
    Action action = new Action("Show When Content Changes", IAction.AS_CHECK_BOX) {

      @Override
      public void run() {
        showOnChange = isChecked();
      }
    };
    action.setToolTipText("Show this view when new entries are logged");
    action.setImageDescriptor(PlatformUI.getWorkbench().getSharedImages().getImageDescriptor(ISharedImages.IMG_ETOOL_DEF_PERSPECTIVE));
    action.setChecked(showOnChange);
    return action;
  }

  private Action createFilterAction() {
    Action action = new Action("Filter", IAction.AS_PUSH_BUTTON) {

      @Override
      public void run() {
        showFilterMenu();
      }
    };
    action.setToolTipText("Select the action types to show");
    action.setImageDescriptor(PlatformUI.getWorkbench().getSharedImages().getImageDescriptor(ISharedImages.IMG_OBJS_INFO_TSK));
    return action;
  }

  private void showFilterMenu() {
    Menu menu = new Menu(console);
    fillMenu(menu);
    menu.addListener(SWT.Hide, e -> display.asyncExec(menu::dispose));
    Point location = display.getCursorLocation();
    if (getViewSite().getActionBars().getToolBarManager() instanceof ToolBarManager manager && manager.getControl() != null) {
      ToolBar bar = manager.getControl();
      ToolItem item = bar.getItem(bar.toControl(location));
      if (item != null) {
        Rectangle bounds = item.getBounds();
        location = bar.toDisplay(bounds.x, bounds.y + bounds.height);
      }
    }
    menu.setLocation(location);
    menu.setVisible(true);
  }

  private void fillMenu(Menu menu) {
    for (Filter filter : Filter.values()) {
      MenuItem item = new MenuItem(menu, SWT.CHECK);
      item.setText(filter.label);
      item.setSelection(shown.contains(filter));
      item.addListener(SWT.Selection, e -> {
        if (item.getSelection())
          shown.add(filter);
        else
          shown.remove(filter);
        rebuild();
      });
    }
  }

  private Action createFollowAction() {
    Action action = new Action("Active Editor Session Only", IAction.AS_CHECK_BOX) {

      @Override
      public void run() {
        setFollowEditor(isChecked());
      }
    };
    action.setToolTipText("Show only entries of the session of the active editor");
    action.setImageDescriptor(PlatformUI.getWorkbench().getSharedImages().getImageDescriptor(ISharedImages.IMG_ELCL_SYNCED));
    action.setChecked(followEditor);
    return action;
  }

  private void setFollowEditor(boolean follow) {
    followEditor = follow;
    refreshFollow();
  }

  private void refreshFollow() {
    setContentDescription(followEditor ? "Active editor session only" : "");
    rebuild();
  }

  private boolean isVisible(Entry entry) {
    if (!shown.contains(entry.filter()))
      return false;
    return !followEditor || activePath != null && activePath.equals(sessionPaths.get(entry.sessionId()));
  }

  private void registerSession(AIAnswer answer) {
    if (answer.prompt == null || answer.prompt.arg == null || answer.prompt.arg.absoluteFilePath == null)
      return;
    String path = answer.prompt.arg.absoluteFilePath;
    if (answer.sessionId != null)
      sessionPaths.put(answer.sessionId, path);
    sessionPaths.put(answer.prompt.sessionId(), path);
  }

  private static String pathOf(IWorkbenchPart part) {
    IEditorPart editor = part instanceof AISessionEditor session ? session.getEditor() : part instanceof IEditorPart e ? e : null;
    if (editor == null)
      return null;
    IEditorInput input = editor.getEditorInput();
    IFile file = input == null ? null : input.getAdapter(IFile.class);
    if (file != null && file.getLocation() != null)
      return file.getLocation().toFile().getAbsolutePath();
    if (input instanceof IURIEditorInput uriInput)
      return new File(uriInput.getURI()).getAbsolutePath();
    return null;
  }

  private void onAnswer(AIAnswer answer) {
    if (answer == null || answer.messages.isEmpty())
      return;
    registerSession(answer);
    List<Entry> entries = answer.messages.stream().map(m -> toEntry(answer.sessionId, m)).toList();
    display.asyncExec(() -> {
      if (!console.isDisposed())
        entries.forEach(this::append);
    });
  }

  private static Entry toEntry(String sessionId, ModelMessage message) {
    String line = fit(sessionId, SESSION_WIDTH) + " | " + fit(message.kind().name(), ACTION_WIDTH) + " | " + fit("-", TOOL_WIDTH) + " | " + pad(Integer.toString(message.text().length()), CHARS_WIDTH) + " | " + pad("", 15) + shorten(message.text(), DETAIL_MAX);
    return new Entry(Filter.Message, sessionId, line, message.text());
  }

  private void onDecision(Decision decision) {
    Entry entry = toEntry(decision);
    display.asyncExec(() -> {
      if (!console.isDisposed())
        append(entry);
    });
  }

  private void append(Entry entry) {
    buffer.addLast(entry);
    if (buffer.size() > MAX_LINES && isVisible(buffer.removeFirst())) {
      visible.remove(0);
      console.replaceTextRange(0, console.getOffsetAtLine(1), "");
    }
    if (isVisible(entry)) {
      visible.add(entry);
      console.append(entry.line() + "\n");
      scrollToEnd();
      if (showOnChange)
        try {
          getSite().getPage().showView(ID, null, IWorkbenchPage.VIEW_VISIBLE);
        } catch (PartInitException e) {
          // ignore not present
        }
    }
  }

  private void rebuild() {
    StringBuilder text = new StringBuilder();
    visible.clear();
    for (Entry entry : buffer) {
      if (isVisible(entry)) {
        visible.add(entry);
        text.append(entry.line()).append('\n');
      }
    }
    console.setText(text.toString());
    scrollToEnd();
  }

  private void scrollToEnd() {
    int last = visible.size() - 1;
    console.setCaretOffset(last < 0 ? 0 : console.getOffsetAtLine(last));
    console.showSelection();
    showDetails(last);
  }

  private static Entry toEntry(Decision d) {
    ApprovalItem item = d.item();
    boolean isResult = item != null && (item.kind() == ApprovalItem.Kind.ToolResult || item.kind() == ApprovalItem.Kind.ControlResult);
    Filter filter = switch(d.result()) {
      case Allow, Call ->
        Filter.Allow;
      case Deny ->
        Filter.Denied;
      case Pass ->
        Filter.NoAction;
    };
    String action = switch(filter) {
      case Allow ->
        "Approved";
      case Denied ->
        "Denied";
      case NoAction, Message ->
        "Pass";
    };
    String tool = item == null ? "-" : isResult ? "Result" : item.tool();
    String detail = item == null ? "" : text(isResult ? item.result() : item.arguments());
    String reason = item == null || isResult ? "" : oneLine(item.arguments().path("reason").asText(""));
    String line = fit(d.sessionId(), SESSION_WIDTH) + " | " + fit(action, ACTION_WIDTH) + " | " + fit(tool, TOOL_WIDTH) + " | " + pad(Integer.toString(d.answerLength()), CHARS_WIDTH);
    if (reason.length() > 0)
      line += " | " + reason + ":   " + shorten(detail, DETAIL_MAX);
    else
      line += " | " + pad("", 15) + shorten(detail, DETAIL_MAX);
    return new Entry(filter, d.sessionId(), line, details(d, isResult));
  }

  private static String details(Decision d, boolean isResult) {
    ApprovalItem item = d.item();
    if (item == null)
      return d.instruction().isBlank() ? d.reason() : d.instruction();
    if (isResult) {
      JsonNode results = item.result().path("structuredContent").path("results");
      return join(d.reason(), pretty(results.isMissingNode() ? item.result() : results));
    }
    String reason = item.arguments().path("reason").asText("");
    return join(reason.isBlank() ? d.reason() : reason, pretty(item.arguments()));
  }

  private static String join(String reason, String body) {
    return reason.isBlank() ? body : reason.strip() + "\n\n" + body;
  }

  private static String pretty(JsonNode node) {
    return node.isTextual() ? node.asText() : node.toPrettyString();
  }

  private static String text(JsonNode node) {
    return node.isTextual() ? node.asText() : node.toString();
  }

  private static String oneLine(String s) {
    return s == null ? "" : s.replaceAll("\\s+", " ").strip();
  }

  private static String shorten(String s, int max) {
    String line = oneLine(s);
    return line.length() <= max ? line : line.substring(0, max - 1) + "…";
  }

  private static String fit(String s, int width) {
    return String.format("%-" + width + "s", shorten(s, width));
  }

  private static String pad(String s, int width) {
    return String.format("%" + width + "s", s);
  }

  @Override
  public void dispose() {
    getSite().getPage().removePartListener(partListener);
    if (approver != null)
      approver.removeListener(observer);
    if (promptHandler != null)
      promptHandler.removeAnswerObs(answerObserver);
    super.dispose();
  }

  @Override
  public void setFocus() {
    // disabled
  }
}
