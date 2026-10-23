use super::*;

#[test]
fn position_must_be_in_unit_interval() {
    for p in [0.0, 0.5, 1.0] {
        assert!(Profile::new(p, ProfileConfig::new()).is_ok());
    }
    for p in [-0.1, 1.1, f32::NAN, f32::INFINITY] {
        assert!(matches!(
            Profile::new(p, ProfileConfig::new()),
            Err(PrecisionError::InvalidPrecision { .. })
        ));
    }
}

#[test]
fn config_holds_one_value_per_axis() {
    let cfg = ProfileConfig::new()
        .with("operands", "fp16")
        .with("operands", "fp32")
        .with("fusion", "on");
    assert_eq!(cfg.len(), 2);
    assert_eq!(cfg.get(&AxisId::new("operands")), Some(&ValueId::new("fp32")));
}
