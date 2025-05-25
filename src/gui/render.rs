use std::fmt;
use std::error::Error;
use std::collections::HashMap;

/// Render error
#[derive(Debug)]
pub enum RenderError {
    /// Initialization error
    InitializationError(String),
    /// Drawing error
    DrawingError(String),
    /// Resource error
    ResourceError(String),
    /// Other error
    Other(String),
}

impl Error for RenderError {}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::InitializationError(msg) => write!(f, "Initialization error: {}", msg),
            RenderError::DrawingError(msg) => write!(f, "Drawing error: {}", msg),
            RenderError::ResourceError(msg) => write!(f, "Resource error: {}", msg),
            RenderError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

/// Color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red component (0.0 - 1.0)
    pub r: f32,
    /// Green component (0.0 - 1.0)
    pub g: f32,
    /// Blue component (0.0 - 1.0)
    pub b: f32,
    /// Alpha component (0.0 - 1.0)
    pub a: f32,
}

impl Color {
    /// Create a new color
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    /// Create a new color from RGB values (0-255)
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }
    
    /// Create a new color from RGBA values (0-255)
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
    
    /// Black color
    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
    
    /// White color
    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }
    
    /// Red color
    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }
    
    /// Green color
    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0, 1.0)
    }
    
    /// Blue color
    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0)
    }
    
    /// Yellow color
    pub fn yellow() -> Self {
        Self::new(1.0, 1.0, 0.0, 1.0)
    }
    
    /// Cyan color
    pub fn cyan() -> Self {
        Self::new(0.0, 1.0, 1.0, 1.0)
    }
    
    /// Magenta color
    pub fn magenta() -> Self {
        Self::new(1.0, 0.0, 1.0, 1.0)
    }
    
    /// Transparent color
    pub fn transparent() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

/// Point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
}

impl Point {
    /// Create a new point
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Size
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    /// Width
    pub width: f32,
    /// Height
    pub height: f32,
}

impl Size {
    /// Create a new size
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

/// Rectangle
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    /// Position
    pub position: Point,
    /// Size
    pub size: Size,
}

impl Rectangle {
    /// Create a new rectangle
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Point::new(x, y),
            size: Size::new(width, height),
        }
    }
    
    /// Create a new rectangle from position and size
    pub fn from_position_and_size(position: Point, size: Size) -> Self {
        Self { position, size }
    }
    
    /// Get left edge
    pub fn left(&self) -> f32 {
        self.position.x
    }
    
    /// Get right edge
    pub fn right(&self) -> f32 {
        self.position.x + self.size.width
    }
    
    /// Get top edge
    pub fn top(&self) -> f32 {
        self.position.y
    }
    
    /// Get bottom edge
    pub fn bottom(&self) -> f32 {
        self.position.y + self.size.height
    }
    
    /// Check if point is inside rectangle
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.left() && point.x <= self.right() && point.y >= self.top() && point.y <= self.bottom()
    }
    
    /// Check if rectangle intersects with another rectangle
    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.left() <= other.right() && self.right() >= other.left() && self.top() <= other.bottom() && self.bottom() >= other.top()
    }
}

/// Renderer
pub struct Renderer {
    /// Renderer ID
    pub id: String,
    /// Renderer metadata
    pub metadata: HashMap<String, String>,
}

impl Renderer {
    /// Create a new renderer
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            metadata: HashMap::new(),
        }
    }
    
    /// Initialize the renderer
    pub fn initialize(&mut self) -> Result<(), RenderError> {
        // Initialize the renderer
        Ok(())
    }
    
    /// Start the renderer
    pub fn start(&mut self) -> Result<(), RenderError> {
        // Start the renderer
        Ok(())
    }
    
    /// Stop the renderer
    pub fn stop(&mut self) -> Result<(), RenderError> {
        // Stop the renderer
        Ok(())
    }
    
    /// Update the renderer
    pub fn update(&mut self) -> Result<(), RenderError> {
        // Update the renderer
        Ok(())
    }
    
    /// Clear the screen
    pub fn clear(&mut self, color: Color) -> Result<(), RenderError> {
        // Clear the screen
        Ok(())
    }
    
    /// Draw rectangle
    pub fn draw_rectangle(&mut self, rectangle: Rectangle, color: Color) -> Result<(), RenderError> {
        // Draw rectangle
        Ok(())
    }
    
    /// Draw line
    pub fn draw_line(&mut self, start: Point, end: Point, color: Color, thickness: f32) -> Result<(), RenderError> {
        // Draw line
        Ok(())
    }
    
    /// Draw circle
    pub fn draw_circle(&mut self, center: Point, radius: f32, color: Color) -> Result<(), RenderError> {
        // Draw circle
        Ok(())
    }
    
    /// Draw text
    pub fn draw_text(&mut self, text: &str, position: Point, font_size: f32, color: Color) -> Result<(), RenderError> {
        // Draw text
        Ok(())
    }
    
    /// Add metadata
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
    
    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Initialize render module
pub fn init() -> Result<(), RenderError> {
    // Initialize render module
    Ok(())
}

/// Start render module
pub fn start() -> Result<(), RenderError> {
    // Start render module
    Ok(())
}

/// Stop render module
pub fn stop() -> Result<(), RenderError> {
    // Stop render module
    Ok(())
}
