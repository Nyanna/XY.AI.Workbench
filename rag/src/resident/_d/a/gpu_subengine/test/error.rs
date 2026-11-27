use super::*;
#[test]
fn gpu_error_is_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<GpuError>();
}
fn assert_error<E: std::error::Error + Send + Sync + 'static>() {}
#[test]
fn is_a_thread_safe_error() {
    assert_error::<GpuError>();
}
#[test]
fn display_texts() {
    assert_eq!(GpuError::NoDevice.to_string(), "keine nutzbare GPU");
    assert_eq!(
        GpuError::InvalidConfig("x".to_string()).to_string(),
        "ungültige GPU-Konfiguration: x"
    );
    assert_eq!(GpuError::Backend("boom".to_string()).to_string(), "GPU-Backend: boom");
    let model = ModelId::new("m");
    let index = IndexId::new("i");
    assert_eq!(
        GpuError::UnknownModel(model.clone()).to_string(),
        format!("Modell {model:?} nicht registriert")
    );
    assert_eq!(
        GpuError::UnknownIndex(index.clone()).to_string(),
        format!("Index {index:?} nicht angebunden")
    );
    assert_eq!(
        GpuError::AlreadyAttached(index.clone()).to_string(),
        format!("Index {index:?} bereits angebunden")
    );
    assert_eq!(
        GpuError::AlreadyRegistered(model.clone()).to_string(),
        format!("Modell {model:?} bereits registriert")
    );
}
#[test]
fn wrapped_errors_keep_the_text_of_their_cause() {
    let e = GpuError::LaneSet(LaneSetError::NoLanes);
    assert!(e.to_string().contains(& LaneSetError::NoLanes.to_string()));
    let e = GpuError::Budget(BudgetError::DoesNotFit {
        requested: 2,
        usable: 1,
    });
    assert!(
        e.to_string().contains(& format!("{}", BudgetError::DoesNotFit { requested : 2,
        usable : 1 }))
    );
    let e = GpuError::Subengine(SubengineError::NotActive);
    assert_eq!(e.to_string(), SubengineError::NotActive.to_string());
    let e = GpuError::Lifecycle(LifecycleError::Failed("nope".to_string()));
    assert!(e.to_string().contains("nope"));
}
