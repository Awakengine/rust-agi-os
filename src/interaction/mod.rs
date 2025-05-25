// 交互模块 - 多模态接口
pub mod interface;

// 导出交互接口初始化函数
pub use interface::initialize;

// 导出交互模块启动函数
pub use interface::start;

// 导出多模态接口的stop函数
pub use interface::stop;

// 交互模块 - 自然语言处理
pub mod language;

// 交互模块 - 上下文管理
pub mod context;