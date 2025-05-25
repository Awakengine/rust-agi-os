# 内存管理模块单元测试设计

## 概述
本文档设计了kernel/memory.rs模块的单元测试，确保所有public函数和关键路径都有测试覆盖。

## 现有测试分析
memory.rs模块已有以下测试：
- test_memory_allocation：测试基本的内存分配、重新分配和释放功能
- test_memory_protection：测试内存保护和隔离区域创建
- test_memory_config：测试内存配置设置（未完全显示在代码片段中）

## 缺失测试分析
以下是需要补充的测试：

### 1. ProtectionFlags相关测试
- 测试所有ProtectionFlags构造函数
- 测试不同保护标志组合

### 2. MemoryError相关测试
- 测试所有错误类型的创建和显示
- 测试错误处理路径

### 3. IsolatedMemoryRegion相关测试
- 测试边界条件（大小为0、极大值等）
- 测试内存访问方法（as_ptr, as_slice等）

### 4. MemoryManager相关测试
- 测试register_region和unregister_region功能
- 测试find_region功能
- 测试内存区域重叠情况

### 5. 全局函数相关测试
- 测试init函数的重复调用
- 测试内存分配失败情况
- 测试无效参数情况

## 测试设计

### 1. test_protection_flags
```rust
#[test]
fn test_protection_flags() {
    // 测试默认构造函数
    let flags = ProtectionFlags::new(true, false, true);
    assert_eq!(flags.read, true);
    assert_eq!(flags.write, false);
    assert_eq!(flags.execute, true);
    
    // 测试预定义构造函数
    let read_only = ProtectionFlags::read_only();
    assert_eq!(read_only.read, true);
    assert_eq!(read_only.write, false);
    assert_eq!(read_only.execute, false);
    
    let read_write = ProtectionFlags::read_write();
    assert_eq!(read_write.read, true);
    assert_eq!(read_write.write, true);
    assert_eq!(read_write.execute, false);
    
    let read_execute = ProtectionFlags::read_execute();
    assert_eq!(read_execute.read, true);
    assert_eq!(read_execute.write, false);
    assert_eq!(read_execute.execute, true);
    
    let no_access = ProtectionFlags::no_access();
    assert_eq!(no_access.read, false);
    assert_eq!(no_access.write, false);
    assert_eq!(no_access.execute, false);
}
```

### 2. test_memory_error
```rust
#[test]
fn test_memory_error() {
    // 测试错误类型创建
    let errors = [
        MemoryError::OutOfMemory,
        MemoryError::InvalidAlignment,
        MemoryError::InvalidAddress,
        MemoryError::PermissionDenied,
        MemoryError::RegionAlreadyExists,
        MemoryError::RegionNotFound,
        MemoryError::InvalidSize,
        MemoryError::General("Test error message"),
    ];
    
    // 测试错误显示
    for error in &errors {
        let error_string = format!("{}", error);
        assert!(!error_string.is_empty());
    }
}
```

### 3. test_isolated_memory_region
```rust
#[test]
fn test_isolated_memory_region() {
    // 测试正常创建
    let region = IsolatedMemoryRegion::new(
        1024,
        ProtectionFlags::read_write(),
        Some("test_region".to_string()),
    ).unwrap();
    
    assert_eq!(region.region.size, 1024);
    assert_eq!(region.region.protection.read, true);
    assert_eq!(region.region.protection.write, true);
    assert_eq!(region.region.protection.execute, false);
    assert_eq!(region.region.name, Some("test_region".to_string()));
    
    // 测试内存访问方法
    let ptr = region.as_ptr();
    assert!(!ptr.is_null());
    
    let slice = region.as_slice();
    assert_eq!(slice.len(), 1024);
    
    // 测试无效大小
    let result = IsolatedMemoryRegion::new(
        0,
        ProtectionFlags::read_write(),
        None,
    );
    assert!(result.is_err());
    match result {
        Err(MemoryError::InvalidSize) => {},
        _ => panic!("Expected InvalidSize error"),
    }
}
```

### 4. test_memory_manager_regions
```rust
#[test]
fn test_memory_manager_regions() {
    let mut manager = MemoryManager::new();
    
    // 测试注册区域
    let region = MemoryRegion {
        base: 0x2000_0000,
        size: 4096,
        protection: ProtectionFlags::read_write(),
        name: Some("test_region".to_string()),
    };
    
    assert!(manager.register_region(region.clone()).is_ok());
    
    // 测试重复注册
    let result = manager.register_region(region.clone());
    assert!(result.is_err());
    match result {
        Err(MemoryError::RegionAlreadyExists) => {},
        _ => panic!("Expected RegionAlreadyExists error"),
    }
    
    // 测试查找区域
    let found = manager.find_region(0x2000_0000);
    assert!(found.is_some());
    assert_eq!(found.unwrap().base, 0x2000_0000);
    
    let found = manager.find_region(0x2000_0FFF);
    assert!(found.is_some());
    
    let found = manager.find_region(0x2000_1000);
    assert!(found.is_none());
    
    // 测试注销区域
    let removed = manager.unregister_region(0x2000_0000).unwrap();
    assert_eq!(removed.base, 0x2000_0000);
    
    // 测试注销不存在的区域
    let result = manager.unregister_region(0x2000_0000);
    assert!(result.is_err());
    match result {
        Err(MemoryError::RegionNotFound) => {},
        _ => panic!("Expected RegionNotFound error"),
    }
}
```

### 5. test_memory_allocation_edge_cases
```rust
#[test]
fn test_memory_allocation_edge_cases() {
    // 初始化内存子系统
    assert!(init().is_ok());
    
    // 测试分配大小为0
    let ptr = allocate(0, 8).unwrap();
    assert_eq!(ptr as usize, 8);
    
    // 测试无效对齐
    let result = allocate(1024, 3);
    assert!(result.is_err());
    match result {
        Err(MemoryError::InvalidAlignment) => {},
        _ => panic!("Expected InvalidAlignment error"),
    }
    
    // 测试释放空指针
    deallocate(std::ptr::null_mut(), 0, 8);
    
    // 测试重新分配为0大小
    let ptr = allocate(1024, 8).unwrap();
    let new_ptr = reallocate(ptr, 1024, 0, 8).unwrap();
    assert_eq!(new_ptr as usize, 8);
}
```

### 6. test_memory_protection_edge_cases
```rust
#[test]
fn test_memory_protection_edge_cases() {
    // 初始化内存子系统
    assert!(init().is_ok());
    
    // 创建隔离内存区域
    let region = create_isolated_region(
        4096,
        ProtectionFlags::read_write(),
        Some("test_region".to_string()),
    ).unwrap();
    
    // 测试无效地址
    let result = set_protection(
        0x1000_0000,
        1024,
        ProtectionFlags::read_only(),
    );
    assert!(result.is_err());
    match result {
        Err(MemoryError::InvalidAddress) => {},
        _ => panic!("Expected InvalidAddress error"),
    }
    
    // 测试越界保护
    let result = set_protection(
        region.region.base,
        region.region.size + 1,
        ProtectionFlags::read_only(),
    );
    assert!(result.is_err());
    match result {
        Err(MemoryError::InvalidAddress) => {},
        _ => panic!("Expected InvalidAddress error"),
    }
    
    // 测试禁用保护
    let config = MemoryConfig {
        memory_limit: usize::MAX,
        enable_protection: false,
        enable_isolation: true,
    };
    assert!(set_config(config).is_ok());
    
    // 当保护被禁用时，即使地址无效也应成功
    let result = set_protection(
        0x1000_0000,
        1024,
        ProtectionFlags::read_only(),
    );
    assert!(result.is_ok());
}
```

### 7. test_memory_isolation_disabled
```rust
#[test]
fn test_memory_isolation_disabled() {
    // 初始化内存子系统
    assert!(init().is_ok());
    
    // 禁用内存隔离
    let config = MemoryConfig {
        memory_limit: usize::MAX,
        enable_protection: true,
        enable_isolation: false,
    };
    assert!(set_config(config).is_ok());
    
    // 尝试创建隔离区域应该失败
    let result = create_isolated_region(
        4096,
        ProtectionFlags::read_write(),
        Some("test_region".to_string()),
    );
    assert!(result.is_err());
    match result {
        Err(MemoryError::General(_)) => {},
        _ => panic!("Expected General error"),
    }
}
```

### 8. test_memory_stats
```rust
#[test]
fn test_memory_stats() {
    // 创建默认分配器
    let allocator = DefaultAllocator::new();
    
    // 初始状态
    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated, 0);
    assert_eq!(stats.total_deallocated, 0);
    assert_eq!(stats.allocation_count, 0);
    assert_eq!(stats.deallocation_count, 0);
    assert_eq!(stats.reallocation_count, 0);
    assert_eq!(stats.current_usage(), 0);
    
    // 分配内存
    let ptr = allocator.allocate(1024, 8).unwrap();
    
    // 检查统计
    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated, 1024);
    assert_eq!(stats.allocation_count, 1);
    assert_eq!(stats.current_usage(), 1024);
    
    // 重新分配
    let new_ptr = allocator.reallocate(ptr, 1024, 2048, 8).unwrap();
    
    // 检查统计
    let stats = allocator.get_stats();
    assert_eq!(stats.reallocation_count, 1);
    assert_eq!(stats.current_usage(), 2048);
    
    // 释放内存
    allocator.deallocate(new_ptr, 2048, 8);
    
    // 检查统计
    let stats = allocator.get_stats();
    assert_eq!(stats.deallocation_count, 1);
    assert_eq!(stats.current_usage(), 0);
}
```

## 实现计划
1. 将上述测试函数添加到memory.rs的tests模块中
2. 运行测试并修复任何失败
3. 确认所有public函数都有测试覆盖
4. 检查边界条件和错误处理路径的覆盖情况
