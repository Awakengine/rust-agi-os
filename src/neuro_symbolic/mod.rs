mod neural;
mod symbolic;
mod knowledge;
mod learning;
mod integration;

// 使用具体的模块导出，避免glob导出冲突
pub use neural::{NeuralNetwork, NeuralNetworkManager, NeuralError};
pub use symbolic::{Symbol, SymbolicSystem, SymbolicError};
pub use knowledge::{Knowledge, KnowledgeBase, KnowledgeError};
pub use learning::{Learning, LearningSystem, LearningError};
pub use integration::{Integration, IntegrationSystem, IntegrationError};

// 导出特定函数，避免冲突
pub use neural::init as neural_init;
pub use neural::start as neural_start;
pub use neural::stop as neural_stop;

pub use symbolic::init as symbolic_init;
pub use symbolic::start as symbolic_start;
pub use symbolic::stop as symbolic_stop;

pub use knowledge::init as knowledge_init;
pub use knowledge::start as knowledge_start;
pub use knowledge::stop as knowledge_stop;

pub use learning::init as learning_init;
pub use learning::start as learning_start;
pub use learning::stop as learning_stop;

pub use integration::init as integration_init;
pub use integration::start as integration_start;
pub use integration::stop as integration_stop;
