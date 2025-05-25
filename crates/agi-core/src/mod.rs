//! # System Module
//! 
//! This module provides system-level integration and management capabilities for the AGI operating system,
//! including resource management, configuration, monitoring, and lifecycle management.

pub mod resource;
pub mod config;
pub mod monitoring;
pub mod lifecycle;
pub mod integration;

use std::sync::Once;
use std::fmt;
use std::error::Error;

static INIT: Once = Once::new();

/// Initialize the system subsystem
pub fn init() -> Result<(), SystemError> {
    let mut result = Ok(());
    
    INIT.call_once(|| {
        // Initialize resource management
        if let Err(err) = resource::init() {
            result = Err(SystemError::ResourceError(err));
            return;
        }
        
        // Initialize configuration management
        if let Err(err) = config::init() {
            result = Err(SystemError::ConfigError(err));
            return;
        }
        
        // Initialize system monitoring
        if let Err(err) = monitoring::init() {
            result = Err(SystemError::MonitoringError(err));
            return;
        }
        
        // Initialize lifecycle management
        if let Err(err) = lifecycle::init() {
            result = Err(SystemError::LifecycleError(err));
            return;
        }
        
        // Initialize system integration
        if let Err(err) = integration::init() {
            result = Err(SystemError::IntegrationError(err));
            return;
        }
    });
    
    result
}

/// Error type for system operations
#[derive(Debug)]
pub enum SystemError {
    /// Resource management error
    ResourceError(resource::ResourceError),
    /// Configuration management error
    ConfigError(config::ConfigError),
    /// System monitoring error
    MonitoringError(monitoring::MonitoringError),
    /// Lifecycle management error
    LifecycleError(lifecycle::LifecycleError),
    /// System integration error
    IntegrationError(integration::IntegrationError),
    /// General error
    General(&'static str),
}

// 实现Display trait，解决E0277错误
impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemError::ResourceError(e) => write!(f, "Resource error: {}", e),
            SystemError::ConfigError(e) => write!(f, "Config error: {}", e),
            SystemError::MonitoringError(e) => write!(f, "Monitoring error: {}", e),
            SystemError::LifecycleError(e) => write!(f, "Lifecycle error: {}", e),
            SystemError::IntegrationError(e) => write!(f, "Integration error: {}", e),
            SystemError::General(msg) => write!(f, "General system error: {}", msg),
        }
    }
}

// 实现Error trait，解决?操作符错误转换问题
impl Error for SystemError {}

/// System configuration
#[derive(Debug, Clone)]
pub struct SystemConfig {
    /// Resource management configuration
    pub resource_config: resource::ResourceConfig,
    /// Configuration management configuration
    pub config_config: config::ConfigConfig,
    /// System monitoring configuration
    pub monitoring_config: monitoring::MonitoringConfig,
    /// Lifecycle management configuration
    pub lifecycle_config: lifecycle::LifecycleConfig,
    /// System integration configuration
    pub integration_config: integration::IntegrationConfig,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            resource_config: resource::ResourceConfig::default(),
            config_config: config::ConfigConfig::default(),
            monitoring_config: monitoring::MonitoringConfig::default(),
            lifecycle_config: lifecycle::LifecycleConfig::default(),
            integration_config: integration::IntegrationConfig::default(),
        }
    }
}

/// System status
#[derive(Debug, Clone)]
pub struct SystemStatus {
    /// Resource management status
    pub resource_status: resource::ResourceStatus,
    /// Configuration management status
    pub config_status: config::ConfigStatus,
    /// System monitoring status
    pub monitoring_status: monitoring::MonitoringStatus,
    /// Lifecycle management status
    pub lifecycle_status: lifecycle::LifecycleStatus,
    /// System integration status
    pub integration_status: integration::IntegrationStatus,
}

/// Get the current system status
pub fn get_status() -> Result<SystemStatus, SystemError> {
    Ok(SystemStatus {
        resource_status: resource::get_status().map_err(SystemError::ResourceError)?,
        config_status: config::get_status().map_err(SystemError::ConfigError)?,
        monitoring_status: monitoring::get_status().map_err(SystemError::MonitoringError)?,
        lifecycle_status: lifecycle::get_status().map_err(SystemError::LifecycleError)?,
        integration_status: integration::get_status().map_err(SystemError::IntegrationError)?,
    })
}

/// Set the system configuration
pub fn set_config(config: SystemConfig) -> Result<(), SystemError> {
    resource::set_config(config.resource_config).map_err(SystemError::ResourceError)?;
    config::set_config(config.config_config, config::ConfigValue::Null).map_err(SystemError::ConfigError)?;
    monitoring::set_config(config.monitoring_config).map_err(SystemError::MonitoringError)?;
    lifecycle::set_config(config.lifecycle_config).map_err(SystemError::LifecycleError)?;
    integration::set_config(config.integration_config).map_err(SystemError::IntegrationError)?;
    
    Ok(())
}
