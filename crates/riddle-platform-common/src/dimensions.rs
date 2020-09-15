use crate::traits::WindowExt;

use riddle_math::Vector2;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LogicalSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LogicalPosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LogicalVec2 {
    pub x: u32,
    pub y: u32,
}

impl LogicalSize {
    pub fn into_physical<W: WindowExt>(self, window: &W) -> (u32, u32) {
        window.logical_to_physical(self)
    }
}

impl From<LogicalVec2> for LogicalSize {
    fn from(vec2: LogicalVec2) -> Self {
        Self {
            width: vec2.x,
            height: vec2.y,
        }
    }
}

impl From<Vector2<u32>> for LogicalSize {
    fn from(vec: Vector2<u32>) -> Self {
        Self {
            width: vec.x as u32,
            height: vec.y as u32,
        }
    }
}

impl From<LogicalSize> for Vector2<f32> {
    fn from(size: LogicalSize) -> Self {
        Self {
            x: size.width as f32,
            y: size.height as f32,
        }
    }
}

impl LogicalPosition {
    pub fn into_physical<W: WindowExt>(self, window: &W) -> (u32, u32) {
        window.logical_to_physical(self)
    }
}

impl From<LogicalVec2> for LogicalPosition {
    fn from(vec2: LogicalVec2) -> Self {
        Self {
            x: vec2.x,
            y: vec2.y,
        }
    }
}

impl From<Vector2<u32>> for LogicalPosition {
    fn from(vec: Vector2<u32>) -> Self {
        Self {
            x: vec.x as u32,
            y: vec.y as u32,
        }
    }
}

impl Default for LogicalPosition {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl From<LogicalPosition> for Vector2<f32> {
    fn from(pos: LogicalPosition) -> Self {
        Self {
            x: pos.x as f32,
            y: pos.y as f32,
        }
    }
}

impl LogicalVec2 {
    pub fn into_physical<W: WindowExt>(self, window: &W) -> (u32, u32) {
        window.logical_to_physical(self)
    }
}

impl From<LogicalSize> for LogicalVec2 {
    fn from(size: LogicalSize) -> Self {
        Self {
            x: size.width,
            y: size.height,
        }
    }
}

impl From<LogicalPosition> for LogicalVec2 {
    fn from(pos: LogicalPosition) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

impl From<Vector2<u32>> for LogicalVec2 {
    fn from(vec: Vector2<u32>) -> Self {
        Self {
            x: vec.x as u32,
            y: vec.y as u32,
        }
    }
}

impl From<LogicalVec2> for Vector2<f32> {
    fn from(vec: LogicalVec2) -> Self {
        Self {
            x: vec.x as f32,
            y: vec.y as f32,
        }
    }
}
