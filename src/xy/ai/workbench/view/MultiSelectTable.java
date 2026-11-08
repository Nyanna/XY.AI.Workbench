package xy.ai.workbench.view;

import org.eclipse.swt.SWT;
import org.eclipse.swt.graphics.Point;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Event;
import org.eclipse.swt.widgets.Table;
import org.eclipse.swt.widgets.TableColumn;
import org.eclipse.swt.widgets.TableItem;
import xy.ai.workbench.view.session.Clearable;

/**
 * List-like multi select with the following click behaviour:
 * <ul>
 * <li>plain click: toggles the clicked row</li>
 * <li>CTRL + click: only the clicked row is active</li>
 * <li>SHIFT + click: rows from the anchor (last clicked row) to the clicked row
 * are active</li>
 * <li>SPACE: toggles the row with the keyboard cursor</li>
 * </ul>
 * <p>
 * The "active" state of a row is stored by this class in the row itself
 * ({@link TableItem#setData(Object)}) and is never touched by the native
 * selection of the table. The native selection only serves as the cursor. So
 * there is no second copy of the state that could get out of sync, and
 * programmatic changes ({@link #setItems(String[])},
 * {@link #setSelection(String[])}) never influence the next click.
 * <p>
 * The clicked row is determined with {@link Table#getItem(Point)}.
 * <p>
 * Display: either the active rows are drawn like selected rows and the native
 * highlight is suppressed ({@code markerColumn == false}), or an additional
 * first column shows a check mark for active rows ({@code markerColumn == true},
 * no custom drawing).
 * <p>
 * {@code onChange} is called after each change by the user. Programmatic
 * changes do not call it.
 */
public class MultiSelectTable implements Clearable {
  private static final int MARKER_WIDTH = 28;
  private final Table table;
  private final Runnable onChange;
  private final boolean marker;
  private TableColumn markerCol;
  private final TableColumn textCol;
  private int anchor = -1;
  private int cursor = -1;
  /**
   * Row states right after the anchor click; basis for (re-adjustable) shift ranges.
   */
  private boolean[] baseline;

  public MultiSelectTable(Composite parent, Runnable onChange) {
    this(parent, false, onChange);
  }

  public MultiSelectTable(Composite parent, boolean markerColumn, Runnable onChange) {
    this.onChange = onChange;
    this.marker = markerColumn;
    table = new Table(parent, SWT.SINGLE | SWT.V_SCROLL | SWT.FULL_SELECTION);
    table.setHeaderVisible(false);
    table.setLinesVisible(false);
    if (marker)
      markerCol = new TableColumn(table, SWT.CENTER);
    textCol = new TableColumn(table, SWT.NONE);
    table.addListener(SWT.Resize, e -> layoutColumns());
    table.addListener(SWT.MouseDown, this::onMouseDown);
    table.addListener(SWT.KeyDown, this::onKeyDown);
    if (!marker) {
      table.addListener(SWT.Selection, this::onNativeSelection);
      table.addListener(SWT.MouseUp, e -> dropNativeSelectionLater());
    }
  }

  private void onMouseDown(Event e) {
    if (e.button != 1)
      return;
    table.setFocus();
    TableItem item = table.getItem(new Point(e.x, e.y));
    if (item == null)
      return;
    click(table.indexOf(item), e.stateMask);
    if (!marker) {
      e.doit = false;
      dropNativeSelectionLater();
    }
  }

  private void onKeyDown(Event e) {
    if (e.character != ' ')
      return;
    int idx = marker ? table.getSelectionIndex() : cursor;
    if (idx >= 0) {
      click(idx, 0);
      e.doit = false;
    }
  }

  /**
   * Active rows are drawn via item colors only. The native selection (which would
   * look like an additional active row) is dropped again and only remembered as
   * keyboard cursor. Modifier clicks do not always report a Selection event, so
   * mouse down/up drop it as well, after the native handling has finished.
   */
  private void onNativeSelection(Event e) {
    int idx = e.item instanceof TableItem ? table.indexOf((TableItem) e.item) : table.getSelectionIndex();
    if (idx >= 0)
      cursor = idx;
    table.deselectAll();
  }

  private void dropNativeSelectionLater() {
    table.deselectAll();
    table.getDisplay().asyncExec(() -> {
      if (!table.isDisposed())
        table.deselectAll();
    });
  }

  private void click(int idx, int stateMask) {
    int count = table.getItemCount();
    if (idx < 0 || idx >= count)
      return;
    cursor = idx;
    if ((stateMask & SWT.SHIFT) != 0) {
      int from = validAnchor(idx);
      if (baseline == null || baseline.length != count)
        baseline = snapshot();
      int lo = Math.min(from, idx);
      int hi = Math.max(from, idx);
      for (int i = 0; i < count; i++) {
        boolean invert = i >= lo && i <= hi && i != from;
        set(i, invert != baseline[i]);
      }
      // anchor and baseline stay so the range can be adjusted by further shift clicks
    } else if ((stateMask & SWT.MOD1) != 0) {
      for (int i = 0; i < count; i++) set(i, i == idx);
      anchor = idx;
      baseline = snapshot();
    } else {
      set(idx, !isOn(table.getItem(idx)));
      anchor = idx;
      baseline = snapshot();
    }
    onChange.run();
  }

  private boolean[] snapshot() {
    boolean[] state = new boolean[table.getItemCount()];
    for (int i = 0; i < state.length; i++) state[i] = isOn(table.getItem(i));
    return state;
  }

  /**
   * Applies the given active rows; invalidates the shift baseline only if the state actually changes.
   */
  private void apply(java.util.Set<Integer> on) {
    boolean changed = false;
    for (int i = 0; i < table.getItemCount(); i++) {
      boolean want = on.contains(i);
      if (isOn(table.getItem(i)) != want) {
        changed = true;
        set(i, want);
      }
    }
    if (changed)
      baseline = null;
  }

  /**
   * The anchor if valid, otherwise the first active row, otherwise the clicked row.
   */
  private int validAnchor(int clicked) {
    if (anchor >= 0 && anchor < table.getItemCount())
      return anchor;
    for (int i = 0; i < table.getItemCount(); i++) if (isOn(table.getItem(i)))
      return i;
    return clicked;
  }

  // ------------------------------------------------------------ rendering
  private static boolean isOn(TableItem item) {
    return Boolean.TRUE.equals(item.getData());
  }

  private void set(int idx, boolean on) {
    TableItem item = table.getItem(idx);
    item.setData(on);
    render(item);
  }

  private void render(TableItem item) {
    boolean on = isOn(item);
    if (marker)
      item.setText(0, on ? "✓" : "");
    else {
      var d = table.getDisplay();
      item.setBackground(on ? d.getSystemColor(SWT.COLOR_LIST_SELECTION) : null);
      item.setForeground(on ? d.getSystemColor(SWT.COLOR_LIST_SELECTION_TEXT) : null);
    }
  }

  private int textIndex() {
    return marker ? 1 : 0;
  }

  private void layoutColumns() {
    int w = table.getClientArea().width;
    if (marker) {
      markerCol.setWidth(MARKER_WIDTH);
      textCol.setWidth(Math.max(0, w - MARKER_WIDTH));
    } else
      textCol.setWidth(w);
  }

  public Table getControl() {
    return table;
  }

  public void setLayoutData(Object layoutData) {
    table.setLayoutData(layoutData);
  }

  public void setFocus() {
    table.setFocus();
  }

  public void setItems(String[] items) {
    table.setRedraw(false);
    try {
      table.removeAll();
      anchor = -1;
      cursor = -1;
      baseline = null;
      if (items != null)
        for (String text : items) {
          TableItem item = new TableItem(table, SWT.NONE);
          item.setText(textIndex(), text);
          item.setData(Boolean.FALSE);
          render(item);
        }
      layoutColumns();
    } finally {
      table.setRedraw(true);
    }
  }

  /**
   * Updates the texts in place if the row count is unchanged; keeps activation, anchor and scroll position.
   */
  public boolean updateTexts(String[] items) {
    if (items == null || items.length != table.getItemCount())
      return false;
    for (int i = 0; i < items.length; i++) table.getItem(i).setText(textIndex(), items[i]);
    return true;
  }

  public void resetAnchor() {
    anchor = -1;
    baseline = null;
  }

  public String[] getItems() {
    String[] result = new String[table.getItemCount()];
    for (int i = 0; i < result.length; i++) result[i] = table.getItem(i).getText(textIndex());
    return result;
  }

  /**
   * Texts of the active rows.
   */
  public String[] getSelection() {
    return java.util.Arrays.stream(getSelectionIndices()).mapToObj(i -> table.getItem(i).getText(textIndex())).toArray(String[]::new);
  }

  /**
   * Indices of the active rows.
   */
  public int[] getSelectionIndices() {
    return java.util.stream.IntStream.range(0, table.getItemCount()).filter(i -> isOn(table.getItem(i))).toArray();
  }

  public void setSelectionIndices(int[] indices) {
    java.util.Set<Integer> on = new java.util.HashSet<>();
    if (indices != null)
      for (int i : indices) on.add(i);
    apply(on);
  }

  public void setSelection(String[] items) {
    java.util.List<String> all = java.util.Arrays.asList(getItems());
    java.util.Set<Integer> on = new java.util.HashSet<>();
    if (items != null)
      for (String item : items) {
        int idx = all.indexOf(item);
        if (idx >= 0)
          on.add(idx);
      }
    apply(on);
  }

  @Override
  public void clear() {
    anchor = -1;
    baseline = null;
    for (int i = 0; i < table.getItemCount(); i++) set(i, false);
  }
}
