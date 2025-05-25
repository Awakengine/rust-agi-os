// AGI操作系统 - 主程序入口
// 此文件是AGI操作系统的启动入口点

use rust_agi_os::kernel::{memory, process};
use rust_agi_os::neuro_symbolic::{neural, symbolic, learning};
use rust_agi_os::meta_reasoning::{reasoning, planning};
use rust_agi_os::interaction::{multimodal, language, context, interface};
use rust_agi_os::security::{verification, threat_detection, sandbox};
use rust_agi_os::system::{resource, config, lifecycle, monitoring, integration};
use rust_agi_os::reflection::{performance, reflection};

use std::sync::{Arc, Mutex};
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    println!("启动AGI操作系统...");
    
    // 初始化系统配置
    let system_config = match config::SystemConfig::load_from_file("config/system.json") {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("无法加载系统配置: {}", e);
            process::exit(1);
        }
    };
    
    println!("系统配置加载完成");
    
    // 初始化内存管理
    let memory_manager = match memory::MemoryManager::new(system_config.get_memory_config()) {
        Ok(mm) => Arc::new(Mutex::new(mm)),
        Err(e) => {
            eprintln!("内存管理初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("内存管理初始化完成");
    
    // 初始化进程管理
    let process_manager = match process::ProcessManager::new(system_config.get_process_config(), memory_manager.clone()) {
        Ok(pm) => Arc::new(Mutex::new(pm)),
        Err(e) => {
            eprintln!("进程管理初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("进程管理初始化完成");
    
    // 初始化安全沙箱
    let sandbox_manager = match sandbox::SandboxManager::new(system_config.get_security_config()) {
        Ok(sm) => Arc::new(Mutex::new(sm)),
        Err(e) => {
            eprintln!("安全沙箱初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("安全沙箱初始化完成");
    
    // 初始化神经网络引擎
    let neural_engine = match neural::NeuralEngine::new(system_config.get_neural_config()) {
        Ok(ne) => Arc::new(Mutex::new(ne)),
        Err(e) => {
            eprintln!("神经网络引擎初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("神经网络引擎初始化完成");
    
    // 初始化符号系统
    let symbolic_system = match symbolic::SymbolicSystem::new(system_config.get_symbolic_config()) {
        Ok(ss) => Arc::new(Mutex::new(ss)),
        Err(e) => {
            eprintln!("符号系统初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("符号系统初始化完成");
    
    // 初始化学习系统
    let learning_system = match learning::LearningSystem::new(
        system_config.get_learning_config(),
        neural_engine.clone(),
        symbolic_system.clone()
    ) {
        Ok(ls) => Arc::new(Mutex::new(ls)),
        Err(e) => {
            eprintln!("学习系统初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("学习系统初始化完成");
    
    // 初始化推理引擎
    let reasoning_engine = match reasoning::ReasoningEngine::new(
        system_config.get_reasoning_config(),
        symbolic_system.clone(),
        learning_system.clone()
    ) {
        Ok(re) => Arc::new(Mutex::new(re)),
        Err(e) => {
            eprintln!("推理引擎初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("推理引擎初始化完成");
    
    // 初始化规划系统
    let planning_system = match planning::PlanningSystem::new(
        system_config.get_planning_config(),
        reasoning_engine.clone()
    ) {
        Ok(ps) => Arc::new(Mutex::new(ps)),
        Err(e) => {
            eprintln!("规划系统初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("规划系统初始化完成");
    
    // 初始化多模态交互引擎
    let multimodal_engine = match multimodal::MultimodalEngine::new(system_config.get_multimodal_config()) {
        Ok(me) => Arc::new(Mutex::new(me)),
        Err(e) => {
            eprintln!("多模态交互引擎初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("多模态交互引擎初始化完成");
    
    // 初始化语言处理器
    let language_processor = match language::LanguageProcessor::new(system_config.get_language_config()) {
        Ok(lp) => Arc::new(Mutex::new(lp)),
        Err(e) => {
            eprintln!("语言处理器初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("语言处理器初始化完成");
    
    // 初始化上下文管理器
    let context_manager = match context::ContextManager::new(system_config.get_context_config()) {
        Ok(cm) => Arc::new(Mutex::new(cm)),
        Err(e) => {
            eprintln!("上下文管理器初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("上下文管理器初始化完成");
    
    // 初始化接口管理器
    let interface_manager = match interface::InterfaceManager::new(system_config.get_interface_config()) {
        Ok(im) => Arc::new(Mutex::new(im)),
        Err(e) => {
            eprintln!("接口管理器初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("接口管理器初始化完成");
    
    // 初始化验证引擎
    let verification_engine = match verification::VerificationEngine::new(
        system_config.get_verification_config(),
        reasoning_engine.clone()
    ) {
        Ok(ve) => Arc::new(Mutex::new(ve)),
        Err(e) => {
            eprintln!("验证引擎初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("验证引擎初始化完成");
    
    // 初始化威胁检测系统
    let threat_detector = match threat_detection::ThreatDetector::new(system_config.get_threat_config()) {
        Ok(td) => Arc::new(Mutex::new(td)),
        Err(e) => {
            eprintln!("威胁检测系统初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("威胁检测系统初始化完成");
    
    // 初始化资源管理器
    let resource_manager = match resource::ResourceManager::new(
        system_config.get_resource_config(),
        memory_manager.clone(),
        process_manager.clone()
    ) {
        Ok(rm) => Arc::new(Mutex::new(rm)),
        Err(e) => {
            eprintln!("资源管理器初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("资源管理器初始化完成");
    
    // 初始化监控系统
    let monitoring_system = match monitoring::MonitoringSystem::new(
        system_config.get_monitoring_config(),
        resource_manager.clone()
    ) {
        Ok(ms) => Arc::new(Mutex::new(ms)),
        Err(e) => {
            eprintln!("监控系统初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("监控系统初始化完成");
    
    // 初始化集成接口
    let integration_interface = match integration::IntegrationInterface::new(
        system_config.get_integration_config(),
        interface_manager.clone()
    ) {
        Ok(ii) => Arc::new(Mutex::new(ii)),
        Err(e) => {
            eprintln!("集成接口初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("集成接口初始化完成");
    
    // 初始化性能监控
    let performance_monitor = match performance::PerformanceMonitor::new(
        system_config.get_performance_config(),
        monitoring_system.clone()
    ) {
        Ok(pm) => Arc::new(Mutex::new(pm)),
        Err(e) => {
            eprintln!("性能监控初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("性能监控初始化完成");
    
    // 初始化反思系统
    let reflection_system = match reflection::ReflectionSystem::new(
        system_config.get_reflection_config(),
        reasoning_engine.clone(),
        performance_monitor.clone()
    ) {
        Ok(rs) => Arc::new(Mutex::new(rs)),
        Err(e) => {
            eprintln!("反思系统初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("反思系统初始化完成");
    
    // 初始化生命周期管理器
    let lifecycle_manager = match lifecycle::LifecycleManager::new(
        system_config.get_lifecycle_config(),
        vec![
            resource_manager.clone(),
            monitoring_system.clone(),
            integration_interface.clone(),
            performance_monitor.clone(),
            reflection_system.clone()
        ]
    ) {
        Ok(lm) => Arc::new(Mutex::new(lm)),
        Err(e) => {
            eprintln!("生命周期管理器初始化失败: {}", e);
            process::exit(1);
        }
    };
    
    println!("生命周期管理器初始化完成");
    
    // 启动系统
    match lifecycle_manager.lock().unwrap().start_system() {
        Ok(_) => println!("AGI操作系统启动成功"),
        Err(e) => {
            eprintln!("AGI操作系统启动失败: {}", e);
            process::exit(1);
        }
    }
    
    // 运行系统主循环
    match lifecycle_manager.lock().unwrap().run_system_loop() {
        Ok(_) => println!("AGI操作系统正常关闭"),
        Err(e) => {
            eprintln!("AGI操作系统运行错误: {}", e);
            process::exit(1);
        }
    }
    
    Ok(())
}