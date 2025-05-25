mod config;
mod resource;
mod lifecycle;
mod monitoring;
mod integration;

// 使用具体的模块导出，避免glob导出冲突
pub use config::{SystemConfig, SystemConfigManager, ConfigError};
pub use resource::{Resource, ResourceManager, ResourceError};
pub use lifecycle::{SystemLifecycle, SystemLifecycleManager, LifecycleError};
pub use monitoring::{Monitoring, MonitoringManager, MonitoringError, MetricType};
pub use integration::{SystemIntegration, SystemIntegrationManager, IntegrationError};

// 导出特定函数，避免冲突
pub use config::init as config_init;
pub use config::start as config_start;
pub use config::stop as config_stop;

pub use resource::init as resource_init;
pub use resource::start as resource_start;
pub use resource::stop as resource_stop;

pub use lifecycle::init as lifecycle_init;
pub use lifecycle::start as lifecycle_start;
pub use lifecycle::stop as lifecycle_stop;

pub use monitoring::init as monitoring_init;
pub use monitoring::start as monitoring_start;
pub use monitoring::stop as monitoring_stop;

pub use integration::init as integration_init;
pub use integration::start as integration_start;
pub use integration::stop as integration_stop;
