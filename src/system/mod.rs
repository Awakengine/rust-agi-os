// 系统管理模块 - 资源管理
pub mod resource;

// 系统管理模块 - 配置管理
pub mod configuration;

// 系统管理模块 - 生命周期管理
pub mod lifecycle;

// 系统管理模块 - 监控系统
pub mod monitoring;

// 系统管理模块 - 集成接口
pub mod integration;

// 系统管理模块 - 核心功能
pub mod core;

// 导出系统核心初始化函数
pub use core::initialize;

// 导出系统启动函数
pub use lifecycle::start;

// 导出系统停止函数
pub use lifecycle::stop;