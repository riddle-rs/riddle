use crate::traits::WindowCommon;

use riddle_math::Vector2;

/// A 2d size in logical screen units.
///
/// Logical units may be different to screen units depending on dpi scaling
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LogicalSize {
	pub width: u32,
	pub height: u32,
}

/// A 2d position in logical screen units.
///
/// Logical units may be different to screen units depending on dpi scaling
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LogicalPosition {
	pub x: u32,
	pub y: u32,
}

/// A 2d vector in logical screen units.
///
/// Logical units may be different to screen units depending on dpi scaling
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LogicalVec2 {
	pub x: u32,
	pub y: u32,
}

impl LogicalSize {
	/// Convert the size from logical units to physical pixel units, with respect
	/// to a specific window's current scaling.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_platform_common::*;
	/// # let window = doctest::MockWindow {};
	/// // Given a window with DPI scaling factor of 2:
	/// let size = LogicalSize{ width: 10, height: 10 };
	/// assert_eq!((20, 20), size.into_physical(&window));
	/// ```
	pub fn into_physical<W: WindowCommon>(self, window: &W) -> (u32, u32) {
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
	/// Convert the position from logical units to physical pixel units, with respect
	/// to a specific window's current scaling.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_platform_common::*;
	/// # let window = doctest::MockWindow {};
	/// // Given a window with DPI scaling factor of 2:
	/// let position = LogicalPosition{ x: 10, y: 10 };
	/// assert_eq!((20, 20), position.into_physical(&window));
	/// ```
	pub fn into_physical<W: WindowCommon>(self, window: &W) -> (u32, u32) {
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
	/// Convert the vector from logical units to physical pixel units, with respect
	/// to a specific window's current scaling.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_platform_common::*;
	/// # let window = doctest::MockWindow {};
	/// // Given a window with DPI scaling factor of 2:
	/// let vector = LogicalVec2{ x: 10, y: 10 };
	/// assert_eq!((20, 20), vector.into_physical(&window));
	/// ```
	pub fn into_physical<W: WindowCommon>(self, window: &W) -> (u32, u32) {
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
