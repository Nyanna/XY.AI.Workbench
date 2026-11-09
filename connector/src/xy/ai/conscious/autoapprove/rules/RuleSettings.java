package xy.ai.conscious.autoapprove.rules;

import java.util.ArrayList;
import java.util.Collections;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Set;
import java.util.TreeMap;
import java.util.stream.Collectors;

/**
 * Typed, named settings of one rule. Every value lives in a {@link Setting}
 * container so an editor can render the settings generically by type.
 * <p>
 * A rule exposes its defaults (by clone) through
 * {@link IApprovalRule#getDefaultSetting()}. Per-prompt overrides are held as
 * {@link #diff(RuleSettings) diff} against the defaults and resolved with
 * {@link #overlay(RuleSettings)}.
 */
public class RuleSettings {
  public static final String ACTIVE = "active";
  private final Map<String, Setting> settings = new LinkedHashMap<>();

  /**
   * Creates settings containing only the {@value #ACTIVE} flag (enabled).
   */
  public RuleSettings() {
    add(new BooleanSetting(ACTIVE, true));
  }

  private RuleSettings(boolean empty) {
  }

  /**
   * Settings without any entry; used for sparse overrides.
   */
  public static RuleSettings empty() {
    return new RuleSettings(true);
  }

  public RuleSettings add(Setting setting) {
    settings.put(setting.getName(), setting);
    return this;
  }

  public RuleSettings addBoolean(String name, boolean value) {
    return add(new BooleanSetting(name, value));
  }

  public RuleSettings addNumber(String name, long value) {
    return add(new NumberSetting(name, value));
  }

  public RuleSettings addStrings(String name, Iterable<String> values) {
    return add(new StringListSetting(name, values));
  }

  public RuleSettings addStrings(String name, String... values) {
    return add(new StringListSetting(name, List.of(values)));
  }

  /**
   * Unmodifiable view, setting name to container.
   */
  public Map<String, Setting> settings() {
    return Collections.unmodifiableMap(settings);
  }

  public boolean isEmpty() {
    return settings.isEmpty();
  }

  /**
   * Absent flag counts as active.
   */
  public boolean isActive() {
    Setting s = settings.get(ACTIVE);
    return !(s instanceof BooleanSetting b) || b.get();
  }

  public long getNumber(String name) {
    return settings.get(name) instanceof NumberSetting n ? n.get() : 0;
  }

  public boolean getBoolean(String name) {
    return settings.get(name) instanceof BooleanSetting b && b.get();
  }

  public List<String> getStrings(String name) {
    return settings.get(name) instanceof StringListSetting l ? l.get() : List.of();
  }

  public Set<String> getStringSet(String name) {
    return Set.copyOf(getStrings(name));
  }

  /**
   * Deep copy.
   */
  @Override
  public RuleSettings clone() {
    RuleSettings copy = new RuleSettings(true);
    for (Setting s : settings.values()) copy.settings.put(s.getName(), s.copy());
    return copy;
  }

  /**
   * Copy of these settings where every entry with a same-named, same-typed entry
   * in {@code override} takes over its value. Entries only present in
   * {@code override} are ignored.
   */
  public RuleSettings overlay(RuleSettings override) {
    RuleSettings result = clone();
    if (override == null)
      return result;
    for (Setting o : override.settings.values()) {
      Setting own = result.settings.get(o.getName());
      if (own != null && own.getClass() == o.getClass())
        result.settings.put(own.getName(), o.copy());
    }
    return result;
  }

  /**
   * Only the entries that differ from {@code base} (or are missing there).
   */
  public RuleSettings diff(RuleSettings base) {
    RuleSettings result = new RuleSettings(true);
    for (Setting s : settings.values()) if (base == null || !s.equals(base.settings.get(s.getName())))
      result.settings.put(s.getName(), s.copy());
    return result;
  }

  /**
   * Plain values (Boolean, Long, List of String) by setting name.
   */
  public Map<String, Object> toValues() {
    Map<String, Object> out = new TreeMap<>();
    for (Setting s : settings.values()) out.put(s.getName(), s.getValue());
    return out;
  }

  /**
   * Inverse of {@link #toValues()}; the setting type is derived from the value
   * kind. Unsupported values are skipped.
   */
  public static RuleSettings fromValues(Map<String, ?> values) {
    RuleSettings result = new RuleSettings(true);
    if (values == null)
      return result;
    for (Map.Entry<String, ?> e : values.entrySet()) {
      Object v = e.getValue();
      if (v instanceof Boolean b)
        result.add(new BooleanSetting(e.getKey(), b));
      else if (v instanceof Number n)
        result.add(new NumberSetting(e.getKey(), n.longValue()));
      else if (v instanceof List<?> l)
        result.add(new StringListSetting(e.getKey(), l.stream().map(String::valueOf).collect(Collectors.toList())));
    }
    return result;
  }

  public static Map<String, Map<String, Object>> exportAll(Map<String, RuleSettings> byRule) {
    Map<String, Map<String, Object>> out = new TreeMap<>();
    if (byRule != null)
      byRule.forEach((id, rs) -> out.put(id, rs.toValues()));
    return out;
  }

  public static Map<String, RuleSettings> importAll(Map<String, ? extends Map<String, ?>> byRule) {
    Map<String, RuleSettings> out = new LinkedHashMap<>();
    if (byRule != null)
      byRule.forEach((id, values) -> out.put(id, fromValues(values)));
    return out;
  }

  /**
   * Deep copy of a settings map.
   */
  public static Map<String, RuleSettings> copyOf(Map<String, RuleSettings> byRule) {
    Map<String, RuleSettings> out = new LinkedHashMap<>();
    if (byRule != null)
      byRule.forEach((id, rs) -> out.put(id, rs.clone()));
    return out;
  }

  @Override
  public boolean equals(Object obj) {
    return obj instanceof RuleSettings other && settings.equals(other.settings);
  }

  @Override
  public int hashCode() {
    return settings.hashCode();
  }

  @Override
  public String toString() {
    return settings.values().toString();
  }

  /**
   * Named container of one typed value.
   */
  public static abstract class Setting {
    private final String name;

    protected Setting(String name) {
      this.name = Objects.requireNonNull(name);
    }

    public String getName() {
      return name;
    }

    public abstract Setting copy();

    /**
     * Plain value: Boolean, Long or List of String.
     */
    public abstract Object getValue();

    /**
     * Tolerant assignment of a plain value; incompatible values are ignored.
     */
    public abstract void setValue(Object value);

    @Override
    public boolean equals(Object obj) {
      return obj != null && obj.getClass() == getClass() && ((Setting) obj).name.equals(name) && ((Setting) obj).getValue().equals(getValue());
    }

    @Override
    public int hashCode() {
      return Objects.hash(name, getValue());
    }

    @Override
    public String toString() {
      return name + "=" + getValue();
    }
  }

  public static class BooleanSetting extends Setting {
    private boolean value;

    public BooleanSetting(String name, boolean value) {
      super(name);
      this.value = value;
    }

    public boolean get() {
      return value;
    }

    public void set(boolean value) {
      this.value = value;
    }

    @Override
    public Setting copy() {
      return new BooleanSetting(getName(), value);
    }

    @Override
    public Object getValue() {
      return value;
    }

    @Override
    public void setValue(Object v) {
      if (v instanceof Boolean b)
        value = b;
    }
  }

  public static class NumberSetting extends Setting {
    private long value;

    public NumberSetting(String name, long value) {
      super(name);
      this.value = value;
    }

    public long get() {
      return value;
    }

    public void set(long value) {
      this.value = value;
    }

    @Override
    public Setting copy() {
      return new NumberSetting(getName(), value);
    }

    @Override
    public Object getValue() {
      return value;
    }

    @Override
    public void setValue(Object v) {
      if (v instanceof Number n)
        value = n.longValue();
    }
  }

  public static class StringListSetting extends Setting {
    private List<String> values;

    public StringListSetting(String name, Iterable<String> values) {
      super(name);
      this.values = new ArrayList<>();
      values.forEach(this.values::add);
    }

    /**
     * Unmodifiable view.
     */
    public List<String> get() {
      return Collections.unmodifiableList(values);
    }

    public void set(List<String> values) {
      this.values = new ArrayList<>(values);
    }

    @Override
    public Setting copy() {
      return new StringListSetting(getName(), values);
    }

    @Override
    public Object getValue() {
      return List.copyOf(values);
    }

    @Override
    public void setValue(Object v) {
      if (v instanceof List<?> l)
        values = l.stream().map(String::valueOf).collect(Collectors.toCollection(ArrayList::new));
    }
  }
}
