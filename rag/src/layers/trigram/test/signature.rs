use super::*;
fn tg(items: &[&str]) -> Vec<String> {
    let mut v: Vec<String> = items.iter().map(|s| s.to_string()).collect();
    v.sort();
    v
}
#[test]
fn boundary_marked_windows() {
    assert_eq!(query_trigrams("go"), tg(& ["_go", "go_"]));
    assert_eq!(query_trigrams("a"), tg(& ["_a_"]));
    assert_eq!(query_trigrams("user"), tg(& ["_us", "use", "ser", "er_"]));
}
#[test]
fn deduplicated_across_tokens() {
    assert_eq!(query_trigrams("go go"), tg(& ["_go", "go_"]));
}
#[test]
fn start_and_end_are_distinct() {
    let t = query_trigrams("user");
    assert!(t.contains(& "_us".to_string()));
    assert!(! t.contains(& "us_".to_string()));
}
#[test]
fn frequent_trigrams_removed_only_above_floor() {
    let text = "a a a a a a a a a a a a user name thing other word more";
    let kept = file_trigrams(text, 0.10, 10);
    assert!(! kept.contains(& "_a_".to_string()));
    assert!(kept.contains(& "_us".to_string()));
}
#[test]
fn no_pruning_below_floor() {
    let kept = file_trigrams("user name thing", 0.10, 10);
    assert!(kept.contains(& "_us".to_string()));
}
