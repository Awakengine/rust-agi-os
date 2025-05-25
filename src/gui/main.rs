use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};
use crate::gui::window::WindowManager;
use crate::gui::high_dpi::HighDpiScalingManager;
use crate::gui::input_method::InputMethodManager;
use crate::gui::theme::ThemeManager;
use crate::gui::keyboard_input::KeyboardInputManager;
use crate::gui::mouse_input::MouseInputManager;
use crate::gui::deployment::{DeploymentConfig, DeploymentManager, VirtualMachineType};

/// 主程序入口
pub fn main() -> Result<(), Box<dyn Error>> {
    println!("启动Rust AGI OS GUI桌面环境...");
    
    // 初始化主题管理器
    let theme_manager = Arc::new(Mutex::new(crate::gui::theme::initialize()?));
    
    // 初始化窗口管理器
    let window_manager = Arc::new(Mutex::new(crate::gui::window::initialize()?));
    
    // 初始化高分辨率缩放管理器
    let screen_size = {
        let window_manager = window_manager.lock().unwrap();
        window_manager.get_screen_size()
    };
    let scaling_manager = Arc::new(Mutex::new(crate::gui::high_dpi::initialize(screen_size, theme_manager.clone())?));
    
    // 初始化输入法管理器
    let input_method_manager = Arc::new(Mutex::new(crate::gui::input_method::initialize(theme_manager.clone())?));
    
    // 初始化键盘输入管理器
    let keyboard_manager = Arc::new(Mutex::new(crate::gui::keyboard_input::initialize(input_method_manager.clone())?));
    
    // 初始化鼠标输入管理器
    let mouse_manager = Arc::new(Mutex::new(crate::gui::mouse_input::initialize(window_manager.clone())?));
    
    // 创建桌面环境
    let desktop = crate::gui::desktop::create_desktop(
        window_manager.clone(),
        scaling_manager.clone(),
        input_method_manager.clone(),
        theme_manager.clone(),
        keyboard_manager.clone(),
        mouse_manager.clone(),
    )?;
    
    // 启动桌面环境
    desktop.run()?;
    
    println!("Rust AGI OS GUI桌面环境已关闭");
    
    Ok(())
}

/// 打包并部署到虚拟机
pub fn package_and_deploy_to_vm() -> Result<(), Box<dyn Error>> {
    println!("开始打包并部署到虚拟机...");
    
    // 创建部署配置
    let mut config = crate::gui::deployment::create_deployment_config(
        "/tmp/rust_agi_os_deploy",
        VirtualMachineType::VMware,
    );
    
    // 设置虚拟机磁盘路径
    config.set_vm_disk_path("/tmp/rust_agi_os_vm.vmdk");
    
    // 启用硬件加速和4K分辨率
    config.set_hardware_acceleration(true);
    config.set_4k_resolution(true);
    config.set_input_passthrough(true);
    
    // 创建部署管理器
    let mut manager = crate::gui::deployment::create_deployment_manager(config);
    
    // 打包应用
    let package_dir = manager.package_application(".")?;
    
    // 部署到虚拟机
    manager.deploy_to_vm(&package_dir)?;
    
    // 生成部署文档
    let doc_path = manager.generate_deployment_documentation()?;
    
    println!("打包并部署完成，部署文档: {}", doc_path);
    
    Ok(())
}

/// 在虚拟机环境中进行测试
pub fn test_in_vm() -> Result<(), Box<dyn Error>> {
    println!("在虚拟机环境中进行测试...");
    
    // 初始化主题管理器
    let theme_manager = Arc::new(Mutex::new(crate::gui::theme::initialize()?));
    
    // 初始化窗口管理器
    let window_manager = Arc::new(Mutex::new(crate::gui::window::initialize()?));
    
    // 初始化高分辨率缩放管理器
    let screen_size = {
        let window_manager = window_manager.lock().unwrap();
        window_manager.get_screen_size()
    };
    let scaling_manager = Arc::new(Mutex::new(crate::gui::high_dpi::initialize(screen_size, theme_manager.clone())?));
    
    // 初始化输入法管理器
    let input_method_manager = Arc::new(Mutex::new(crate::gui::input_method::initialize(theme_manager.clone())?));
    
    // 初始化键盘输入管理器
    let keyboard_manager = Arc::new(Mutex::new(crate::gui::keyboard_input::initialize(input_method_manager.clone())?));
    
    // 初始化鼠标输入管理器
    let mouse_manager = Arc::new(Mutex::new(crate::gui::mouse_input::initialize(window_manager.clone())?));
    
    // 创建测试环境
    let test_env = crate::gui::testing::create_test_environment(
        window_manager.clone(),
        scaling_manager.clone(),
        input_method_manager.clone(),
        theme_manager.clone(),
        keyboard_manager.clone(),
        mouse_manager.clone(),
    );
    
    // 创建性能测试器
    let mut performance_tester = crate::gui::testing::create_performance_tester(test_env);
    
    // 运行性能测试
    let mut renderer = crate::gui::render::create_renderer()?;
    performance_tester.test_rendering_performance(&mut *renderer, 100)?;
    performance_tester.test_input_performance(100)?;
    performance_tester.test_window_management_performance(50)?;
    
    println!("虚拟机环境测试完成");
    
    Ok(())
}
