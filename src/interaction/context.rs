// AGI操作系统 - 上下文管理模块
// 此文件实现上下文管理功能

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::error::Error;
use std::fmt;
use std::time::{Duration, SystemTime};

/// 上下文管理错误类型
#[derive(Debug)]
pub enum ContextError {
    InitializationError(String),
    StorageError(String),
    RetrievalError(String),
    InvalidContextError(String),
    CapacityError(String),
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContextError::InitializationError(msg) => write!(f, "初始化错误: {}", msg),
            ContextError::StorageError(msg) => write!(f, "存储错误: {}", msg),
            ContextError::RetrievalError(msg) => write!(f, "检索错误: {}", msg),
            ContextError::InvalidContextError(msg) => write!(f, "无效上下文: {}", msg),
            ContextError::CapacityError(msg) => write!(f, "容量错误: {}", msg),
        }
    }
}

impl Error for ContextError {}

/// 上下文类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextType {
    Conversation,
    Task,
    User,
    System,
    Environment,
    Session,
    Custom(String),
}

/// 上下文优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContextPriority {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

/// 记忆类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MemoryType {
    ShortTerm,
    WorkingMemory,
    LongTerm,
    Episodic,
    Semantic,
    Procedural,
}

/// 上下文项
#[derive(Debug, Clone)]
pub struct ContextItem {
    id: String,
    content: String,
    context_type: ContextType,
    priority: ContextPriority,
    memory_type: MemoryType,
    timestamp: SystemTime,
    metadata: HashMap<String, String>,
    ttl: Option<Duration>,
}

impl ContextItem {
    pub fn new(
        id: &str,
        content: &str,
        context_type: ContextType,
        priority: ContextPriority,
        memory_type: MemoryType,
    ) -> Self {
        ContextItem {
            id: id.to_string(),
            content: content.to_string(),
            context_type,
            priority,
            memory_type,
            timestamp: SystemTime::now(),
            metadata: HashMap::new(),
            ttl: None,
        }
    }

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_context_type(&self) -> &ContextType {
        &self.context_type
    }

    pub fn get_priority(&self) -> ContextPriority {
        self.priority
    }

    pub fn get_memory_type(&self) -> &MemoryType {
        &self.memory_type
    }

    pub fn get_timestamp(&self) -> SystemTime {
        self.timestamp
    }

    pub fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    pub fn get_ttl(&self) -> Option<Duration> {
        self.ttl
    }

    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            if let Ok(elapsed) = self.timestamp.elapsed() {
                return elapsed > ttl;
            }
        }
        false
    }

    pub fn update_content(&mut self, content: &str) {
        self.content = content.to_string();
        self.timestamp = SystemTime::now();
    }

    pub fn update_priority(&mut self, priority: ContextPriority) {
        self.priority = priority;
    }

    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
}

/// 上下文窗口
#[derive(Debug)]
pub struct ContextWindow {
    max_items: usize,
    items: VecDeque<ContextItem>,
}

impl ContextWindow {
    pub fn new(max_items: usize) -> Self {
        ContextWindow {
            max_items,
            items: VecDeque::with_capacity(max_items),
        }
    }

    pub fn add_item(&mut self, item: ContextItem) {
        if self.items.len() >= self.max_items {
            self.items.pop_front();
        }
        self.items.push_back(item);
    }

    pub fn get_items(&self) -> &VecDeque<ContextItem> {
        &self.items
    }

    pub fn get_items_mut(&mut self) -> &mut VecDeque<ContextItem> {
        &mut self.items
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn capacity(&self) -> usize {
        self.max_items
    }

    pub fn is_full(&self) -> bool {
        self.items.len() >= self.max_items
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// 上下文配置
#[derive(Debug, Clone)]
pub struct ContextConfig {
    id: String,
    max_items: usize,
    default_ttl: Option<Duration>,
    auto_cleanup: bool,
    parameters: HashMap<String, String>,
}

impl ContextConfig {
    pub fn new(id: &str) -> Self {
        ContextConfig {
            id: id.to_string(),
            max_items: 1000,
            default_ttl: None,
            auto_cleanup: true,
            parameters: HashMap::new(),
        }
    }

    pub fn with_max_items(mut self, max_items: usize) -> Self {
        self.max_items = max_items;
        self
    }

    pub fn with_default_ttl(mut self, ttl: Duration) -> Self {
        self.default_ttl = Some(ttl);
        self
    }

    pub fn with_auto_cleanup(mut self, auto_cleanup: bool) -> Self {
        self.auto_cleanup = auto_cleanup;
        self
    }

    pub fn with_parameter(mut self, key: &str, value: &str) -> Self {
        self.parameters.insert(key.to_string(), value.to_string());
        self
    }
}

/// 上下文管理器
pub struct ContextManager {
    config: ContextConfig,
    items: HashMap<String, ContextItem>,
    windows: HashMap<String, ContextWindow>,
    type_indices: HashMap<ContextType, Vec<String>>,
    memory_indices: HashMap<MemoryType, Vec<String>>,
}

impl ContextManager {
    pub fn new(config: ContextConfig) -> Result<Self, ContextError> {
        let manager = ContextManager {
            config,
            items: HashMap::new(),
            windows: HashMap::new(),
            type_indices: HashMap::new(),
            memory_indices: HashMap::new(),
        };
        
        Ok(manager)
    }

    pub fn add_item(&mut self, item: ContextItem) -> Result<(), ContextError> {
        let item_id = item.get_id().to_string();
        let context_type = item.get_context_type().clone();
        let memory_type = item.get_memory_type().clone();
        
        // 存储项目
        self.items.insert(item_id.clone(), item);
        
        // 更新类型索引
        let type_items = self.type_indices
            .entry(context_type)
            .or_insert_with(Vec::new);
        type_items.push(item_id.clone());
        
        // 更新记忆类型索引
        let memory_items = self.memory_indices
            .entry(memory_type)
            .or_insert_with(Vec::new);
        memory_items.push(item_id);
        
        // 如果配置了自动清理，则检查过期项目
        if self.config.auto_cleanup {
            self.cleanup_expired()?;
        }
        
        Ok(())
    }

    pub fn get_item(&self, id: &str) -> Result<&ContextItem, ContextError> {
        self.items.get(id).ok_or_else(|| {
            ContextError::RetrievalError(format!("上下文项 '{}' 不存在", id))
        })
    }

    pub fn update_item(&mut self, id: &str, content: &str) -> Result<(), ContextError> {
        let item = self.items.get_mut(id).ok_or_else(|| {
            ContextError::InvalidContextError(format!("上下文项 '{}' 不存在", id))
        })?;
        
        item.update_content(content);
        Ok(())
    }

    pub fn remove_item(&mut self, id: &str) -> Result<ContextItem, ContextError> {
        let item = self.items.remove(id).ok_or_else(|| {
            ContextError::InvalidContextError(format!("上下文项 '{}' 不存在", id))
        })?;
        
        // 从类型索引中移除
        if let Some(type_items) = self.type_indices.get_mut(item.get_context_type()) {
            type_items.retain(|item_id| item_id != id);
        }
        
        // 从记忆类型索引中移除
        if let Some(memory_items) = self.memory_indices.get_mut(item.get_memory_type()) {
            memory_items.retain(|item_id| item_id != id);
        }
        
        Ok(item)
    }

    pub fn create_window(&mut self, id: &str, max_items: usize) -> Result<(), ContextError> {
        if self.windows.contains_key(id) {
            return Err(ContextError::InitializationError(
                format!("上下文窗口 '{}' 已存在", id)
            ));
        }
        
        let window = ContextWindow::new(max_items);
        self.windows.insert(id.to_string(), window);
        
        Ok(())
    }

    pub fn add_to_window(&mut self, window_id: &str, item_id: &str) -> Result<(), ContextError> {
        let window = self.windows.get_mut(window_id).ok_or_else(|| {
            ContextError::InvalidContextError(format!("上下文窗口 '{}' 不存在", window_id))
        })?;
        
        let item = self.items.get(item_id).ok_or_else(|| {
            ContextError::InvalidContextError(format!("上下文项 '{}' 不存在", item_id))
        })?;
        
        window.add_item(item.clone());
        Ok(())
    }

    pub fn get_window(&self, id: &str) -> Result<&ContextWindow, ContextError> {
        self.windows.get(id).ok_or_else(|| {
            ContextError::RetrievalError(format!("上下文窗口 '{}' 不存在", id))
        })
    }

    pub fn get_items_by_type(&self, context_type: &ContextType) -> Vec<&ContextItem> {
        match self.type_indices.get(context_type) {
            Some(ids) => {
                ids.iter()
                    .filter_map(|id| self.items.get(id))
                    .collect()
            },
            None => Vec::new(),
        }
    }

    pub fn get_items_by_memory_type(&self, memory_type: &MemoryType) -> Vec<&ContextItem> {
        match self.memory_indices.get(memory_type) {
            Some(ids) => {
                ids.iter()
                    .filter_map(|id| self.items.get(id))
                    .collect()
            },
            None => Vec::new(),
        }
    }

    pub fn search_items(&self, query: &str) -> Vec<&ContextItem> {
        self.items.values()
            .filter(|item| item.get_content().contains(query))
            .collect()
    }

    pub fn cleanup_expired(&mut self) -> Result<usize, ContextError> {
        let mut expired_ids = Vec::new();
        
        // 找出所有过期项目
        for (id, item) in &self.items {
            if item.is_expired() {
                expired_ids.push(id.clone());
            }
        }
        
        // 移除过期项目
        let count = expired_ids.len();
        for id in expired_ids {
            self.remove_item(&id)?;
        }
        
        Ok(count)
    }

    pub fn get_config(&self) -> &ContextConfig {
        &self.config
    }

    pub fn get_item_count(&self) -> usize {
        self.items.len()
    }

    pub fn get_window_count(&self) -> usize {
        self.windows.len()
    }

    pub fn get_all_items(&self) -> Vec<&ContextItem> {
        self.items.values().collect()
    }

    pub fn get_all_windows(&self) -> Vec<(&String, &ContextWindow)> {
        self.windows.iter().collect()
    }
}
