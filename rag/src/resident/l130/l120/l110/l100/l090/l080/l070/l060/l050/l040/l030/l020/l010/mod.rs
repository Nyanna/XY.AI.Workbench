pub mod background_pool;
pub mod capability_model;
pub mod completion_wait;
pub mod extensions;
pub mod queue;
pub mod queue_state;
pub mod statistics;
pub use completion_wait::{CompletionNotifier, Subscription};
pub use extensions::Extensions;
pub use background_pool::{BackgroundPool, JobError, JobFuture, PoolError};
pub use statistics::{
    dispersion, estimate, median, minimum, Dispersion, Estimator, MIN_DISPERSION_SAMPLES,
};
