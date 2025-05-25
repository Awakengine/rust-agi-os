// AGI操作系统 - 进程管理模块
// 此文件实现进程创建、调度和管理

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// 进程错误类型
#[derive(Debug)]
pub enum ProcessError {
    CreationFailed(String),
    NotFound(String),
    AlreadyExists(String),
    SchedulingError(String),
    ExecutionError(String),
    TerminationError(String),
    LimitExceeded(String),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessError::CreationFailed(msg) => write!(f, "进程创建失败: {}", msg),
            ProcessError::NotFound(msg) => write!(f, "进程未找到: {}", msg),
            ProcessError::AlreadyExists(msg) => write!(f, "进程已存在: {}", msg),
            ProcessError::SchedulingError(msg) => write!(f, "调度错误: {}", msg),
            ProcessError::ExecutionError(msg) => write!(f, "执行错误: {}", msg),
            ProcessError::TerminationError(msg) => write!(f, "终止错误: {}", msg),
            ProcessError::LimitExceeded(msg) => write!(f, "超出限制: {}", msg),
        }
    }
}

impl Error for ProcessError {}

// 进程状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    Created,
    Ready,
    Running,
    Blocked,
    Terminated,
}

// 进程优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessPriority {
    Low,
    Normal,
    High,
    Critical,
}

// 进程定义
pub struct Process {
    id: String,
    name: String,
    status: ProcessStatus,
    priority: ProcessPriority,
    creation_time: Instant,
    execution_time: Duration,
    memory_usage: usize,
    cpu_usage: f32,
}

impl Process {
    pub fn new(id: String, name: String, priority: ProcessPriority) -> Self {
        Process {
            id,
            name,
            status: ProcessStatus::Created,
            priority,
            creation_time: Instant::now(),
            execution_time: Duration::from_secs(0),
            memory_usage: 0,
            cpu_usage: 0.0,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_status(&self) -> ProcessStatus {
        self.status
    }

    pub fn set_status(&mut self, status: ProcessStatus) {
        self.status = status;
    }

    pub fn get_priority(&self) -> ProcessPriority {
        self.priority
    }

    pub fn set_priority(&mut self, priority: ProcessPriority) {
        self.priority = priority;
    }

    pub fn get_creation_time(&self) -> Instant {
        self.creation_time
    }

    pub fn get_execution_time(&self) -> Duration {
        self.execution_time
    }

    pub fn update_execution_time(&mut self, duration: Duration) {
        self.execution_time += duration;
    }

    pub fn get_memory_usage(&self) -> usize {
        self.memory_usage
    }

    pub fn set_memory_usage(&mut self, usage: usize) {
        self.memory_usage = usage;
    }

    pub fn get_cpu_usage(&self) -> f32 {
        self.cpu_usage
    }

    pub fn set_cpu_usage(&mut self, usage: f32) {
        self.cpu_usage = usage;
    }
}

// 进程调度器特性
pub trait ProcessScheduler: Send + Sync {
    fn name(&self) -> &str;
    fn add_process(&mut self, process: Arc<Mutex<Process>>) -> Result<(), ProcessError>;
    fn remove_process(&mut self, id: &str) -> Result<Arc<Mutex<Process>>, ProcessError>;
    fn next_process(&mut self) -> Option<Arc<Mutex<Process>>>;
    fn get_process(&self, id: &str) -> Option<Arc<Mutex<Process>>>;
    fn get_all_processes(&self) -> Vec<Arc<Mutex<Process>>>;
    fn get_process_count(&self) -> usize;
}

// 轮询调度器实现
pub struct RoundRobinScheduler {
    name: String,
    processes: Vec<Arc<Mutex<Process>>>,
    current_index: usize,
}

impl RoundRobinScheduler {
    pub fn new(name: String) -> Self {
        RoundRobinScheduler {
            name,
            processes: Vec::new(),
            current_index: 0,
        }
    }
}

impl ProcessScheduler for RoundRobinScheduler {
    fn name(&self) -> &str {
        &self.name
    }

    fn add_process(&mut self, process: Arc<Mutex<Process>>) -> Result<(), ProcessError> {
        let id = process.lock().unwrap().get_id().to_string();
        
        // 检查进程是否已存在
        if self.get_process(&id).is_some() {
            return Err(ProcessError::AlreadyExists(format!("进程 ID '{}' 已存在", id)));
        }
        
        self.processes.push(process);
        Ok(())
    }

    fn remove_process(&mut self, id: &str) -> Result<Arc<Mutex<Process>>, ProcessError> {
        let position = self.processes.iter().position(|p| p.lock().unwrap().get_id() == id);
        
        match position {
            Some(index) => {
                let process = self.processes.remove(index);
                
                // 调整当前索引
                if index <= self.current_index && self.current_index > 0 {
                    self.current_index -= 1;
                }
                
                Ok(process)
            },
            None => Err(ProcessError::NotFound(format!("进程 ID '{}' 不存在", id))),
        }
    }

    fn next_process(&mut self) -> Option<Arc<Mutex<Process>>> {
        if self.processes.is_empty() {
            return None;
        }
        
        let process = self.processes[self.current_index].clone();
        
        // 更新索引到下一个进程
        self.current_index = (self.current_index + 1) % self.processes.len();
        
        Some(process)
    }

    fn get_process(&self, id: &str) -> Option<Arc<Mutex<Process>>> {
        self.processes.iter()
            .find(|p| p.lock().unwrap().get_id() == id)
            .cloned()
    }

    fn get_all_processes(&self) -> Vec<Arc<Mutex<Process>>> {
        self.processes.clone()
    }

    fn get_process_count(&self) -> usize {
        self.processes.len()
    }
}

// 优先级调度器实现
pub struct PriorityScheduler {
    name: String,
    processes: HashMap<String, Arc<Mutex<Process>>>,
}

impl PriorityScheduler {
    pub fn new(name: String) -> Self {
        PriorityScheduler {
            name,
            processes: HashMap::new(),
        }
    }
}

impl ProcessScheduler for PriorityScheduler {
    fn name(&self) -> &str {
        &self.name
    }

    fn add_process(&mut self, process: Arc<Mutex<Process>>) -> Result<(), ProcessError> {
        let id = process.lock().unwrap().get_id().to_string();
        
        // 检查进程是否已存在
        if self.processes.contains_key(&id) {
            return Err(ProcessError::AlreadyExists(format!("进程 ID '{}' 已存在", id)));
        }
        
        self.processes.insert(id, process);
        Ok(())
    }

    fn remove_process(&mut self, id: &str) -> Result<Arc<Mutex<Process>>, ProcessError> {
        if let Some(process) = self.processes.remove(id) {
            Ok(process)
        } else {
            Err(ProcessError::NotFound(format!("进程 ID '{}' 不存在", id)))
        }
    }

    fn next_process(&mut self) -> Option<Arc<Mutex<Process>>> {
        if self.processes.is_empty() {
            return None;
        }
        
        // 找出优先级最高的就绪进程
        let mut highest_priority = None;
        let mut highest_priority_id = None;
        
        for (id, process) in &self.processes {
            let process_guard = process.lock().unwrap();
            
            if process_guard.get_status() == ProcessStatus::Ready {
                let priority = process_guard.get_priority();
                
                if highest_priority.is_none() || priority > highest_priority.unwrap() {
                    highest_priority = Some(priority);
                    highest_priority_id = Some(id.clone());
                }
            }
        }
        
        highest_priority_id.and_then(|id| self.processes.get(&id).cloned())
    }

    fn get_process(&self, id: &str) -> Option<Arc<Mutex<Process>>> {
        self.processes.get(id).cloned()
    }

    fn get_all_processes(&self) -> Vec<Arc<Mutex<Process>>> {
        self.processes.values().cloned().collect()
    }

    fn get_process_count(&self) -> usize {
        self.processes.len()
    }
}

// 进程管理器
pub struct ProcessManager {
    process_limit: usize,
    scheduler: Box<dyn ProcessScheduler>,
}

impl ProcessManager {
    pub fn new(process_limit: usize, scheduler_policy: String) -> Result<Self, ProcessError> {
        let scheduler: Box<dyn ProcessScheduler> = match scheduler_policy.as_str() {
            "round_robin" => Box::new(RoundRobinScheduler::new("RoundRobin".to_string())),
            "priority" => Box::new(PriorityScheduler::new("Priority".to_string())),
            _ => return Err(ProcessError::SchedulingError(format!("未知的调度策略: {}", scheduler_policy))),
        };
        
        Ok(ProcessManager {
            process_limit,
            scheduler,
        })
    }

    pub fn create_process(&mut self, id: String, name: String, priority: ProcessPriority) -> Result<Arc<Mutex<Process>>, ProcessError> {
        if self.scheduler.get_process_count() >= self.process_limit {
            return Err(ProcessError::LimitExceeded(format!("已达到进程数量上限: {}", self.process_limit)));
        }
        
        let process = Arc::new(Mutex::new(Process::new(id, name, priority)));
        self.scheduler.add_process(process.clone())?;
        
        Ok(process)
    }

    pub fn terminate_process(&mut self, id: &str) -> Result<(), ProcessError> {
        let process = self.scheduler.remove_process(id)?;
        let mut process_guard = process.lock().unwrap();
        process_guard.set_status(ProcessStatus::Terminated);
        
        Ok(())
    }

    pub fn schedule_next(&mut self) -> Option<Arc<Mutex<Process>>> {
        let process = self.scheduler.next_process()?;
        
        {
            let mut process_guard = process.lock().unwrap();
            if process_guard.get_status() == ProcessStatus::Ready {
                process_guard.set_status(ProcessStatus::Running);
            }
        }
        
        Some(process)
    }

    pub fn get_process(&self, id: &str) -> Option<Arc<Mutex<Process>>> {
        self.scheduler.get_process(id)
    }

    pub fn get_all_processes(&self) -> Vec<Arc<Mutex<Process>>> {
        self.scheduler.get_all_processes()
    }

    pub fn get_process_count(&self) -> usize {
        self.scheduler.get_process_count()
    }

    pub fn get_process_limit(&self) -> usize {
        self.process_limit
    }

    pub fn get_scheduler_name(&self) -> &str {
        self.scheduler.name()
    }

    pub fn shutdown(&mut self) -> Result<(), ProcessError> {
        // 终止所有进程
        let processes = self.scheduler.get_all_processes();
        
        for process in processes {
            let id = process.lock().unwrap().get_id().to_string();
            match self.terminate_process(&id) {
                Ok(_) => {},
                Err(e) => {
                    println!("终止进程 {} 时出错: {}", id, e);
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_process_creation() {
        let process = Process::new("p1".to_string(), "Test Process".to_string(), ProcessPriority::Normal);
        
        assert_eq!(process.get_id(), "p1");
        assert_eq!(process.get_name(), "Test Process");
        assert_eq!(process.get_status(), ProcessStatus::Created);
        assert_eq!(process.get_priority(), ProcessPriority::Normal);
        assert_eq!(process.get_memory_usage(), 0);
        assert_eq!(process.get_cpu_usage(), 0.0);
    }

    #[test]
    fn test_process_status_update() {
        let mut process = Process::new("p1".to_string(), "Test Process".to_string(), ProcessPriority::Normal);
        
        process.set_status(ProcessStatus::Ready);
        assert_eq!(process.get_status(), ProcessStatus::Ready);
        
        process.set_status(ProcessStatus::Running);
        assert_eq!(process.get_status(), ProcessStatus::Running);
        
        process.set_status(ProcessStatus::Blocked);
        assert_eq!(process.get_status(), ProcessStatus::Blocked);
        
        process.set_status(ProcessStatus::Terminated);
        assert_eq!(process.get_status(), ProcessStatus::Terminated);
    }

    #[test]
    fn test_round_robin_scheduler() {
        let mut scheduler = RoundRobinScheduler::new("TestRR".to_string());
        
        let p1 = Arc::new(Mutex::new(Process::new("p1".to_string(), "Process 1".to_string(), ProcessPriority::Normal)));
        let p2 = Arc::new(Mutex::new(Process::new("p2".to_string(), "Process 2".to_string(), ProcessPriority::Normal)));
        let p3 = Arc::new(Mutex::new(Process::new("p3".to_string(), "Process 3".to_string(), ProcessPriority::Normal)));
        
        scheduler.add_process(p1.clone()).unwrap();
        scheduler.add_process(p2.clone()).unwrap();
        scheduler.add_process(p3.clone()).unwrap();
        
        assert_eq!(scheduler.get_process_count(), 3);
        
        // 测试轮询调度
        let next1 = scheduler.next_process().unwrap();
        assert_eq!(next1.lock().unwrap().get_id(), "p1");
        
        let next2 = scheduler.next_process().unwrap();
        assert_eq!(next2.lock().unwrap().get_id(), "p2");
        
        let next3 = scheduler.next_process().unwrap();
        assert_eq!(next3.lock().unwrap().get_id(), "p3");
        
        let next4 = scheduler.next_process().unwrap();
        assert_eq!(next4.lock().unwrap().get_id(), "p1");
        
        // 测试移除进程
        scheduler.remove_process("p2").unwrap();
        assert_eq!(scheduler.get_process_count(), 2);
        
        let next5 = scheduler.next_process().unwrap();
        assert_eq!(next5.lock().unwrap().get_id(), "p3");
        
        let next6 = scheduler.next_process().unwrap();
        assert_eq!(next6.lock().unwrap().get_id(), "p1");
    }

    #[test]
    fn test_priority_scheduler() {
        let mut scheduler = PriorityScheduler::new("TestPriority".to_string());
        
        let p1 = Arc::new(Mutex::new(Process::new("p1".to_string(), "Process 1".to_string(), ProcessPriority::Normal)));
        let p2 = Arc::new(Mutex::new(Process::new("p2".to_string(), "Process 2".to_string(), ProcessPriority::High)));
        let p3 = Arc::new(Mutex::new(Process::new("p3".to_string(), "Process 3".to_string(), ProcessPriority::Low)));
        
        p1.lock().unwrap().set_status(ProcessStatus::Ready);
        p2.lock().unwrap().set_status(ProcessStatus::Ready);
        p3.lock().unwrap().set_status(ProcessStatus::Ready);
        
        scheduler.add_process(p1.clone()).unwrap();
        scheduler.add_process(p2.clone()).unwrap();
        scheduler.add_process(p3.clone()).unwrap();
        
        assert_eq!(scheduler.get_process_count(), 3);
        
        // 测试优先级调度
        let next1 = scheduler.next_process().unwrap();
        assert_eq!(next1.lock().unwrap().get_id(), "p2"); // 高优先级
        
        // 将p2设置为非就绪状态
        next1.lock().unwrap().set_status(ProcessStatus::Running);
        
        let next2 = scheduler.next_process().unwrap();
        assert_eq!(next2.lock().unwrap().get_id(), "p1"); // 普通优先级
        
        // 将p1设置为非就绪状态
        next2.lock().unwrap().set_status(ProcessStatus::Running);
        
        let next3 = scheduler.next_process().unwrap();
        assert_eq!(next3.lock().unwrap().get_id(), "p3"); // 低优先级
    }

    #[test]
    fn test_process_manager() {
        let mut manager = ProcessManager::new(10, "round_robin".to_string()).unwrap();
        
        // 创建进程
        let p1 = manager.create_process("p1".to_string(), "Process 1".to_string(), ProcessPriority::Normal).unwrap();
        let p2 = manager.create_process("p2".to_string(), "Process 2".to_string(), ProcessPriority::High).unwrap();
        
        p1.lock().unwrap().set_status(ProcessStatus::Ready);
        p2.lock().unwrap().set_status(ProcessStatus::Ready);
        
        assert_eq!(manager.get_process_count(), 2);
        
        // 测试调度
        let next1 = manager.schedule_next().unwrap();
        assert_eq!(next1.l
(Content truncated due to size limit. Use line ranges to read in chunks)