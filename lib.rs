// AGI操作系统 - 主库文件
// 此文件导出所有公共模块和接口

// 导出核心模块
pub mod kernel;
pub mod neuro_symbolic;
pub mod meta_reasoning;
pub mod interaction;
pub mod security;
pub mod system;
pub mod reflection;

// 导出测试模块
#[cfg(test)]
mod tests {
    #[test]
    fn test_name() {
        let name = env!("CARGO_PKG_NAME");
        assert_eq!(name, "rust_agi_os");
    }

    #[test]
    fn test_version() {
        let version = env!("CARGO_PKG_VERSION");
        assert_eq!(version, "0.1.0");
    }

    #[test]
    fn test_authors() {
        let authors = env!("CARGO_PKG_AUTHORS");
        assert_eq!(authors, "AGI OS Team");
    }

    #[test]
    fn test_description() {
        let description = env!("CARGO_PKG_DESCRIPTION");
        assert_eq!(description, "A strong artificial general intelligence operating system");
    }

    #[test]
    fn test_license() {
        let license = env!("CARGO_PKG_LICENSE");
        assert_eq!(license, "MIT");
    }

    #[test]
    fn test_repository() {
        let repository = env!("CARGO_PKG_REPOSITORY");
        assert_eq!(repository, "https://github.com/agi-os/rust-agi-os");
    }
}

// 系统版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

// 系统状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemStatus {
    Initializing,
    Running,
    Paused,
    Terminating,
    Terminated,
    Error,
}

// 系统配置接口
pub trait ConfigurableSystem {
    fn get_status(&self) -> SystemStatus;
    fn set_config(&mut self, config: &str) -> Result<(), Box<dyn std::error::Error>>;
}

// 系统组件接口
pub trait SystemComponent: ConfigurableSystem {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn pause(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn resume(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

// 系统初始化函数
pub fn initialize_system() -> Result<(), Box<dyn std::error::Error>> {
    println!("初始化 AGI 操作系统 v{}", VERSION);
    
    // 初始化各个核心模块
    kernel::initialize()?;
    neuro_symbolic::initialize()?;
    meta_reasoning::initialize()?;
    interaction::initialize()?;
    security::initialize()?;
    system::initialize()?;
    reflection::initialize()?;
    
    println!("AGI 操作系统初始化完成");
    Ok(())
}

// 系统启动函数
pub fn start_system() -> Result<(), Box<dyn std::error::Error>> {
    println!("启动 AGI 操作系统 v{}", VERSION);
    
    // 启动各个核心模块
    kernel::start()?;
    neuro_symbolic::start()?;
    meta_reasoning::start()?;
    interaction::start()?;
    security::start()?;
    system::start()?;
    reflection::start()?;
    
    println!("AGI 操作系统启动完成");
    Ok(())
}

// 系统停止函数
pub fn stop_system() -> Result<(), Box<dyn std::error::Error>> {
    println!("停止 AGI 操作系统");
    
    // 按照相反的顺序停止各个模块
    reflection::stop()?;
    system::stop()?;
    security::stop()?;
    interaction::stop()?;
    meta_reasoning::stop()?;
    neuro_symbolic::stop()?;
    kernel::stop()?;
    
    println!("AGI 操作系统已停止");
    Ok(())
}
