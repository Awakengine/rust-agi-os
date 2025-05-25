// 神经符号模块 - 神经网络引擎
pub mod neural_network;

// 导出神经网络引擎初始化函数
pub use neural_network::initialize;

// 导出神经网络引擎启动函数
pub use neural_network::start;

// 导出神经网络引擎的stop函数
pub use neural_network::stop;

// 神经符号模块 - 符号推理系统
pub mod symbolic_reasoning;

// 神经符号模块 - 学习系统
pub mod learning_system;