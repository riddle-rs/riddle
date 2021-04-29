use crate::{common::*, *};

use std::collections::HashMap;

pub(crate) struct WindowMap {
	next_window_id: u32,

	winit_windows: HashMap<winit::window::WindowId, std::sync::Weak<WindowInternal>>,
	windows: HashMap<WindowId, std::sync::Weak<WindowInternal>>,
}

impl WindowMap {
	pub fn new() -> Self {
		Self {
			next_window_id: 0,

			winit_windows: HashMap::new(),
			windows: HashMap::new(),
		}
	}

	pub fn register_window(&mut self, window: Window) {
		let weak_window = std::sync::Arc::downgrade(&window.internal);
		self.windows.insert(window.id(), weak_window.clone());
		self.winit_windows
			.insert(window.winit_window_id(), weak_window);
	}

	pub fn unregister_window_internal(&mut self, window: &WindowInternal) {
		self.windows.remove(&window.id);
		self.winit_windows.remove(&window.winit_window.id());
	}

	pub fn lookup_window(&self, window_id: WindowId) -> Option<Window> {
		self.windows
			.get(&window_id)
			.and_then(|weak| std::sync::Weak::upgrade(weak))
			.map(|internal| Window { internal })
	}

	pub fn lookup_winit_window(&self, winit_id: winit::window::WindowId) -> Option<Window> {
		self.winit_windows
			.get(&winit_id)
			.and_then(|weak| std::sync::Weak::upgrade(weak))
			.map(|internal| Window { internal })
	}

	pub fn take_next_window_id(&mut self) -> WindowId {
		let id = self.next_window_id;
		self.next_window_id += 1;

		WindowId::new(id)
	}

	pub fn windows(&self) -> Vec<Window> {
		self.windows
			.values()
			.into_iter()
			.filter_map(|w| std::sync::Weak::upgrade(w))
			.map(|internal| Window { internal })
			.collect()
	}
}
