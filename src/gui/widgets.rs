use std::sync::{Arc, Mutex};
use crate::gui::theme::ThemeManager;

/// 基础控件特征
pub trait Widget: Send + Sync {
    /// 绘制控件
    fn draw(&self, renderer: &mut dyn Renderer);
    
    /// 更新控件
    fn update(&mut self, delta_time: f32);
    
    /// 处理点击事件
    fn handle_click(&mut self, x: f32, y: f32) -> bool;
    
    /// 获取边界
    fn bounds(&self) -> Rect;
    
    /// 设置边界
    fn set_bounds(&mut self, bounds: Rect);
    
    /// 是否可见
    fn is_visible(&self) -> bool;
    
    /// 设置可见性
    fn set_visible(&mut self, visible: bool);
}

/// 渲染器特征
pub trait Renderer {
    /// 填充矩形
    fn fill_rect(&mut self, rect: Rect, color: (u8, u8, u8, u8));
    
    /// 绘制图像
    fn draw_image(&mut self, image_path: &str, bounds: Rect);
    
    /// 绘制文本
    fn draw_text(&mut self, text: &str, position: (f32, f32), color: (u8, u8, u8, u8), font_size: f32);
}

/// 矩形区域
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    /// 创建新的矩形
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    /// 检查点是否在矩形内
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
    
    /// 获取中心点
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

/// 按钮控件
pub struct Button {
    bounds: Rect,
    label: String,
    background_color: (u8, u8, u8, u8),
    text_color: (u8, u8, u8, u8),
    hover_color: (u8, u8, u8, u8),
    pressed_color: (u8, u8, u8, u8),
    is_hover: bool,
    is_pressed: bool,
    is_visible: bool,
    on_click: Option<Box<dyn Fn() + Send + Sync>>,
    theme: Arc<Mutex<ThemeManager>>,
}

impl Button {
    /// 创建新按钮
    pub fn new(bounds: Rect, label: &str, theme: Arc<Mutex<ThemeManager>>) -> Self {
        let theme_manager = theme.lock().unwrap();
        let current_theme = theme_manager.current_theme();
        let background_color = current_theme.button_color();
        let text_color = current_theme.button_text_color();
        let hover_color = current_theme.button_hover_color();
        let pressed_color = current_theme.button_pressed_color();
        // 释放锁，避免同时持有锁和移动所有权
        drop(theme_manager);
        
        Self {
            bounds,
            label: label.to_string(),
            background_color,
            text_color,
            hover_color,
            pressed_color,
            is_hover: false,
            is_pressed: false,
            is_visible: true,
            on_click: None,
            theme: theme.clone(),
        }
    }
    
    /// 设置点击回调
    pub fn set_on_click<F>(&mut self, callback: F) where F: Fn() + Send + Sync + 'static {
        self.on_click = Some(Box::new(callback));
    }
}

impl Widget for Button {
    fn draw(&self, renderer: &mut dyn Renderer) {
        if !self.is_visible {
            return;
        }
        
        let color = if self.is_pressed {
            self.pressed_color
        } else if self.is_hover {
            self.hover_color
        } else {
            self.background_color
        };
        
        // 绘制背景
        renderer.fill_rect(self.bounds, color);
        
        // 绘制文本
        let (center_x, center_y) = self.bounds.center();
        let text_position = (
            center_x - (self.label.len() as f32 * 7.0) / 2.0, // 简单估算文本宽度
            center_y - 10.0, // 简单估算文本高度
        );
        renderer.draw_text(&self.label, text_position, self.text_color, 16.0);
    }
    
    fn update(&mut self, _delta_time: f32) {
        // 按钮通常不需要更新逻辑
    }
    
    fn handle_click(&mut self, x: f32, y: f32) -> bool {
        if !self.is_visible {
            return false;
        }
        
        if self.bounds.contains(x, y) {
            self.is_pressed = true;
            
            // 触发点击回调
            if let Some(callback) = &self.on_click {
                callback();
            }
            
            return true;
        }
        
        false
    }
    
    fn bounds(&self) -> Rect {
        self.bounds
    }
    
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }
    
    fn is_visible(&self) -> bool {
        self.is_visible
    }
    
    fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }
}

/// 文本框控件
pub struct TextBox {
    bounds: Rect,
    text: String,
    placeholder: String,
    background_color: (u8, u8, u8, u8),
    text_color: (u8, u8, u8, u8),
    border_color: (u8, u8, u8, u8),
    is_focused: bool,
    is_visible: bool,
    cursor_position: usize,
    theme: Arc<Mutex<ThemeManager>>,
}

impl TextBox {
    /// 创建新文本框
    pub fn new(bounds: Rect, placeholder: &str, theme: Arc<Mutex<ThemeManager>>) -> Self {
        let theme_manager = theme.lock().unwrap();
        let current_theme = theme_manager.current_theme();
        let background_color = current_theme.textbox_color();
        let text_color = current_theme.textbox_text_color();
        let border_color = current_theme.textbox_border_color();
        // 释放锁，避免同时持有锁和移动所有权
        drop(theme_manager);
        
        Self {
            bounds,
            text: String::new(),
            placeholder: placeholder.to_string(),
            background_color,
            text_color,
            border_color,
            is_focused: false,
            is_visible: true,
            cursor_position: 0,
            theme: theme.clone(),
        }
    }
    
    /// 获取文本
    pub fn text(&self) -> &str {
        &self.text
    }
    
    /// 设置文本
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.cursor_position = self.text.len();
    }
    
    /// 添加字符
    pub fn add_char(&mut self, c: char) {
        self.text.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }
    
    /// 删除字符
    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.text.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }
    
    /// 设置焦点
    pub fn set_focus(&mut self, focused: bool) {
        self.is_focused = focused;
    }
}

impl Widget for TextBox {
    fn draw(&self, renderer: &mut dyn Renderer) {
        if !self.is_visible {
            return;
        }
        
        // 绘制背景
        renderer.fill_rect(self.bounds, self.background_color);
        
        // 绘制边框
        let border_rect = Rect {
            x: self.bounds.x - 1.0,
            y: self.bounds.y - 1.0,
            width: self.bounds.width + 2.0,
            height: self.bounds.height + 2.0,
        };
        renderer.fill_rect(border_rect, self.border_color);
        renderer.fill_rect(self.bounds, self.background_color);
        
        // 绘制文本
        let text_position = (self.bounds.x + 5.0, self.bounds.y + self.bounds.height / 2.0 - 8.0);
        
        if self.text.is_empty() {
            // 绘制占位符
            let placeholder_color = (
                self.text_color.0 / 2,
                self.text_color.1 / 2,
                self.text_color.2 / 2,
                self.text_color.3 / 2,
            );
            renderer.draw_text(&self.placeholder, text_position, placeholder_color, 16.0);
        } else {
            // 绘制实际文本
            renderer.draw_text(&self.text, text_position, self.text_color, 16.0);
        }
        
        // 如果有焦点，绘制光标
        if self.is_focused {
            let cursor_x = text_position.0 + (self.cursor_position as f32 * 8.0); // 简单估算字符宽度
            let cursor_rect = Rect {
                x: cursor_x,
                y: text_position.1,
                width: 2.0,
                height: 16.0,
            };
            renderer.fill_rect(cursor_rect, self.text_color);
        }
    }
    
    fn update(&mut self, _delta_time: f32) {
        // 文本框通常不需要更新逻辑
    }
    
    fn handle_click(&mut self, x: f32, y: f32) -> bool {
        if !self.is_visible {
            return false;
        }
        
        if self.bounds.contains(x, y) {
            self.is_focused = true;
            // 简单估算光标位置
            let text_start_x = self.bounds.x + 5.0;
            let relative_x = x - text_start_x;
            let estimated_position = (relative_x / 8.0) as usize; // 简单估算字符宽度
            self.cursor_position = estimated_position.min(self.text.len());
            return true;
        } else {
            self.is_focused = false;
        }
        
        false
    }
    
    fn bounds(&self) -> Rect {
        self.bounds
    }
    
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }
    
    fn is_visible(&self) -> bool {
        self.is_visible
    }
    
    fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }
}

/// 标签控件
pub struct Label {
    bounds: Rect,
    text: String,
    text_color: (u8, u8, u8, u8),
    font_size: f32,
    is_visible: bool,
    theme: Arc<Mutex<ThemeManager>>,
}

impl Label {
    /// 创建新标签
    pub fn new(bounds: Rect, text: &str, theme: Arc<Mutex<ThemeManager>>) -> Self {
        let theme_manager = theme.lock().unwrap();
        let current_theme = theme_manager.current_theme();
        let text_color = current_theme.label_text_color();
        // 释放锁，避免同时持有锁和移动所有权
        drop(theme_manager);
        
        Self {
            bounds,
            text: text.to_string(),
            text_color,
            font_size: 16.0,
            is_visible: true,
            theme: theme.clone(),
        }
    }
    
    /// 设置文本
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
    
    /// 设置字体大小
    pub fn set_font_size(&mut self, size: f32) {
        self.font_size = size;
    }
}

impl Widget for Label {
    fn draw(&self, renderer: &mut dyn Renderer) {
        if !self.is_visible {
            return;
        }
        
        // 绘制文本
        let text_position = (self.bounds.x, self.bounds.y + self.bounds.height / 2.0 - self.font_size / 2.0);
        renderer.draw_text(&self.text, text_position, self.text_color, self.font_size);
    }
    
    fn update(&mut self, _delta_time: f32) {
        // 标签通常不需要更新逻辑
    }
    
    fn handle_click(&mut self, _x: f32, _y: f32) -> bool {
        // 标签通常不响应点击
        false
    }
    
    fn bounds(&self) -> Rect {
        self.bounds
    }
    
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }
    
    fn is_visible(&self) -> bool {
        self.is_visible
    }
    
    fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }
}

/// 菜单项
pub struct MenuItem {
    bounds: Rect,
    label: String,
    background_color: (u8, u8, u8, u8),
    text_color: (u8, u8, u8, u8),
    hover_color: (u8, u8, u8, u8),
    is_hover: bool,
    is_visible: bool,
    on_click: Option<Box<dyn Fn() + Send + Sync>>,
    submenu: Option<Vec<MenuItem>>,
    theme: Arc<Mutex<ThemeManager>>,
}

impl MenuItem {
    /// 创建新菜单项
    pub fn new(bounds: Rect, label: &str, theme: Arc<Mutex<ThemeManager>>) -> Self {
        let theme_manager = theme.lock().unwrap();
        let current_theme = theme_manager.current_theme();
        let background_color = current_theme.menu_color();
        let text_color = current_theme.menu_text_color();
        let hover_color = current_theme.menu_hover_color();
        // 释放锁，避免同时持有锁和移动所有权
        drop(theme_manager);
        
        Self {
            bounds,
            label: label.to_string(),
            background_color,
            text_color,
            hover_color,
            is_hover: false,
            is_visible: true,
            on_click: None,
            submenu: None,
            theme: theme.clone(),
        }
    }
    
    /// 设置点击回调
    pub fn set_on_click<F>(&mut self, callback: F) where F: Fn() + Send + Sync + 'static {
        self.on_click = Some(Box::new(callback));
    }
    
    /// 添加子菜单项
    pub fn add_submenu_item(&mut self, item: MenuItem) {
        if self.submenu.is_none() {
            self.submenu = Some(Vec::new());
        }
        
        if let Some(submenu) = &mut self.submenu {
            submenu.push(item);
        }
    }
    
    /// 绘制菜单项
    pub fn draw(&self, renderer: &mut dyn Renderer, is_active: bool) {
        if !self.is_visible {
            return;
        }
        
        let color = if is_active || self.is_hover {
            self.hover_color
        } else {
            self.background_color
        };
        
        // 绘制背景
        renderer.fill_rect(self.bounds, color);
        
        // 绘制文本
        let text_position = (
            self.bounds.x + 5.0,
            self.bounds.y + self.bounds.height / 2.0 - 8.0,
        );
        renderer.draw_text(&self.label, text_position, self.text_color, 16.0);
        
        // 绘制子菜单
        if is_active && self.submenu.is_some() {
            if let Some(submenu) = &self.submenu {
                for (i, item) in submenu.iter().enumerate() {
                    item.draw(renderer, false);
                }
            }
        }
    }
    
    /// 处理点击事件
    pub fn handle_click(&mut self, x: f32, y: f32) -> bool {
        if !self.is_visible {
            return false;
        }
        
        if self.bounds.contains(x, y) {
            // 触发点击回调
            if let Some(callback) = &self.on_click {
                callback();
            }
            
            return true;
        }
        
        // 检查子菜单
        if let Some(submenu) = &mut self.submenu {
            for item in submenu {
                if item.handle_click(x, y) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// 获取边界
    pub fn bounds(&self) -> Rect {
        self.bounds
    }
}
