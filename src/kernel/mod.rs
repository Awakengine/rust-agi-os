mod memory;
mod process;

// 使用具体的模块导出，避免glob导出冲突
pub use memory::{Memory, MemoryManager, MemoryError};
pub use process::{Process, ProcessManager, ProcessError};

// 导出特定函数，避免冲突
pub use memory::init as memory_init;
pub use memory::start as memory_start;
pub use memory::stop as memory_stop;

pub use process::init as process_init;
pub use process::start as process_start;
pub use process::stop as process_stop;
