use super::*;

fn set(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|s| s.to_string()).collect()
}

#[test]
fn empty_graph_renders_nothing() {
    assert_eq!(render_edges(&BTreeMap::new()), "");
}

#[test]
fn one_line_per_edge_in_sorted_order() {
    let mut edges = BTreeMap::new();
    edges.insert("B".to_string(), set(&["std::fmt::Display", "A"]));
    edges.insert("A".to_string(), set(&["C"]));
    assert_eq!(
        render_edges(&edges),
        "A\tC\nB\tA\nB\tstd::fmt::Display\n"
    );
}

#[test]
fn object_without_dependencies_is_still_listed() {
    let mut edges = BTreeMap::new();
    edges.insert("A".to_string(), BTreeSet::new());
    assert_eq!(render_edges(&edges), "A\t\n");
}
