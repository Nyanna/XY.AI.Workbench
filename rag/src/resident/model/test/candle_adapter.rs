use std::path::PathBuf;
use super::*;
use crate::resident::memory::layout::Layout;
use crate::resident::memory::{
    ArenaKind, ArenaSizes, CpuMemoryManager, DType, DeviceMemory,
};
use crate::resident::model::{
    plan_batches, BatchConfig, ModelRegistry, RegionIo, TensorRef,
};
use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};
fn model_dir() -> Option<PathBuf> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("model/multilingual-e5-small");
    dir.join("model.safetensors").exists().then_some(dir)
}
fn embed(texts: &[&str], dir: &std::path::Path, dtype: DType, device: DeviceKind) -> Vec<Vec<f32>> {
    embed_with(&ModelSpec::e5_small(dir, dtype), texts, device)
}

fn embed_with(spec: &ModelSpec, texts: &[&str], device: DeviceKind) -> Vec<Vec<f32>> {
    let dir = spec.paths.config.parent().unwrap();
    let dim = spec.dim;
    let loaded = ModelRegistry::with_defaults().load(spec, device).unwrap();
    let tok = tokenizers::Tokenizer::from_file(dir.join("tokenizer.json")).unwrap();
    let seqs: Vec<Vec<u32>> = texts
        .iter()
        .map(|t| tok.encode(*t, true).unwrap().get_ids().to_vec())
        .collect();
    let batches = plan_batches(
        &seqs,
        &BatchConfig {
            token_budget: 1 << 14,
            max_seq_len: 512,
            pad_token_id: 1,
        },
    );
    let sizes = ArenaSizes {
        weights: 1 << 10,
        index_segment: 1 << 10,
        workspace: 1 << 20,
        staging: 1 << 20,
        param_block: 1 << 12,
    };
    let mem = Arc::new(CpuMemoryManager::new(sizes, 1));
    let scratch = mem.alloc_in(ArenaKind::Workspace, 1024, 16).unwrap();
    let lane = RecordingLane::new(LaneClass::Query, LaneConfig::new(scratch));
    let io: Arc<dyn RegionIo> = mem.clone();
    let mut result = vec![Vec::new(); texts.len()];
    for b in &batches {
        let t_layout = Layout::contiguous(&[2, b.rows(), b.seq_len], DType::U32);
        let t_region = mem
            .alloc_in(ArenaKind::Staging, t_layout.byte_len() as u64, 16)
            .unwrap();
        mem.write(t_region, &b.pack()).unwrap();
        let o_layout = Layout::contiguous(&[b.rows(), dim], DType::F32);
        let o_region = mem
            .alloc_in(ArenaKind::Workspace, o_layout.byte_len() as u64, 16)
            .unwrap();
        let out = TensorRef {
            region: o_region,
            layout: o_layout,
        };
        loaded
            .encoder()
            .encode_into(
                &io,
                &TensorRef {
                    region: t_region,
                    layout: t_layout,
                },
                &out,
                &lane,
            )
            .unwrap()
            .wait()
            .unwrap();
        let mut raw = vec![0u8; out.layout.byte_len()];
        mem.read(o_region, &mut raw).unwrap();
        let flat: Vec<f32> = raw
            .chunks_exact(4)
            .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect();
        for (row, idx) in b.indices.iter().enumerate() {
            result[*idx] = flat[row * dim..(row + 1) * dim].to_vec();
        }
    }
    result
}
fn cosine(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
#[test]
fn e5_small_embeddings_are_normalised_and_padding_invariant() {
    let Some(dir) = model_dir() else { return };
    let texts = [
        "query: hello world",
        "passage: a much longer passage about retrieval of documents with embeddings",
    ];
    let batched = embed(&texts, &dir, DType::F32, DeviceKind::Cpu);
    let single = embed(&texts[..1], &dir, DType::F32, DeviceKind::Cpu);
    for v in &batched {
        assert!((cosine(v, v) - 1.0).abs() < 1e-4);
    }
    assert!(cosine(& batched[0], & single[0]) > 0.9999);
}
#[test]
fn e5_small_ranks_related_text_higher() {
    let Some(dir) = model_dir() else { return };
    let v = embed(
        &[
            "query: how do plants make energy",
            "passage: Photosynthesis converts light into chemical energy in plants.",
            "passage: The stock market closed lower on Tuesday.",
        ],
        &dir,
        DType::F32,
        DeviceKind::Cpu,
    );
    assert!(cosine(& v[0], & v[1]) > cosine(& v[0], & v[2]));
}
#[test]
fn wrong_hidden_size_is_incompatible() {
    let Some(dir) = model_dir() else { return };
    let mut spec = ModelSpec::e5_small(&dir, DType::F32);
    spec.dim = 128;
    assert!(
        matches!(CandleBert::load(& spec, DeviceKind::Cpu),
        Err(ModelError::Incompatible { .. }))
    );
}
fn spec_for(name: &str, dir: &std::path::Path) -> Option<ModelSpec> {
    match name {
        "multilingual-e5-small" => Some(ModelSpec::e5_small(dir, DType::F32)),
        _ => None,
    }
}

fn installed_models() -> Vec<(String, PathBuf)> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("model");
    let mut found: Vec<_> = std::fs::read_dir(root)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().join("config.json").exists())
        .map(|e| (e.file_name().to_string_lossy().into_owned(), e.path()))
        .collect();
    found.sort();
    found
}

fn reference(name: &str) -> Option<(Vec<String>, Vec<Vec<f32>>)> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/data/reference/{name}.json"));
    let json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()?;
    let texts = json["texts"].as_array()?.iter().map(|t| t.as_str().map(str::to_owned)).collect::<Option<_>>()?;
    let embeddings = json["embeddings"]
        .as_array()?
        .iter()
        .map(|row| row.as_array()?.iter().map(|x| x.as_f64().map(|x| x as f32)).collect::<Option<Vec<_>>>())
        .collect::<Option<_>>()?;
    Some((texts, embeddings))
}

fn assert_matches_reference(device: DeviceKind, min_cosine: f32) {
    let models = installed_models();
    assert!(!models.is_empty(), "no model installed under model/ (see model/README.md)");
    for (name, dir) in models {
        let (texts, expected) = reference(&name)
            .unwrap_or_else(|| panic!("model `{name}` has no tests/data/reference/{name}.json (see model/README.md)"));
        let spec = spec_for(&name, &dir).unwrap_or_else(|| panic!("no ModelSpec registered for `{name}` in spec_for"));
        let refs: Vec<&str> = texts.iter().map(String::as_str).collect();
        let got = embed_with(&spec, &refs, device);
        assert_eq!(got.len(), expected.len(), "{name}");
        for (i, (g, e)) in got.iter().zip(&expected).enumerate() {
            let c = cosine(g, e);
            assert!(c > min_cosine, "{name} text {i}: cosine {c}");
        }
    }
}

#[test]
fn installed_models_cpu_match_reference() {
    assert_matches_reference(DeviceKind::Cpu, 0.9999);
}

#[cfg(feature = "cuda")]
#[test]
#[ignore = "requires a CUDA device"]
fn installed_models_gpu_match_reference() {
    assert_matches_reference(DeviceKind::Gpu, 0.999);
}

#[cfg(feature = "cuda")]
#[test]
#[ignore = "requires a CUDA device"]
fn e5_small_gpu_matches_cpu() {
    let Some(dir) = model_dir() else { return };
    let texts = [
        "query: hello world",
        "passage: a much longer passage about retrieval of documents with embeddings",
    ];
    let cpu = embed(&texts, &dir, DType::F32, DeviceKind::Cpu);
    let gpu = embed(&texts, &dir, DType::F32, DeviceKind::Gpu);
    for (c, g) in cpu.iter().zip(&gpu) {
        assert!(g.iter().all(| x | x.is_finite()));
        assert!(cosine(c, g) > 0.999, "cosine {}", cosine(c, g));
    }
}
