use crate::*;

use riddle_common::eventpub::*;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use riddle_window_common::traits::WindowExt;
use std::{borrow::Borrow, rc::Rc};

pub struct Window {
    window_system: Rc<WindowSystem>,
    winit_window: winit::window::Window,
    event_sub: EventSub<SystemEvent>,
    event_pub: EventPub<WindowEvent>,

    id: riddle_window_common::WindowId,
}

/*#[derive(Eq, PartialEq, Hash)]
pub struct WindowId {
    id: winit::window::WindowId,
}*/

pub struct WindowBuilder {
    width: u32,
    height: u32,
    title: String,
    resizeable: bool,
    cursor_visible: bool,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        WindowBuilder {
            width: 800,
            height: 600,
            title: String::from("Riddle Window"),
            resizeable: true,
            cursor_visible: true,
        }
    }
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn resizeable(&mut self, resizeable: bool) -> &mut Self {
        self.resizeable = resizeable;
        self
    }

    pub fn title<S: Into<String>>(&mut self, title: S) -> &mut Self {
        self.title = title.into();
        self
    }

    pub fn cursor_visible(&mut self, cursor_visible: bool) -> &mut Self {
        self.cursor_visible = cursor_visible;
        self
    }

    pub fn build<'a, C>(&self, ctx: C) -> Result<Rc<Window>, WindowError>
    where
        C: Borrow<WindowContext<'a>>,
    {
        Window::new_shared(ctx.borrow(), self)
    }

    fn configure_window(&self, window: &winit::window::Window) {
        window.set_cursor_visible(self.cursor_visible);
    }
}

impl Window {
    fn new_shared(ctx: &WindowContext, args: &WindowBuilder) -> Result<Rc<Window>, WindowError> {
        let window: Rc<Window> = Window::new(ctx, args)?.into();
        ctx.system
            .borrow_window_map_mut()
            .register_window(window.clone());
        Ok(window)
    }

    fn new(ctx: &WindowContext, args: &WindowBuilder) -> Result<Self, WindowError> {
        let system = ctx.system.clone();

        #[cfg(target_os = "windows")]
        let winit_builder = {
            use winit::platform::windows::WindowBuilderExtWindows;
            winit::window::WindowBuilder::new()
                // Required to prevent cpal fighting with winit over ole threading model and panicing
                // Tracking Issues:
                // https://github.com/RustAudio/cpal/pull/348
                // https://github.com/rust-windowing/winit/pull/1524
                .with_drag_and_drop(false)
        };

        #[cfg(not(target_os = "windows"))]
        let winit_builder = winit::window::WindowBuilder::new();

        let winit_builder = winit_builder
            .with_inner_size(winit::dpi::LogicalSize::new(
                args.width as f32,
                args.height as f32,
            ))
            .with_title(args.title.clone())
            .with_resizable(args.resizeable);

        let winit_window =
            ctx.with_event_loop(|el| winit_builder.build(el).map_err(|_| WindowError::Unknown))?;

        args.configure_window(&winit_window);

        let event_sub = EventSub::new_with_filter(Self::event_filter);
        system.event_pub().attach(&event_sub);

        let window = Self {
            window_system: system.clone(),
            winit_window: winit_window,
            event_sub,
            event_pub: EventPub::new(),
            id: system.borrow_window_map_mut().take_next_window_id(),
        };

        Ok(window)
    }

    pub fn drawable_size(&self) -> (u32, u32) {
        let size = self.winit_window.inner_size();
        (size.width, size.height)
    }

    pub fn logical_size(&self) -> LogicalSize {
        let physical_size = self.winit_window.inner_size();
        let logical_size: winit::dpi::LogicalSize<u32> =
            physical_size.to_logical(self.winit_window.scale_factor());
        dimensions::logical_size_from_winit(logical_size)
    }

    pub fn scale_factor(&self) -> f64 {
        self.winit_window.scale_factor()
    }

    pub fn set_title(&self, title: &str) {
        self.winit_window.set_title(title)
    }

    pub fn subscribe_to_events(&self, sub: &EventSub<WindowEvent>) {
        self.event_pub.attach(sub);
    }

    pub(crate) fn update(&self) {
        for event in self.event_sub.collect() {
            match event {
                SystemEvent::Window(event) => self.event_pub.dispatch(&event),
                _ => (),
            }
        }
    }

    pub(crate) fn winit_window_id(&self) -> winit::window::WindowId {
        self.winit_window.id()
    }

    fn event_filter(event: &SystemEvent) -> bool {
        match event {
            SystemEvent::Window(WindowEvent::WindowResize(_)) => true,
            _ => false,
        }
    }
}

impl WindowExt for Window {
    fn logical_to_physical<L: Into<LogicalVec2>>(&self, vec2: L) -> (u32, u32) {
        let winit_pos = dimensions::logical_vec_to_winit(vec2.into());
        let physical_size = winit_pos.to_physical(self.scale_factor());
        (physical_size.x, physical_size.y)
    }

    fn window_id(&self) -> WindowId {
        self.id
    }
}

impl std::cmp::PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.window_id() == other.window_id()
    }
}

impl std::cmp::Eq for Window {}

impl Drop for Window {
    fn drop(&mut self) {
        self.window_system
            .borrow_window_map_mut()
            .unregister_window(&self);
    }
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.winit_window.raw_window_handle()
    }
}
