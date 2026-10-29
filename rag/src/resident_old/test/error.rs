use super::*;
fn assert_send_sync_static<T: Send + Sync + 'static>() {}
#[test]
fn all_errors_are_send_sync_static() {
    assert_send_sync_static::<ResidentError>();
    assert_send_sync_static::<IndexError>();
    assert_send_sync_static::<ManagerError>();
    assert_send_sync_static::<PrecisionError>();
    assert_send_sync_static::<PlannerError>();
    assert_send_sync_static::<CapabilityError>();
    assert_send_sync_static::<ConfigError>();
    assert_send_sync_static::<DeviceError>();
    assert_send_sync_static::<crate::resident::memory::MemoryError>();
    assert_send_sync_static::<crate::resident::model::ModelError>();
    assert_send_sync_static::<crate::resident::tokenize::TokenizeError>();
    assert_send_sync_static::<crate::resident::kernel::KernelError>();
}
#[test]
fn layer_errors_convert_into_resident_error() {
    let e: ResidentError = IndexError::UnknownBatch {
        batch: 3,
    }
        .into();
    assert!(matches!(e, ResidentError::Index(IndexError::UnknownBatch { batch : 3 })));
    let e: ResidentError = ConfigError::MissingField {
        field: "model".into(),
    }
        .into();
    assert!(matches!(e, ResidentError::Config(_)));
    let e: ResidentError = PrecisionError::NoActiveProfile.into();
    assert!(matches!(e, ResidentError::Precision(_)));
    let e: ResidentError = PlannerError::UnknownModel {
        model: "m".into(),
    }
        .into();
    assert!(matches!(e, ResidentError::Planner(_)));
    let e: ResidentError = CapabilityError::Missing {
        capability: "c".into(),
    }
        .into();
    assert!(matches!(e, ResidentError::Capability(_)));
}
#[test]
fn transparent_wrapper_keeps_message() {
    let inner = IndexError::ArenaExhausted {
        requested: 8,
        available: 2,
    };
    let msg = inner.to_string();
    assert_eq!(ResidentError::from(inner).to_string(), msg);
    assert!(msg.contains('8') && msg.contains('2'));
}
#[test]
fn retryability_classification() {
    assert!(IndexError::ArenaExhausted { requested : 1, available : 0 } .is_retryable());
    assert!(IndexError::IndexUnavailable { scope : "s".into() } .is_retryable());
    assert!(! IndexError::KernelFailed { reason : "r".into() } .is_retryable());
    assert!(ManagerError::StaleRegion { arena : 1, gen : 2 } .is_retryable());
    assert!(PrecisionError::CalibrationUnstable { reason : "r".into() } .is_retryable());
    assert!(! PrecisionError::NoActiveProfile.is_retryable());
    assert!(
        ! ConfigError::InvalidValue { field : "f".into(), reason : "r".into() }
        .is_retryable()
    );
    assert!(! CapabilityError::Unknown { capability : "c".into() } .is_retryable());
    let wrapped: ResidentError = IndexError::ArenaExhausted {
        requested: 1,
        available: 0,
    }
        .into();
    assert!(wrapped.is_retryable());
    let fatal: ResidentError = PlannerError::InvalidConfig {
        reason: "r".into(),
    }
        .into();
    assert!(! fatal.is_retryable());
}
#[test]
fn index_integrity_errors_are_fatal_and_descriptive() {
    let schema = IndexError::IncompatibleSchema {
        expected: "embedding.dim=384".into(),
        found: "embedding.dim=768".into(),
    };
    assert!(! schema.is_retryable());
    assert!(schema.to_string().contains("embedding.dim=768"));
    let sum = IndexError::ChecksumMismatch {
        entry: 7,
    };
    assert!(! sum.is_retryable());
    assert!(sum.to_string().contains('7'));
    let wrapped: ResidentError = sum.into();
    assert!(
        matches!(wrapped, ResidentError::Index(IndexError::ChecksumMismatch { entry : 7
        }))
    );
}
