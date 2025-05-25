#[test]
fn test_create_sandbox_with_default_config() {
    // 初始化沙箱子系统
    init().unwrap();
    
    // 使用默认配置创建沙箱
    let sandbox_id = create_sandbox("test_sandbox", None, None, None).unwrap();
    
    // 验证沙箱已创建
    let registry = SANDBOX_REGISTRY.lock().unwrap();
    let sandbox = registry.get_sandbox(sandbox_id).unwrap();
    let sandbox = sandbox.lock().unwrap();
    
    assert_eq!(sandbox.name(), "test_sandbox");
    assert_eq!(sandbox.state(), SandboxState::Created);
    assert_eq!(sandbox.resource_limits().memory_limit, 100 * 1024 * 1024); // 默认100MB
    assert_eq!(sandbox.resource_limits().cpu_limit, 10.0); // 默认10%
    assert!(!sandbox.capabilities().network_access); // 默认禁止网络访问
    assert!(!sandbox.capabilities().filesystem_access); // 默认禁止文件系统访问
    assert_eq!(sandbox.working_dir(), &PathBuf::from("/tmp")); // 默认工作目录
}

#[test]
fn test_create_sandbox_with_custom_resource_limits() {
    // 初始化沙箱子系统
    init().unwrap();
    
    // 自定义资源限制
    let resource_limits = ResourceLimits {
        memory_limit: 200 * 1024 * 1024, // 200MB
        cpu_limit: 20.0, // 20%
        network_bandwidth_limit: Some(1024 * 1024), // 1MB/s
        filesystem_space_limit: Some(500 * 1024 * 1024), // 500MB
        max_execution_time: Some(60.0), // 60秒
    };
    
    // 创建具有自定义资源限制的沙箱
    let sandbox_id = create_sandbox("test_sandbox", Some(resource_limits.clone()), None, None).unwrap();
    
    // 验证沙箱已创建且具有自定义资源限制
    let registry = SANDBOX_REGISTRY.lock().unwrap();
    let sandbox = registry.get_sandbox(sandbox_id).unwrap();
    let sandbox = sandbox.lock().unwrap();
    
    assert_eq!(sandbox.resource_limits().memory_limit, 200 * 1024 * 1024);
    assert_eq!(sandbox.resource_limits().cpu_limit, 20.0);
    assert_eq!(sandbox.resource_limits().network_bandwidth_limit, Some(1024 * 1024));
    assert_eq!(sandbox.resource_limits().filesystem_space_limit, Some(500 * 1024 * 1024));
    assert_eq!(sandbox.resource_limits().max_execution_time, Some(60.0));
}

#[test]
fn test_create_sandbox_with_custom_capabilities() {
    // 初始化沙箱子系统
    init().unwrap();
    
    // 自定义能力
    let mut capabilities = Capabilities::default();
    capabilities.network_access = true;
    capabilities.filesystem_access = true;
    capabilities.process_creation = true;
    capabilities.system_calls.push("open".to_string());
    capabilities.system_calls.push("read".to_string());
    capabilities.device_access.insert("gpu".to_string(), Permission::ReadWrite);
    
    // 创建具有自定义能力的沙箱
    let sandbox_id = create_sandbox("test_sandbox", None, Some(capabilities), None).unwrap();
    
    // 验证沙箱已创建且具有自定义能力
    let registry = SANDBOX_REGISTRY.lock().unwrap();
    let sandbox = registry.get_sandbox(sandbox_id).unwrap();
    let sandbox = sandbox.lock().unwrap();
    
    assert!(sandbox.capabilities().network_access);
    assert!(sandbox.capabilities().filesystem_access);
    assert!(sandbox.capabilities().process_creation);
    assert!(sandbox.capabilities().system_calls.contains(&"open".to_string()));
    assert!(sandbox.capabilities().system_calls.contains(&"read".to_string()));
    assert_eq!(sandbox.capabilities().device_access.get("gpu"), Some(&Permission::ReadWrite));
}

#[test]
fn test_create_sandbox_with_custom_working_dir() {
    // 初始化沙箱子系统
    init().unwrap();
    
    // 自定义工作目录
    let working_dir = PathBuf::from("/home/user/sandbox");
    
    // 创建具有自定义工作目录的沙箱
    let sandbox_id = create_sandbox("test_sandbox", None, None, Some(working_dir.clone())).unwrap();
    
    // 验证沙箱已创建且具有自定义工作目录
    let registry = SANDBOX_REGISTRY.lock().unwrap();
    let sandbox = registry.get_sandbox(sandbox_id).unwrap();
    let sandbox = sandbox.lock().unwrap();
    
    assert_eq!(sandbox.working_dir(), &working_dir);
}

#[test]
fn test_start_already_running_sandbox() {
    // 初始化沙箱子系统
    init().unwrap();
    
    // 创建并启动沙箱
    let sandbox_id = create_sandbox("already_running_test", None, None, None).unwrap();
    start_sandbox(sandbox_id).unwrap();
    
    // 尝试再次启动已运行的沙箱
    let result = start_sandbox(sandbox_id);
    
    // 验证错误
    assert!(result.is_err());
    if let Err(SandboxError::General(msg)) = result {
        assert_eq!(msg, "Sandbox is not in a startable state");
    } else {
        panic!("Expected SandboxError::General");
    }
}

#[test]
fn test_pause_not_running_sandbox() {
    // 初始化沙箱子系统
    init().unwrap();
    
    // 创建沙箱但不启动
    let sandbox_id = create_sandbox("not_running_test", None, None, None).unwrap();
    
    // 尝试暂停未运行的沙箱
    let result = pause_sandbox(sandbox_id);
    
    // 验证错误
    assert!(result.is_err());
    if let Err(SandboxError::General(msg)) = result {
        assert_eq!(msg, "Sandbox is not running");
    } else {
        panic!("Expected SandboxError::General");
    }
}

#[test]
fn test_resume_not_paused_sandbox() {
    // 初始化沙箱子系统
    init().unwrap();
    
    // 创建并启动沙箱
    let sandbox_id = create_sandbox("not_paused_test", None, None, None).unwrap();
    start_sandbox(sandbox_id).unwrap();
    
    // 尝试恢复未暂停的沙箱
    let result = resume_sandbox(sandbox_id);
    
    // 验证错误
    assert!(result.is_err());
    if let Err(SandboxError::General(msg)) = result {
        assert_eq!(msg, "Sandbox is not paused");
    } else {
        panic!("Expected SandboxError::General");
    }
}

#[test]
fn test_terminate_already_terminated_sandbox() {
    // 初始化沙箱子系统
    init().unwrap();
    
    // 创建、启动并终止沙箱
    let sandbox_id = create_sandbox("already_terminated_test", None, None, None).unwrap();
    start_sandbox(sandbox_id).unwrap();
    terminate_sandbox(sandbox_id).unwrap();
    
    // 尝试再次终止已终止的沙箱
    let result = terminate_sandbox(sandbox_id);
    
    // 验证错误
    assert!(result.is_err());
    if let Err(SandboxError::General(msg)) = result {
        assert_eq!(msg, "Sandbox is already terminated");
    } else {
        panic!("Expected SandboxError::General");
    }
}

#[test]
fn test_execute_in_not_running_sandbox() {
    // 初始化沙箱子系统
    init().unwrap();
    
    // 创建沙箱但不启动
    let sandbox_id = create_sandbox("not_running_exec_test", None, None, None).unwrap();
    
    // 尝试在未运行的沙箱中执行代码
    let result = execute_in_sandbox(sandbox_id, "test code");
    
    // 验证错误
    assert!(result.is_err());
    if let Err(SandboxError::General(msg)) = result {
        assert_eq!(msg, "Sandbox is not running");
    } else {
        panic!("Expected SandboxError::General");
    }
}

#[test]
fn test_get_nonexistent_sandbox() {
    // 初始化沙箱子系统
    init().unwrap();
    
    // 尝试获取不存在的沙箱
    let registry = SANDBOX_REGISTRY.lock().unwrap();
    let result = registry.get_sandbox(SandboxId(9999));
    
    // 验证错误
    assert!(result.is_err());
    if let Err(SandboxError::General(msg)) = result {
        assert_eq!(msg, "Sandbox not found");
    } else {
        panic!("Expected SandboxError::General");
    }
}

#[test]
fn test_remove_nonexistent_sandbox() {
    // 初始化沙箱子系统
    init().unwrap();
    
    // 尝试移除不存在的沙箱
    let mut registry = SANDBOX_REGISTRY.lock().unwrap();
    let result = registry.remove_sandbox(SandboxId(9999));
    
    // 验证错误
    assert!(result.is_err());
    if let Err(SandboxError::General(msg)) = result {
        assert_eq!(msg, "Sandbox not found");
    } else {
        panic!("Expected SandboxError::General");
    }
}

#[test]
fn test_sandbox_registry_set_config() {
    // 初始化沙箱子系统
    init().unwrap();
    
    // 创建自定义配置
    let config = SandboxConfig {
        enable_memory_isolation: false,
        enable_process_isolation: false,
        enable_resource_limits: true,
        enable_capabilities: true,
        default_memory_limit: 200 * 1024 * 1024, // 200MB
        default_cpu_limit: 20.0, // 20%
        default_network_access: true,
        default_filesystem_access: true,
    };
    
    // 设置配置
    {
        let mut registry = SANDBOX_REGISTRY.lock().unwrap();
        registry.set_config(config.clone());
    }
    
    // 创建沙箱并验证默认值是否反映了新配置
    let sandbox_id = create_sandbox("config_test", None, None, None).unwrap();
    
    let registry = SANDBOX_REGISTRY.lock().unwrap();
    let sandbox = registry.get_sandbox(sandbox_id).unwrap();
    let sandbox = sandbox.lock().unwrap();
    
    assert_eq!(sandbox.resource_limits().memory_limit, 200 * 1024 * 1024);
    assert_eq!(sandbox.resource_limits().cpu_limit, 20.0);
    assert!(sandbox.capabilities().network_access);
    assert!(sandbox.capabilities().filesystem_access);
}

#[test]
fn test_sandbox_registry_get_status() {
    // 初始化沙箱子系统
    init().unwrap();
    
    // 创建初始状态
    let initial_registry = SANDBOX_REGISTRY.lock().unwrap();
    let initial_status = initial_registry.get_status();
    let initial_active_count = initial_status.active_sandbox_count;
    let initial_terminated_count = initial_status.terminated_sandbox_count;
    drop(initial_registry);
    
    // 创建并终止一个沙箱
    let sandbox_id = create_sandbox("status_test", None, None, None).unwrap();
    start_sandbox(sandbox_id).unwrap();
    terminate_sandbox(sandbox_id).unwrap();
    
    // 移除沙箱
    {
        let mut registry = SANDBOX_REGISTRY.lock().unwrap();
        registry.remove_sandbox(sandbox_id).unwrap();
    }
    
    // 获取更新后的状态
    let registry = SANDBOX_REGISTRY.lock().unwrap();
    let status = registry.get_status();
    
    // 验证状态
    assert_eq!(status.active_sandbox_count, initial_active_count);
    assert_eq!(status.terminated_sandbox_count, initial_terminated_count + 1);
}

#[test]
fn test_has_capability() {
    // 初始化沙箱子系统
    init().unwrap();
    
    // 创建具有特定能力的沙箱
    let mut capabilities = Capabilities::default();
    capabilities.network_access = true;
    capabilities.system_calls.push("open".to_string());
    
    let sandbox_id = create_sandbox("capability_test", None, Some(capabilities), None).unwrap();
    
    // 获取沙箱
    let registry = SANDBOX_REGISTRY.lock().unwrap();
    let sandbox = registry.get_sandbox(sandbox_id).unwrap();
    let sandbox = sandbox.lock().unwrap();
    
    // 验证能力检查
    assert!(sandbox.has_capability("network_access"));
    assert!(!sandbox.has_capability("filesystem_access"));
    assert!(sandbox.has_capability("open"));
    assert!(!sandbox.has_capability("close"));
}

#[test]
fn test_sandbox_error_display() {
    // 测试错误显示
    let errors = vec![
        (SandboxError::MemoryError("out of memory".to_string()), "Memory error: out of memory"),
        (SandboxError::ProcessError(ProcessError::NotFound), "Process error: Process not found"),
        (SandboxError::ResourceLimitExceeded("memory limit".to_string()), "Resource limit exceeded: memory limit"),
        (SandboxError::PermissionDenied("network access".to_string()), "Permission denied: network access"),
        (SandboxError::CreationError("invalid config".to_string()), "Sandbox creation error: invalid config"),
        (SandboxError::ExecutionError("runtime error".to_string()), "Execution error: runtime error"),
        (SandboxError::General("general error"), "General error: general error"),
    ];
    
    for (error, expected) in errors {
        assert_eq!(format!("{}", error), expected);
    }
}
