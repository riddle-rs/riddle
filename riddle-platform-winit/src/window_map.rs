use crate::{common::*, *};

use std::collections::HashMap;

pub(crate) struct WindowMap {
    next_window_id: u32,

    winit_windows: HashMap<winit::window::WindowId, WindowWeak>,
    windows: HashMap<WindowId, WindowWeak>,
}

impl WindowMap {
    pub fn new() -> Self {
        Self {
            next_window_id: 0,

            winit_windows: HashMap::new(),
            windows: HashMap::new(),
        }
    }

    pub fn register_window(&mut self, window: WindowHandle) {
        self.windows
            .insert(window.id(), WindowHandle::downgrade(&window));
        self.winit_windows
            .insert(window.winit_window_id(), WindowHandle::downgrade(&window));
    }

    pub fn unregister_window(&mut self, window: &Window) {
        self.windows.remove(&window.id());
        self.winit_windows.remove(&window.winit_window_id());
    }

    pub fn lookup_window(&self, window_id: WindowId) -> Option<WindowHandle> {
        self.windows
            .get(&window_id)
            .and_then(|weak| WindowWeak::upgrade(weak))
    }

    pub fn lookup_winit_window(&self, winit_id: winit::window::WindowId) -> Option<WindowHandle> {
        self.winit_windows
            .get(&winit_id)
            .and_then(|weak| WindowWeak::upgrade(weak))
    }

    pub fn take_next_window_id(&mut self) -> WindowId {
        let id = self.next_window_id;
        self.next_window_id += 1;

        WindowId::new(id)
    }

    pub fn windows(&self) -> Vec<WindowHandle> {
        self.windows
            .values()
            .into_iter()
            .filter_map(|w| WindowWeak::upgrade(w))
            .collect()
    }
}
