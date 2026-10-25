use std::time::Duration;
use proptest::prelude::*;
use super::*;
use crate::resident::facade::{
    EmbeddingParams, InferenceParams, ScorerKind, SimilarityParams, TaskKind, TextRole,
};
use crate::resident::op::{EngineSelector, IndexId, ModelId, Op, OpKind, ResourceKind};
use crate::resident::processor::StarvedSet;
const TOML: &str = r#"
    default_model = "e5-small"

    [models.e5-small]
    pooling = "mean"
    normalize = true
    max_seq_len = 512
    dtype = "f16"

    [models.e5-small.prefixes]
    query = "query: "
    passage = "passage: "
"#;
const KINDS: [ResourceKind; 6] = [
    ResourceKind::Tokenizer,
    ResourceKind::Normalizer,
    ResourceKind::Encoder,
    ResourceKind::Scorer,
    ResourceKind::Index,
    ResourceKind::CrossEncoder,
];
fn planner() -> Planner {
    Planner::new(RuleSet::from_toml(TOML).unwrap())
}
fn hybrid() -> EngineOffers {
    EngineOffers::new()
        .with_cpu(KINDS)
        .with_gpu([
            ResourceKind::Encoder,
            ResourceKind::Scorer,
            ResourceKind::CrossEncoder,
        ])
}
fn gpu_only_models() -> EngineOffers {
    EngineOffers::new()
        .with_cpu([ResourceKind::Tokenizer, ResourceKind::Index])
        .with_gpu([
            ResourceKind::Encoder,
            ResourceKind::Scorer,
            ResourceKind::CrossEncoder,
        ])
}
fn kinds(ops: &[PlannedOp]) -> Vec<OpKind> {
    ops.iter().map(|o| o.desc.kind()).collect()
}
fn flags(ops: &[PlannedOp]) -> Vec<bool> {
    ops.iter().map(|o| o.materialize).collect()
}
fn none() -> StarvedSnapshot {
    StarvedSnapshot::default()
}
#[test]
fn embedding_chain_keeps_gpu_results_on_device() {
    let params = EmbeddingParams::new(TextRole::Query)
        .write_index(IndexId("idx".into()));
    let ops = planner()
        .expand(&TaskSpec::Embedding(&params), none(), &gpu_only_models())
        .unwrap();
    assert_eq!(
        kinds(& ops), [OpKind::Tokenize, OpKind::Embed, OpKind::Pool, OpKind::L2Norm,
        OpKind::WriteIndex]
    );
    assert_eq!(flags(& ops), [true, false, false, true, true]);
}
#[test]
fn embedding_without_index_ends_on_host() {
    let params = EmbeddingParams::new(TextRole::Passage);
    let ops = planner()
        .expand(&TaskSpec::Embedding(&params), none(), &gpu_only_models())
        .unwrap();
    assert_eq!(
        kinds(& ops), [OpKind::Tokenize, OpKind::Embed, OpKind::Pool, OpKind::L2Norm]
    );
    assert_eq!(flags(& ops), [true, false, false, true]);
}
#[test]
fn cpu_only_offers_select_cpu() {
    let offers = EngineOffers::new().with_cpu(KINDS);
    let params = EmbeddingParams::default();
    let ops = planner().expand(&TaskSpec::Embedding(&params), none(), &offers).unwrap();
    assert!(
        ops.iter().all(| o | o.desc.params().requires.engine == EngineSelector::Cpu)
    );
    assert!(ops.iter().all(| o | o.materialize));
}
#[test]
fn missing_capability_is_reported() {
    let offers = EngineOffers::new().with_cpu([ResourceKind::Tokenizer]);
    let params = EmbeddingParams::default();
    let err = planner()
        .expand(&TaskSpec::Embedding(&params), none(), &offers)
        .unwrap_err();
    assert!(matches!(err, PlannerError::MissingCapability { .. }));
}
#[test]
fn similarity_sets_top_k_and_index() {
    let params = SimilarityParams::new(IndexId("idx".into())).top_k(7);
    let ops = planner()
        .expand(&TaskSpec::Similarity(&params), none(), &gpu_only_models())
        .unwrap();
    assert_eq!(kinds(& ops), [OpKind::Similarity, OpKind::TopK]);
    assert_eq!(ops[1].desc.params().top_k, Some(7));
    assert_eq!(ops[0].desc.params().requires.index, Some(IndexId("idx".into())));
    assert_eq!(flags(& ops), [false, true]);
}
#[test]
fn similarity_rejects_zero_top_k() {
    let params = SimilarityParams::new(IndexId("idx".into())).top_k(0);
    let err = planner()
        .expand(&TaskSpec::Similarity(&params), none(), &hybrid())
        .unwrap_err();
    assert!(matches!(err, PlannerError::InvalidConfig { .. }));
}
#[test]
fn similarity_rejects_cross_encoder_instead_of_falling_back() {
    let params = SimilarityParams::new(IndexId("idx".into()))
        .scorer(ScorerKind::CrossEncoder(ModelId("reranker".into())));
    let err = planner()
        .expand(&TaskSpec::Similarity(&params), none(), &hybrid())
        .unwrap_err();
    assert!(matches!(err, PlannerError::ScorerUnavailable { .. }));
}
#[test]
fn similarity_defaults_to_cosine() {
    let params = SimilarityParams::new(IndexId("idx".into()));
    assert_eq!(params.get_scorer(), & ScorerKind::Cosine);
}
#[test]
fn inference_uses_requested_model() {
    let params = InferenceParams::new().model(ModelId("e5-small".into()));
    let ops = planner()
        .expand(&TaskSpec::Inference(&params), none(), &hybrid())
        .unwrap();
    assert_eq!(kinds(& ops), [OpKind::Tokenize, OpKind::Infer]);
    assert_eq!(ops[1].desc.params().requires.model, Some(ModelId("e5-small".into())));
}
#[test]
fn unknown_model_is_rejected() {
    let params = InferenceParams::new().model(ModelId("nope".into()));
    let err = planner()
        .expand(&TaskSpec::Inference(&params), none(), &hybrid())
        .unwrap_err();
    assert!(matches!(err, PlannerError::UnknownModel { .. }));
}
#[test]
fn pipeline_flags_shape_the_chain() {
    let src = TOML
        .replace("pooling = \"mean\"", "pooling = \"none\"")
        .replace("normalize = true", "normalize = false");
    let planner = Planner::new(RuleSet::from_toml(&src).unwrap());
    let params = EmbeddingParams::default();
    let ops = planner.expand(&TaskSpec::Embedding(&params), none(), &hybrid()).unwrap();
    assert_eq!(kinds(& ops), [OpKind::Tokenize, OpKind::Embed]);
}
#[test]
fn config_validation() {
    assert!(RuleSet::from_toml(TOML).is_ok());
    let unknown_field = TOML.replace("dtype", "dtpye");
    assert!(
        matches!(RuleSet::from_toml(& unknown_field), Err(PlannerError::InvalidConfig {
        .. }))
    );
    let zero_len = TOML.replace("512", "0");
    assert!(
        matches!(RuleSet::from_toml(& zero_len), Err(PlannerError::InvalidConfig { .. }))
    );
    let bad_default = TOML
        .replace("default_model = \"e5-small\"", "default_model = \"x\"");
    assert!(
        matches!(RuleSet::from_toml(& bad_default), Err(PlannerError::UnknownModel { ..
        }))
    );
}
#[test]
fn register_requires_configured_model() {
    let mut rules = RuleSet::from_toml(TOML).unwrap();
    let err = rules
        .register(ModelId("nope".into()), TaskKind::Embedding, Box::new(Fixed))
        .unwrap_err();
    assert!(matches!(err, PlannerError::UnknownModel { .. }));
    rules
        .register(ModelId("e5-small".into()), TaskKind::Embedding, Box::new(Fixed))
        .unwrap();
}
struct Fixed;
impl PlanExpander for Fixed {
    fn expand(
        &self,
        _task: &TaskSpec<'_>,
        _ctx: &ExpandContext<'_>,
    ) -> Result<Vec<PlannedOp>, PlannerError> {
        Ok(Vec::new())
    }
}
#[test]
fn reload_swaps_snapshot_atomically() {
    let planner = planner();
    let before = planner.snapshot();
    let src = TOML.replace("e5-small", "other");
    planner.reload(RuleSet::from_toml(&src).unwrap());
    assert_eq!(before.default_model(), & ModelId("e5-small".into()));
    assert_eq!(planner.snapshot().default_model(), & ModelId("other".into()));
}
proptest! {
    #[test] fn starved_hint_changes_engines_not_ops(mask in prop::array::uniform6(any::<
    bool > ())) { let set = StarvedSet::new(Duration::from_secs(60)); for (kind, on) in
    KINDS.iter().zip(mask) { if on { set.mark(* kind); } } let params =
    EmbeddingParams::new(TextRole::Query).write_index(IndexId("idx".into())); let planner
    = planner(); let offers = hybrid(); let plain = planner.expand(&
    TaskSpec::Embedding(& params), none(), & offers).unwrap(); let hinted = planner
    .expand(& TaskSpec::Embedding(& params), set.snapshot(), & offers).unwrap();
    prop_assert_eq!(kinds(& plain), kinds(& hinted)); let again = planner.expand(&
    TaskSpec::Embedding(& params), set.snapshot(), & offers).unwrap();
    prop_assert_eq!(hinted, again); }
}
#[test]
fn first_resource_matches_expansion() {
    assert_eq!(first_resource(TaskKind::Similarity), ResourceKind::Scorer);
    assert_eq!(first_resource(TaskKind::Embedding), ResourceKind::Tokenizer);
    assert_eq!(first_resource(TaskKind::Inference), ResourceKind::Tokenizer);
}
