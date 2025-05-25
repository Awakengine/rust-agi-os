//! # Memory Management Module
//! 
//! This module provides safe memory management primitives for the AGI operating system,
//! including memory allocation, protection, and isolation.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fmt;

// 正确导入lazy_static宏
use lazy_static::lazy_static;

/// Memory allocation error
#[derive(Debug, Clone)]
pub enum MemoryError {
    /// Out of memory
    OutOfMemory,
    /// Invalid alignment
    InvalidAlignment,
    /// Invalid address
    InvalidAddress,
    /// Permission denied
    PermissionDenied,
    /// Region already exists
    RegionAlreadyExists,
    /// Region not found
    RegionNotFound,
    /// Invalid size
    InvalidSize,
    /// General error
    General(&'static str),
}

impl std::error::Error for MemoryError {}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::OutOfMemory => write!(f, "Out of memory"),
            MemoryError::InvalidAlignment => write!(f, "Invalid alignment"),
            MemoryError::InvalidAddress => write!(f, "Invalid address"),
            MemoryError::PermissionDenied => write!(f, "Permission denied"),
            MemoryError::RegionAlreadyExists => write!(f, "Region already exists"),
            MemoryError::RegionNotFound => write!(f, "Region not found"),
            MemoryError::InvalidSize => write!(f, "Invalid size"),
            MemoryError::General(msg) => write!(f, "General error: {}", msg),
        }
    }
}

/// Memory protection flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtectionFlags {
    /// Read permission
    pub read: bool,
    /// Write permission
    pub write: bool,
    /// Execute permission
    pub execute: bool,
}

impl ProtectionFlags {
    /// Create new protection flags
    pub fn new(read: bool, write: bool, execute: bool) -> Self {
        Self { read, write, execute }
    }
    
    /// Read-only protection
    pub fn read_only() -> Self {
        Self { read: true, write: false, execute: false }
    }
    
    /// Read-write protection
    pub fn read_write() -> Self {
        Self { read: true, write: true, execute: false }
    }
    
    /// Read-execute protection
    pub fn read_execute() -> Self {
        Self { read: true, write: false, execute: true }
    }
    
    /// No access
    pub fn no_access() -> Self {
        Self { read: false, write: false, execute: false }
    }
}

/// Memory region information
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    /// Base address
    pub base: usize,
    /// Size in bytes
    pub size: usize,
    /// Protection flags
    pub protection: ProtectionFlags,
    /// Region name (optional)
    pub name: Option<String>,
}

/// Isolated memory region
#[derive(Debug)]
pub struct IsolatedMemoryRegion {
    /// Region information
    pub region: MemoryRegion,
    /// Internal data
    _data: Vec<u8>,
}

impl IsolatedMemoryRegion {
    /// Create a new isolated memory region
    pub fn new(size: usize, protection: ProtectionFlags, name: Option<String>) -> Result<Self, MemoryError> {
        if size == 0 {
            return Err(MemoryError::InvalidSize);
        }
        
        let base = NEXT_REGION_BASE.fetch_add(size, Ordering::SeqCst);
        let _data = vec![0; size];
        
        Ok(Self {
            region: MemoryRegion {
                base,
                size,
                protection,
                name,
            },
            _data,
        })
    }
    
    /// Get a pointer to the region
    pub fn as_ptr(&self) -> *const u8 {
        self._data.as_ptr()
    }
    
    /// Get a mutable pointer to the region
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self._data.as_mut_ptr()
    }
    
    /// Get a slice of the region
    pub fn as_slice(&self) -> &[u8] {
        &self._data
    }
    
    /// Get a mutable slice of the region
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self._data
    }
}

/// Memory allocator trait
pub trait SafeMemoryAllocator: Send + Sync + fmt::Debug {
    /// Allocate memory
    fn allocate(&self, size: usize, align: usize) -> Result<*mut u8, MemoryError>;
    
    /// Deallocate memory
    fn deallocate(&self, ptr: *mut u8, size: usize, align: usize);
    
    /// Reallocate memory
    fn reallocate(&self, ptr: *mut u8, old_size: usize, new_size: usize, align: usize) 
        -> Result<*mut u8, MemoryError>;
        
    /// Get memory statistics
    fn get_stats(&self) -> MemoryStats;
}

/// Default memory allocator
#[derive(Debug)]
pub struct DefaultAllocator {
    /// Allocation statistics
    stats: Arc<Mutex<MemoryStats>>,
}

impl DefaultAllocator {
    /// Create a new default allocator
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(MemoryStats::default())),
        }
    }
}

impl SafeMemoryAllocator for DefaultAllocator {
    fn allocate(&self, size: usize, align: usize) -> Result<*mut u8, MemoryError> {
        if size == 0 {
            return Ok(align as *mut u8);
        }
        
        if !align.is_power_of_two() {
            return Err(MemoryError::InvalidAlignment);
        }
        
        let layout = std::alloc::Layout::from_size_align(size, align)
            .map_err(|_| MemoryError::InvalidAlignment)?;
            
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            return Err(MemoryError::OutOfMemory);
        }
        
        // Update statistics - 使用作用域限制锁的生命周期
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_allocated += size;
            stats.allocation_count += 1;
        }
        
        Ok(ptr)
    }
    
    fn deallocate(&self, ptr: *mut u8, size: usize, align: usize) {
        if ptr.is_null() || size == 0 {
            return;
        }
        
        let layout = match std::alloc::Layout::from_size_align(size, align) {
            Ok(layout) => layout,
            Err(_) => return,
        };
        
        unsafe { std::alloc::dealloc(ptr, layout) };
        
        // Update statistics - 使用作用域限制锁的生命周期
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_deallocated += size;
            stats.deallocation_count += 1;
        }
    }
    
    fn reallocate(&self, ptr: *mut u8, old_size: usize, new_size: usize, align: usize) 
        -> Result<*mut u8, MemoryError> {
        if new_size == 0 {
            self.deallocate(ptr, old_size, align);
            return Ok(align as *mut u8);
        }
        
        if !align.is_power_of_two() {
            return Err(MemoryError::InvalidAlignment);
        }
        
        let old_layout = std::alloc::Layout::from_size_align(old_size, align)
            .map_err(|_| MemoryError::InvalidAlignment)?;
            
        let new_layout = std::alloc::Layout::from_size_align(new_size, align)
            .map_err(|_| MemoryError::InvalidAlignment)?;
            
        let new_ptr = if ptr.is_null() {
            unsafe { std::alloc::alloc(new_layout) }
        } else {
            unsafe { std::alloc::realloc(ptr, old_layout, new_size) }
        };
        
        if new_ptr.is_null() {
            return Err(MemoryError::OutOfMemory);
        }
        
        // Update statistics - 使用作用域限制锁的生命周期
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_allocated += new_size;
            stats.total_deallocated += old_size;
            stats.reallocation_count += 1;
        }
        
        Ok(new_ptr)
    }
    
    fn get_stats(&self) -> MemoryStats {
        // 克隆统计数据而不是返回引用，避免生命周期问题
        self.stats.lock().unwrap().clone()
    }
}

/// Memory statistics
#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    /// Total bytes allocated
    pub total_allocated: usize,
    /// Total bytes deallocated
    pub total_deallocated: usize,
    /// Number of allocations
    pub allocation_count: usize,
    /// Number of deallocations
    pub deallocation_count: usize,
    /// Number of reallocations
    pub reallocation_count: usize,
}

impl MemoryStats {
    /// Get current memory usage
    pub fn current_usage(&self) -> usize {
        self.total_allocated.saturating_sub(self.total_deallocated)
    }
}

/// Memory configuration
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    /// Total memory limit
    pub memory_limit: usize,
    /// Enable memory protection
    pub enable_protection: bool,
    /// Enable memory isolation
    pub enable_isolation: bool,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            memory_limit: usize::MAX,
            enable_protection: true,
            enable_isolation: true,
        }
    }
}

/// Memory status
#[derive(Debug, Clone)]
pub struct MemoryStatus {
    /// Memory statistics
    pub stats: MemoryStats,
    /// Number of memory regions
    pub region_count: usize,
}

// 全局内存管理器 - 修改为pub而非pub(crate)，使其可以被外部模块访问
lazy_static! {
    pub static ref MEMORY_MANAGER: Mutex<MemoryManager> = Mutex::new(MemoryManager::new());
}

// Next region base address
static NEXT_REGION_BASE: AtomicUsize = AtomicUsize::new(0x1000_0000);

/// Memory manager
#[derive(Debug)]
pub struct MemoryManager {
    /// Memory allocator
    allocator: Box<dyn SafeMemoryAllocator>,
    /// Memory regions
    regions: HashMap<usize, MemoryRegion>,
    /// Memory configuration
    config: MemoryConfig,
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new() -> Self {
        Self {
            allocator: Box::new(DefaultAllocator::new()),
            regions: HashMap::new(),
            config: MemoryConfig::default(),
        }
    }
    
    /// Set memory configuration
    pub fn set_config(&mut self, config: MemoryConfig) {
        self.config = config;
    }
    
    /// Register a memory region
    pub fn register_region(&mut self, region: MemoryRegion) -> Result<(), MemoryError> {
        if self.regions.contains_key(&region.base) {
            return Err(MemoryError::RegionAlreadyExists);
        }
        
        self.regions.insert(region.base, region);
        Ok(())
    }
    
    /// Unregister a memory region
    pub fn unregister_region(&mut self, base: usize) -> Result<MemoryRegion, MemoryError> {
        self.regions.remove(&base).ok_or(MemoryError::RegionNotFound)
    }
    
    /// Find a memory region containing the address
    pub fn find_region(&self, addr: usize) -> Option<&MemoryRegion> {
        self.regions.values().find(|region| {
            addr >= region.base && addr < region.base + region.size
        })
    }
    
    /// Set memory protection
    pub fn set_protection(&mut self, addr: usize, size: usize, protection: ProtectionFlags) -> Result<(), MemoryError> {
        if !self.config.enable_protection {
            return Ok(());
        }
        
        // 先查找区域，避免后续重复查找
        let region = self.find_region(addr).ok_or(MemoryError::InvalidAddress)?;
        
        // 检查区域范围
        if addr + size > region.base + region.size {
            return Err(MemoryError::InvalidAddress);
        }
        
        // 创建新区域并更新保护标志
        let new_region = MemoryRegion {
            base: region.base,
            size: region.size,
            protection,
            name: region.name.clone(),
        };
        
        // 替换旧区域
        self.regions.insert(region.base, new_region);
        
        Ok(())
    }
    
    /// Create an isolated memory region
    pub fn create_isolated_region(&mut self, size: usize, protection: ProtectionFlags, name: Option<String>) 
        -> Result<IsolatedMemoryRegion, MemoryError> {
        if !self.config.enable_isolation {
            return Err(MemoryError::General("Memory isolation is disabled"));
        }
        
        // 创建隔离区域
        let region = IsolatedMemoryRegion::new(size, protection, name.clone())?;
        
        // 注册区域
        self.register_region(region.region.clone())?;
        
        Ok(region)
    }
    
    /// Get memory status
    pub fn get_status(&self) -> MemoryStatus {
        // 获取统计数据的克隆，避免生命周期问题
        let stats = self.allocator.get_stats();
        
        MemoryStatus {
            stats,
            region_count: self.regions.len(),
        }
    }
}

/// Initialize the memory management subsystem
pub fn init() -> Result<(), MemoryError> {
    // Already initialized by lazy_static
    Ok(())
}

/// Set memory configuration
pub fn set_config(config: MemoryConfig) -> Result<(), MemoryError> {
    // 使用作用域限制锁的生命周期
    {
        let mut manager = MEMORY_MANAGER.lock().unwrap();
        manager.set_config(config);
    }
    Ok(())
}

/// Allocate memory
pub fn allocate(size: usize, align: usize) -> Result<*mut u8, MemoryError> {
    // 使用作用域限制锁的生命周期
    let result = {
        let manager = MEMORY_MANAGER.lock().unwrap();
        manager.allocator.allocate(size, align)
    };
    result
}

/// Deallocate memory
pub fn deallocate(ptr: *mut u8, size: usize, align: usize) {
    // 使用作用域限制锁的生命周期
    {
        let manager = MEMORY_MANAGER.lock().unwrap();
        manager.allocator.deallocate(ptr, size, align);
    }
}

/// Reallocate memory
pub fn reallocate(ptr: *mut u8, old_size: usize, new_size: usize, align: usize) 
    -> Result<*mut u8, MemoryError> {
    // 使用作用域限制锁的生命周期
    let result = {
        let manager = MEMORY_MANAGER.lock().unwrap();
        manager.allocator.reallocate(ptr, old_size, new_size, align)
    };
    result
}

/// Set memory protection
pub fn set_protection(addr: usize, size: usize, protection: ProtectionFlags) -> Result<(), MemoryError> {
    // 使用作用域限制锁的生命周期
    let result = {
        let mut manager = MEMORY_MANAGER.lock().unwrap();
        manager.set_protection(addr, size, protection)
    };
    result
}

/// Create an isolated memory region
pub fn create_isolated_region(size: usize, protection: ProtectionFlags, name: Option<String>) 
    -> Result<IsolatedMemoryRegion, MemoryError> {
    // 使用作用域限制锁的生命周期
    let result = {
        let mut manager = MEMORY_MANAGER.lock().unwrap();
        manager.create_isolated_region(size, protection, name)
    };
    result
}

/// Get memory status
pub fn get_status() -> Result<MemoryStatus, MemoryError> {
    // 使用作用域限制锁的生命周期，并克隆结果避免生命周期问题
    let status = {
        let manager = MEMORY_MANAGER.lock().unwrap();
        manager.get_status()
    };
    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_allocation() {
        // Initialize memory subsystem
        assert!(init().is_ok());
        
        // Allocate memory
        let ptr = allocate(1024, 8).unwrap();
        assert!(!ptr.is_null());
        
        // Reallocate memory
        let new_ptr = reallocate(ptr, 1024, 2048, 8).unwrap();
        assert!(!new_ptr.is_null());
        
        // Deallocate memory
        deallocate(new_ptr, 2048, 8);
        
        // Get memory status
        let status = get_status().unwrap();
        assert_eq!(status.stats.allocation_count, 1);
        assert_eq!(status.stats.reallocation_count, 1);
        assert_eq!(status.stats.deallocation_count, 1);
    }
    
    #[test]
    fn test_memory_protection() {
        // Initialize memory subsystem
        assert!(init().is_ok());
        
        // 确保内存隔离功能启用
        let config = MemoryConfig {
            memory_limit: usize::MAX,
            enable_protection: true,
            enable_isolation: true,
        };
        assert!(set_config(config).is_ok());
        
        // Create an isolated memory region
        let region = create_isolated_region(
            4096,
            ProtectionFlags::read_write(),
            Some("test_region".to_string()),
        ).unwrap();
        
        // Set memory protection
        assert!(set_protection(
            region.region.base,
            region.region.size,
            ProtectionFlags::read_only(),
        ).is_ok());
(Content truncated due to size limit. Use line ranges to read in chunks)