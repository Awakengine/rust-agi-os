pub mod window;
pub mod render;
pub mod window_system;
pub mod theme;
pub mod desktop;
pub mod window_manager;
pub mod applications;
pub mod high_dpi;
pub mod input_method;
pub mod keyboard_input;
pub mod keyboard_integration;
pub mod mouse_input;
pub mod mouse_integration;
pub mod testing;
pub mod deployment;
pub mod main;
pub mod vm_validation;
pub mod feedback;
pub mod iced_window_system;
pub mod high_resolution_window;

// 导出特定函数，避免冲突
pub use window::init as window_init;
pub use window::start as window_start;
pub use window::stop as window_stop;

pub use render::init as render_init;
pub use render::start as render_start;
pub use render::stop as render_stop;

pub use window_system::init as window_system_init;
pub use window_system::start as window_system_start;
pub use window_system::stop as window_system_stop;

pub use theme::init as theme_init;
pub use theme::start as theme_start;
pub use theme::stop as theme_stop;

pub use desktop::init as desktop_init;
pub use desktop::start as desktop_start;
pub use desktop::stop as desktop_stop;

pub use window_manager::init as window_manager_init;
pub use window_manager::start as window_manager_start;
pub use window_manager::stop as window_manager_stop;

pub use applications::init as applications_init;
pub use applications::start as applications_start;
pub use applications::stop as applications_stop;

pub use high_dpi::init as high_dpi_init;
pub use high_dpi::start as high_dpi_start;
pub use high_dpi::stop as high_dpi_stop;

pub use input_method::init as input_method_init;
pub use input_method::start as input_method_start;
pub use input_method::stop as input_method_stop;
