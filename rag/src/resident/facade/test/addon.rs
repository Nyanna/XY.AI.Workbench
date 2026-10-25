//! Reference cascade built only on the public submit API and parameter extensions.
use super::*;
use crate::resident::index::{EntryId, EntrySet, Scope};
use crate::resident::op::{IndexId, ModelId};
use crate::resident::processor::WorkerConfig;
#[derive(Clone, Copy, Debug, PartialEq)]
struct Correlation(u64);
fn resident() -> Resident {
    Resident::builder()
        .model(ModelId("e5-small".into()))
        .worker_hint(WorkerConfig::new(0, 0))
        .init()
        .unwrap()
}
fn claim_and_complete(r: &Resident, out: Output) -> (TaskMeta, Payload) {
    for node in r.queue().iter_live() {
        let Some(meta) = node.params().copied() else { continue };
        if let Some(payload) = node.claim() {
            node.task_state().unwrap().complete(Ok(out));
            return (meta, payload);
        }
    }
    panic!("no queued task");
}
/// Continuation step: derives follow-up tasks from a finished one through the normal queue.
fn cascade(
    r: &Resident,
    hits: &Output,
    summary: &SimilarityParams,
    members: IndexId,
) -> Vec<TaskHandle> {
    let Output::Hits(hits) = hits else { return Vec::new() };
    let cells = EntrySet::from_ids(hits.iter().map(|(id, _)| *id));
    let follow = SimilarityParams::new(members)
        .scope(Scope::Set(cells))
        .top_k(summary.get_top_k());
    let follow = match summary.ext().get::<Correlation>() {
        Some(c) => follow.with_ext(*c),
        None => follow,
    };
    vec![r.submit_similarity(VectorInput::new(vec![vec![1.0]]), follow)]
}
#[test]
fn foreign_extension_travels_through_the_queue() {
    let r = resident();
    let params = EmbeddingParams::default().with_ext(Correlation(11));
    let _handle = r.submit_embedding(TextInput::single("x"), params);
    let (_, payload) = claim_and_complete(&r, Output::Embeddings(vec![]));
    let Payload::Embedding { params, .. } = payload else { panic!("wrong payload") };
    assert_eq!(params.ext().get::< Correlation > (), Some(& Correlation(11)));
}
#[test]
fn cascade_follow_up_is_a_regular_task_with_derived_scope() {
    let r = resident();
    let summary = SimilarityParams::new(IndexId("summary".into()))
        .with_ext(Correlation(5));
    let first = r.submit_similarity(VectorInput::new(vec![vec![1.0]]), summary.clone());
    let out = Output::Hits(vec![(EntryId::new(3), 0.9), (EntryId::new(8), 0.5)]);
    let (first_meta, _) = claim_and_complete(&r, out.clone());
    drop(first);
    let followers = cascade(&r, &out, &summary, IndexId("members".into()));
    assert_eq!(followers.len(), 1);
    let (meta, payload) = claim_and_complete(&r, Output::Hits(vec![]));
    assert_eq!(meta.kind, first_meta.kind);
    assert_ne!(meta.id, first_meta.id);
    let Payload::Similarity { params, .. } = payload else { panic!("wrong payload") };
    assert_eq!(params.index(), & IndexId("members".into()));
    assert_eq!(params.ext().get::< Correlation > (), Some(& Correlation(5)));
    let Scope::Set(set) = params.get_scope() else { panic!("scope not derived") };
    assert!(set.contains(EntryId::new(3)) && set.contains(EntryId::new(8)));
    assert!(! set.contains(EntryId::new(4)));
}
#[test]
fn follow_up_equals_application_task_without_addon_marker() {
    let r = resident();
    let scope = Scope::Set(EntrySet::from_ids([EntryId::new(1)]));
    let by_app = SimilarityParams::new(IndexId("members".into())).scope(scope.clone());
    let _a = r.submit_similarity(VectorInput::new(vec![vec![1.0]]), by_app);
    let (app_meta, _) = claim_and_complete(&r, Output::Hits(vec![]));
    let summary = SimilarityParams::new(IndexId("summary".into()));
    let out = Output::Hits(vec![(EntryId::new(1), 1.0)]);
    let _f = cascade(&r, &out, &summary, IndexId("members".into()));
    let (addon_meta, payload) = claim_and_complete(&r, Output::Hits(vec![]));
    assert_eq!(addon_meta.kind, app_meta.kind);
    let Payload::Similarity { params, .. } = payload else { panic!("wrong payload") };
    assert_eq!(params.get_scope(), & scope);
    assert!(params.ext().is_empty());
}
