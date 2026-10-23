use super::*;

#[test]
fn ids_compare_by_content() {
    assert_eq!(AxisId::new("operands"), AxisId::from("operands"));
    assert!(AxisId::new("a") < AxisId::new("b"));
    assert_eq!(ValueId::numeric(64).as_str(), "64");
}

#[test]
fn display_is_plain() {
    assert_eq!(ValueId::new("fp16").to_string(), "fp16");
}
