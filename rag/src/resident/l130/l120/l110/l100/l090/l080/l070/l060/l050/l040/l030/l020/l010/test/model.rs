use super::*;

fn n(s: &str) -> Name {
    Name::new(s).unwrap()
}

fn spec(e: &str, s: &str) -> ModelSpec {
    ModelSpec::new(n("m"), StageSpec::new(n(e)), StageSpec::new(n(s))).unwrap()
}

#[test]
fn empty_name_rejected() {
    assert_eq!(Name::new(""), Err(ModelError::EmptyName));
    assert!(Name::new(" ").is_ok());
}

#[test]
fn spec_requires_stage_names() {
    let r = ModelSpec::new(n("m"), StageSpec::default(), StageSpec::new(n("s")));
    assert_eq!(r, Err(ModelError::EmptyName));
    let r = ModelSpec::new(n("m"), StageSpec::new(n("e")), StageSpec::default());
    assert_eq!(r, Err(ModelError::EmptyName));
}

#[test]
fn stage_params() {
    let st = StageSpec::new(n("e")).with_param("k", Param::Int(3));
    assert_eq!(st.params.get("k"), Some(&Param::Int(3)));
}

#[test]
fn duplicate_register() {
    let mut r = Registry::new();
    r.register(n("a"), Arc::new(1)).unwrap();
    assert_eq!(
        r.register(n("a"), Arc::new(2)),
        Err(ModelError::DuplicateName("a".into()))
    );
    assert_eq!(**r.get(&n("a")).unwrap(), 1);
    assert_eq!(r.len(), 1);
}

#[test]
fn names_sorted() {
    let mut r = Registry::<u8>::default();
    assert!(r.is_empty());
    r.register(n("b"), Arc::new(0)).unwrap();
    r.register(n("a"), Arc::new(0)).unwrap();
    let v: Vec<_> = r.names().map(|x| x.as_str()).collect();
    assert_eq!(v, vec!["a", "b"]);
    assert!(r.contains(&n("a")));
}

#[test]
fn resolve_ok_shares_arcs() {
    let mut e = Registry::new();
    let mut s = Registry::new();
    let ea = Arc::new(1u32);
    let sa = Arc::new("x");
    e.register(n("e"), ea.clone()).unwrap();
    s.register(n("s"), sa.clone()).unwrap();
    let r = spec("e", "s").resolve(&e, &s).unwrap();
    assert!(Arc::ptr_eq(&r.encoder, &ea));
    assert!(Arc::ptr_eq(&r.scorer, &sa));
    let c = r.clone();
    assert!(Arc::ptr_eq(&c.encoder, &ea));
    assert_eq!(c.spec, r.spec);
}

#[test]
fn resolve_errors() {
    let mut e = Registry::new();
    let mut s = Registry::new();
    let sp = spec("e", "s");
    assert!(matches!(
        sp.resolve::<u8, u8>(&Registry::new(), &Registry::new()),
        Err(ModelError::UnknownEncoder(x)) if x == "e"
    ));
    e.register(n("e"), Arc::new(0u8)).unwrap();
    assert!(matches!(
        sp.resolve(&e, &s),
        Err(ModelError::UnknownScorer(x)) if x == "s"
    ));
    s.register(n("s"), Arc::new(0u8)).unwrap();
    assert!(sp.resolve(&e, &s).is_ok());
}

fn assert_send_sync<T: Send + Sync + 'static>() {}

#[test]
fn bounds() {
    assert_send_sync::<Name>();
    assert_send_sync::<Param>();
    assert_send_sync::<StageSpec>();
    assert_send_sync::<ModelSpec>();
    assert_send_sync::<Registry<u8>>();
    assert_send_sync::<ResolvedModel<u8, u8>>();
}
