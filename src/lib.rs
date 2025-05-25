pub mod core;
pub mod gui;
pub mod interaction;
pub mod kernel;
pub mod meta_reasoning;
pub mod neuro_symbolic;
pub mod reflection;
pub mod security;
pub mod system;

// 使用具体的模块导出，避免glob导出冲突
// Core模块导出
pub use crate::core::config::{Config, ConfigManager, ConfigError};
pub use crate::core::context::{Context, ContextManager, ContextError};
pub use crate::core::integration::{Integration as CoreIntegration, IntegrationManager as CoreIntegrationManager, IntegrationError as CoreIntegrationError};
pub use crate::core::lifecycle::{Lifecycle as CoreLifecycle, LifecycleManager as CoreLifecycleManager, LifecycleError as CoreLifecycleError};

// GUI模块导出
pub use crate::gui::window::{Window, WindowError};
pub use crate::gui::render::{Renderer, Color, RenderError};
pub use crate::gui::window_system::{WindowSystem, WindowSystemError};
pub use crate::gui::theme::{Theme, ThemeManager, ThemeError};
pub use crate::gui::desktop::{Desktop, DesktopManager, DesktopError, MacMenuBar};
pub use crate::gui::window_manager::{WindowManager, WindowManagerError};

// Interaction模块导出
pub use crate::interaction::vision::{VisionSystem, Image, Object, VisionError};
pub use crate::interaction::speech::{SpeechSystem, Audio, Recording, SpeechError};
pub use crate::interaction::natural_language::{NaturalLanguageSystem, Language, Sentiment, Entity, Intent, NaturalLanguageError};

// Kernel模块导出
pub use crate::kernel::memory::{Memory, MemoryManager, MemoryError};
pub use crate::kernel::process::{Process, ProcessManager, ProcessError};

// Meta-reasoning模块导出
pub use crate::meta_reasoning::planning::{Planning, PlanningSystem, PlanningError};
pub use crate::meta_reasoning::reasoning::{Reasoning, ReasoningSystem, ReasoningError};

// Neuro-symbolic模块导出
pub use crate::neuro_symbolic::neural::{NeuralNetwork, NeuralNetworkManager, NeuralError};
pub use crate::neuro_symbolic::symbolic::{Symbol, SymbolicSystem, SymbolicError};
pub use crate::neuro_symbolic::knowledge::{Knowledge, KnowledgeBase, KnowledgeError};
pub use crate::neuro_symbolic::learning::{Learning, LearningSystem, LearningError};
pub use crate::neuro_symbolic::integration::{Integration as NeuroSymbolicIntegration, IntegrationSystem, IntegrationError as NeuroSymbolicIntegrationError};

// Reflection模块导出
pub use crate::reflection::performance::{Performance, PerformanceMetric, PerformanceError, MetricType as ReflectionMetricType};
pub use crate::reflection::reflection::{Reflection, ReflectionManager, ReflectionError};

// Security模块导出
pub use crate::security::sandbox::{Sandbox, SandboxManager, SandboxError};
pub use crate::security::verification::{Verification, VerificationManager, VerificationError};
pub use crate::security::threat_detection::{ThreatDetection, ThreatDetectionManager, ThreatDetectionError};
pub use crate::security::access_control::{AccessControl, AccessControlManager, AccessControlError};

// System模块导出
pub use crate::system::config::{SystemConfig, SystemConfigManager, ConfigError as SystemConfigError};
pub use crate::system::resource::{Resource, ResourceManager, ResourceError};
pub use crate::system::lifecycle::{SystemLifecycle, SystemLifecycleManager, LifecycleError as SystemLifecycleError};
pub use crate::system::monitoring::{Monitoring, MonitoringManager, MonitoringError, MetricType as SystemMetricType};
pub use crate::system::integration::{SystemIntegration, SystemIntegrationManager, IntegrationError as SystemIntegrationError};

// 导出特定函数，避免冲突
pub use crate::core::config_init;
pub use crate::core::config_start;
pub use crate::core::config_stop;

pub use crate::gui::window_init;
pub use crate::gui::window_start;
pub use crate::gui::window_stop;

pub use crate::security::sandbox_init;
pub use crate::security::sandbox_start;
pub use crate::security::sandbox_stop;

pub use crate::system::lifecycle_init;
pub use crate::system::lifecycle_start;
pub use crate::system::lifecycle_stop;
