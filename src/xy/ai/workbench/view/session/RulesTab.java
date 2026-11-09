package xy.ai.workbench.view.session;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.Optional;
import java.util.TreeMap;
import org.eclipse.swt.SWT;
import org.eclipse.swt.custom.SashForm;
import org.eclipse.swt.custom.ScrolledComposite;
import org.eclipse.swt.events.FocusListener;
import org.eclipse.swt.events.MouseListener;
import org.eclipse.swt.events.SelectionListener;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Button;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Control;
import org.eclipse.swt.widgets.Label;
import org.eclipse.swt.widgets.List;
import org.eclipse.swt.widgets.Spinner;
import org.eclipse.swt.widgets.TabFolder;
import org.eclipse.swt.widgets.TabItem;
import org.eclipse.swt.widgets.Text;
import xy.ai.conscious.autoapprove.RuleRegistry;
import xy.ai.conscious.autoapprove.rules.IApprovalRule;
import xy.ai.conscious.autoapprove.rules.RuleSettings;
import xy.ai.conscious.autoapprove.rules.RuleSettings.BooleanSetting;
import xy.ai.conscious.autoapprove.rules.RuleSettings.NumberSetting;
import xy.ai.conscious.autoapprove.rules.RuleSettings.Setting;
import xy.ai.conscious.autoapprove.rules.RuleSettings.StringListSetting;
import xy.ai.conscious.core.ConfigManager;

/**
 * Lists the approval rules and renders the settings of the selected rule
 * dynamically by setting type. Only deviations from the rule defaults are
 * written to the {@link ConfigManager}.
 */
public class RulesTab implements RuleSelection {
  private final UpdateGuard guard = new UpdateGuard();
  private final java.util.List<IApprovalRule> rules = new ArrayList<>();
  private ConfigManager cfg;
  private RuleRegistry registry;
  private List ruleList;
  private ScrolledComposite scroller;
  private Composite editor;

  public void create(TabFolder instr, ConfigManager cfg, RuleRegistry registry) {
    this.cfg = cfg;
    this.registry = registry;
    TabItem item = new TabItem(instr, SWT.NONE);
    item.setText("Rules");
    SashForm sash = new SashForm(instr, SWT.VERTICAL);
    item.setControl(sash);
    ruleList = new List(sash, SWT.SINGLE | SWT.V_SCROLL | SWT.BORDER);
    ruleList.addMouseListener(MouseListener.mouseDownAdapter(m -> ruleList.setFocus()));
    ruleList.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> showSelected()));
    scroller = new ScrolledComposite(sash, SWT.V_SCROLL | SWT.H_SCROLL | SWT.BORDER);
    scroller.setExpandHorizontal(true);
    scroller.setExpandVertical(true);
    editor = new Composite(scroller, SWT.NONE);
    editor.setLayout(new GridLayout(2, false));
    scroller.setContent(editor);
    sash.setWeights(1, 2);
    rules.addAll(registry.rules());
    rules.sort(Comparator.comparing(IApprovalRule::id, String.CASE_INSENSITIVE_ORDER));
    ruleList.setItems(rules.stream().map(IApprovalRule::id).toArray(String[]::new));
    if (!rules.isEmpty()) {
      ruleList.select(0);
      showSelected();
    }
    cfg.addRuleSettingsObs(overrides -> {
      if (!guard.isUpdating())
        showSelected();
    }, false);
  }

  @Override
  public Map<String, Map<String, Object>> getSelection() {
    Map<String, Map<String, Object>> out = new TreeMap<>();
    for (Map.Entry<String, RuleSettings> e : cfg.getRuleSettings().entrySet()) {
      Optional<IApprovalRule> rule = registry.find(e.getKey());
      if (rule.isEmpty())
        continue;
      RuleSettings diff = e.getValue().diff(rule.get().getDefaultSetting());
      if (!diff.isEmpty())
        out.put(e.getKey(), diff.toValues());
    }
    return out;
  }

  @Override
  public void setSelection(Map<String, Map<String, Object>> selection) {
    Map<String, RuleSettings> overrides = new LinkedHashMap<>();
    for (Map.Entry<String, Map<String, Object>> e : selection.entrySet()) {
      Optional<IApprovalRule> rule = registry.find(e.getKey());
      if (rule.isEmpty())
        continue;
      RuleSettings defaults = rule.get().getDefaultSetting();
      RuleSettings diff = defaults.overlay(RuleSettings.fromValues(e.getValue())).diff(defaults);
      if (!diff.isEmpty())
        overrides.put(e.getKey(), diff);
    }
    cfg.setRuleSettings(overrides);
  }

  @Override
  public void clear() {
    cfg.setRuleSettings(Map.of());
  }

  private void showSelected() {
    guard.run(() -> {
      for (Control child : editor.getChildren()) child.dispose();
    });
    int idx = ruleList.getSelectionIndex();
    if (idx >= 0 && idx < rules.size()) {
      IApprovalRule rule = rules.get(idx);
      RuleSettings working = rule.getDefaultSetting().overlay(cfg.getRuleSettings().get(rule.id()));
      for (Setting setting : working.settings().values()) createRow(rule, working, setting);
    }
    editor.layout(true, true);
    scroller.setMinSize(editor.computeSize(SWT.DEFAULT, SWT.DEFAULT));
  }

  private void createRow(IApprovalRule rule, RuleSettings working, Setting setting) {
    if (setting instanceof BooleanSetting flag) {
      new Label(editor, SWT.NONE).setText(flag.getName());
      Button toggle = new Button(editor, SWT.TOGGLE);
      toggle.setText(flag.get() ? "On" : "Off");
      toggle.setSelection(flag.get());
      GridData toggleData = new GridData(SWT.LEFT, SWT.CENTER, false, false);
      toggleData.widthHint = 60;
      toggle.setLayoutData(toggleData);
      toggle.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> {
        flag.set(toggle.getSelection());
        toggle.setText(flag.get() ? "On" : "Off");
        commit(rule, working);
      }));
    } else if (setting instanceof NumberSetting number) {
      new Label(editor, SWT.NONE).setText(number.getName());
      Spinner spinner = new Spinner(editor, SWT.BORDER);
      spinner.setMinimum(0);
      spinner.setMaximum(Integer.MAX_VALUE);
      spinner.setSelection((int) Math.max(0, Math.min(number.get(), Integer.MAX_VALUE)));
      spinner.setLayoutData(new GridData(SWT.FILL, SWT.CENTER, true, false));
      spinner.addModifyListener(e -> {
        number.set(spinner.getSelection());
        commit(rule, working);
      });
    } else if (setting instanceof StringListSetting list) {
      Label label = new Label(editor, SWT.NONE);
      label.setText(list.getName());
      label.setLayoutData(new GridData(SWT.LEFT, SWT.TOP, false, false));
      Text text = new Text(editor, SWT.MULTI | SWT.BORDER | SWT.V_SCROLL);
      text.setText(String.join(text.getLineDelimiter(), list.get()));
      GridData data = new GridData(SWT.FILL, SWT.FILL, true, false);
      data.widthHint = 1;
      data.heightHint = 60;
      text.setLayoutData(data);
      text.addMouseListener(MouseListener.mouseDownAdapter(m -> text.setFocus()));
      text.addFocusListener(FocusListener.focusLostAdapter(e -> {
        list.set(text.getText().lines().map(String::strip).filter(s -> !s.isEmpty()).toList());
        commit(rule, working);
      }));
    }
  }

  private void commit(IApprovalRule rule, RuleSettings working) {
    if (guard.isUpdating())
      return;
    Map<String, RuleSettings> all = cfg.getRuleSettings();
    RuleSettings diff = working.diff(rule.getDefaultSetting());
    if (diff.isEmpty())
      all.remove(rule.id());
    else
      all.put(rule.id(), diff);
    guard.run(() -> cfg.setRuleSettings(all));
  }
}
