use super::*;

fn reason(d: &ModelDescriptor) -> String {
    match d.validate() {
        Err(ModelConfigError::Incompatible { reason }) => reason,
        Ok(()) => String::new(),
    }
}

#[test]
fn default_values() {
    let d = ModelDescriptor::e5_small_default(Path::new("/nonexistent"), DType::F16);
    assert_eq!(d.model_id, E5_SMALL_MODEL_ID);
    assert_eq!(d.architecture, "bert");
    assert_eq!(d.dimension, 384);
    assert_eq!(d.pooling, Pooling::Mean);
    assert_eq!(d.normalization, Normalization::L2);
    assert_eq!(d.query_prefix, "query: ");
    assert_eq!(d.passage_prefix, "passage: ");
    assert_eq!(d.max_seq_len, 512);
    assert_eq!(d.storage_dtype, DType::F16);
    assert_eq!(d.compute_dtype(DeviceClass::Cpu), DType::F32);
    assert_eq!(d.compute_dtype(DeviceClass::Gpu), DType::F32);
    assert!(d.projection_layers.is_empty());
    assert!(d.multi_vector.is_none());
    assert_eq!(d.config_path, Path::new("/nonexistent/config.json"));
    assert_eq!(
        d.weight_paths,
        vec![PathBuf::from("/nonexistent/model.safetensors")]
    );
    assert!(d.validate().is_ok());
}

#[test]
fn repo_default_matches_dir_default() {
    assert_eq!(
        ModelDescriptor::e5_small_repo_default(DType::F32),
        ModelDescriptor::e5_small_default(Path::new(E5_SMALL_REPO_DIR), DType::F32)
    );
}

#[test]
fn compute_dtypes_are_independent() {
    let mut d = ModelDescriptor::e5_small_repo_default(DType::U8);
    d.compute_dtypes = ComputeDTypes {
        cpu: DType::F32,
        gpu: DType::F16,
    };
    assert_eq!(d.compute_dtype(DeviceClass::Cpu), DType::F32);
    assert_eq!(d.compute_dtype(DeviceClass::Gpu), DType::F16);
    assert_eq!(d.storage_dtype, DType::U8);
}

#[test]
fn validate_dimension_first() {
    let mut d = ModelDescriptor::e5_small_repo_default(DType::F32);
    d.dimension = 0;
    d.max_seq_len = 0;
    d.query_prefix = "query:".into();
    assert!(reason(&d).contains("dimension"));
}

#[test]
fn validate_max_seq_len() {
    let mut d = ModelDescriptor::e5_small_repo_default(DType::F32);
    d.max_seq_len = 0;
    d.query_prefix = "query:".into();
    assert!(reason(&d).contains("max_seq_len"));
}

#[test]
fn validate_prefixes() {
    let mut d = ModelDescriptor::e5_small_repo_default(DType::F32);
    d.query_prefix = "query:".into();
    d.passage_prefix = "passage:".into();
    let r = reason(&d);
    assert!(r.contains("query_prefix"));
    assert!(r.contains("\"query:\""));
    d.query_prefix = String::new();
    assert!(reason(&d).contains("passage_prefix"));
    d.passage_prefix = " ".into();
    assert!(d.validate().is_ok());
}

#[test]
fn display_format() {
    let e = ModelConfigError::Incompatible {
        reason: "x".into(),
    };
    assert_eq!(e.to_string(), "inkompatible Modellkonfiguration: x");
}

#[test]
fn multi_vector_not_validated() {
    let mut d = ModelDescriptor::e5_small_repo_default(DType::F32);
    d.multi_vector = Some(MultiVectorConfig {
        max_query_len: 0,
        max_document_len: 0,
        query_marker: String::new(),
        document_marker: String::new(),
        dimension: 0,
        skiplist: vec![],
    });
    assert!(d.validate().is_ok());
}

#[test]
fn types_are_send_sync() {
    fn check<T: Send + Sync + 'static>() {}
    check::<ModelDescriptor>();
    check::<ModelConfigError>();
}
