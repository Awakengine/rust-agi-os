pub mod config;
pub mod context;
pub mod integration;
pub mod lifecycle;

// 导出特定函数，避免冲突
pub use config::init as config_init;
pub use config::start as config_start;
pub use config::stop as config_stop;

pub use context::init as context_init;
pub use context::start as context_start;
pub use context::stop as context_stop;

pub use integration::init as integration_init;
pub use integration::start as integration_start;
pub use integration::stop as integration_stop;

pub use lifecycle::init as lifecycle_init;
pub use lifecycle::start as lifecycle_start;
pub use lifecycle::stop as lifecycle_stop;
