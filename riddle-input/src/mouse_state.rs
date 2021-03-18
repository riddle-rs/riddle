use crate::*;

use std::collections::HashSet;

pub(crate) struct MouseState {
	logical_position: LogicalPosition,
	buttons: HashSet<MouseButton>,
}

impl MouseState {
	pub fn set_position(&mut self, position: LogicalPosition) {
		self.logical_position = position;
	}

	pub fn button_down(&mut self, button: MouseButton) {
		self.buttons.insert(button);
	}

	pub fn button_up(&mut self, button: MouseButton) {
		self.buttons.remove(&button);
	}

	pub fn position(&self) -> LogicalPosition {
		self.logical_position
	}

	pub fn is_button_down(&self, button: MouseButton) -> bool {
		self.buttons.contains(&button)
	}
}

impl Default for MouseState {
	fn default() -> Self {
		MouseState {
			logical_position: LogicalPosition { x: 0, y: 0 },
			buttons: HashSet::new(),
		}
	}
}
