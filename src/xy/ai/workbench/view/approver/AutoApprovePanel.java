package xy.ai.workbench.view.approver;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.EnumSet;
import java.util.Set;
import java.util.function.Consumer;
import org.eclipse.jface.action.Action;
import org.eclipse.jface.action.IAction;
import org.eclipse.jface.action.IMenuCreator;
import org.eclipse.jface.resource.JFaceResources;
import org.eclipse.swt.SWT;
import org.eclipse.swt.custom.StyledText;
import org.eclipse.swt.widgets.Control;
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

/**
 * Console like log of the decisions of the auto approver for all sessions.
 */
public class AutoApprovePanel extends ViewPart {
  public static final String ID = "xy.ai.workbench.view.approver.AutoApprovePanel";
  private static final int MAX_LINES = 1000;
  private static final int SESSION_WIDTH = 10;
  private static final int ACTION_WIDTH = 8;
  private static final int TOOL_WIDTH = 24;
  private static final int REASON_WIDTH = 30;
  private static final int CHARS_WIDTH = 7;
  private static final int DETAIL_MAX = 200;

  private enum Filter {

    Allow("Allow"), Denied("Denied"), NoAction("No Action");
    private final String label;

    Filter(String label) {
      this.label = label;
    }
  }

  private record Entry(Filter filter, String line) {
  }
  private final Deque<Entry> buffer = new ArrayDeque<>();
  private final Set<Filter> shown = EnumSet.of(Filter.Allow, Filter.Denied);
  private final Consumer<Decision> observer = this::onDecision;
  private Display display;
  private StyledText console;
  private AutoApprover approver;

  @Override
  public void createPartControl(Composite parent) {
    display = parent.getDisplay();
    console = new StyledText(parent, SWT.READ_ONLY | SWT.V_SCROLL | SWT.H_SCROLL);
    console.setFont(JFaceResources.getTextFont());
    console.setWordWrap(false);
    IActionBars bars = getViewSite().getActionBars();
    bars.getToolBarManager().add(createFilterAction());
    bars.updateActionBars();
    approver = Activator.getDefault().promptHandler.approver();
    approver.addListener(observer);
  }

  private Action createFilterAction() {
    Action action = new Action("Filter", IAction.AS_DROP_DOWN_MENU) {
    };
    action.setToolTipText("Select the action types to show");
    action.setMenuCreator(new IMenuCreator() {
      private Menu menu;

      @Override
      public Menu getMenu(Control parent) {
        dispose();
        menu = new Menu(parent);
        fillMenu(menu);
        return menu;
      }

      @Override
      public Menu getMenu(Menu parent) {
        dispose();
        menu = new Menu(parent);
        fillMenu(menu);
        return menu;
      }

      @Override
      public void dispose() {
        if (menu != null && !menu.isDisposed())
          menu.dispose();
        menu = null;
      }
    });
    return action;
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

  private void onDecision(Decision decision) {
    Entry entry = toEntry(decision);
    display.asyncExec(() -> {
      if (!console.isDisposed())
        append(entry);
    });
  }

  private void append(Entry entry) {
    buffer.addLast(entry);
    if (buffer.size() > MAX_LINES && shown.contains(buffer.removeFirst().filter()))
      console.replaceTextRange(0, console.getOffsetAtLine(1), "");
    if (shown.contains(entry.filter())) {
      console.append(entry.line() + "\n");
      scrollToEnd();
    }
  }

  private void rebuild() {
    StringBuilder text = new StringBuilder();
    for (Entry entry : buffer) {
      if (shown.contains(entry.filter()))
        text.append(entry.line()).append('\n');
    }
    console.setText(text.toString());
    scrollToEnd();
  }

  private void scrollToEnd() {
    console.setCaretOffset(console.getCharCount());
    console.showSelection();
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
      case NoAction ->
        "Pass";
    };
    String tool = item == null ? "-" : isResult ? "Result" : item.tool();
    String detail = item == null ? "" : text(isResult ? item.result() : item.arguments());
    String line = fit(d.sessionId(), SESSION_WIDTH) + " " + fit(action, ACTION_WIDTH) + " " + fit(tool, TOOL_WIDTH) + " " + fit(d.reason(), REASON_WIDTH) + " " + pad(Integer.toString(d.answerLength()), CHARS_WIDTH) + " " + shorten(detail, DETAIL_MAX);
    return new Entry(filter, line);
  }

  private static String text(JsonNode node) {
    return node.isTextual() ? node.asText() : node.toString();
  }

  private static String oneLine(String s) {
    return s == null ? "" : s.replaceAll("\\s+", " ").strip();
  }

  private static String shorten(String s, int max) {
    String line = oneLine(s);
    return line.length() <= max ? line : line.substring(0, max - 3) + "...";
  }

  private static String fit(String s, int width) {
    return String.format("%-" + width + "s", shorten(s, width));
  }

  private static String pad(String s, int width) {
    return String.format("%" + width + "s", s);
  }

  @Override
  public void dispose() {
    if (approver != null)
      approver.removeListener(observer);
    super.dispose();
  }

  @Override
  public void setFocus() {
    console.setFocus();
  }
}
