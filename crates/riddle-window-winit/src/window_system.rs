use crate::{event::InternalEvent, *};

use riddle_common::eventpub::EventPub;

use std::{
    cell::RefCell,
    collections::HashMap,
    ops::Deref,
    rc::{Rc, Weak},
};

pub struct WindowSystem {
    event_loop: RefCell<Option<winit::event_loop::EventLoop<InternalEvent>>>,
    event_proxy: winit::event_loop::EventLoopProxy<InternalEvent>,
    windows: RefCell<HashMap<WindowId, Weak<Window>>>,
    event_pub: EventPub<SystemEvent<Rc<Window>>>,
}

pub struct WindowContext<'a> {
    pub(crate) system: Rc<WindowSystem>,
    event_loop: Option<&'a winit::event_loop::EventLoopWindowTarget<InternalEvent>>,
    triggering_event: SystemEvent<Rc<Window>>,
}

impl WindowSystem {
    pub fn borrow_context(this: &Rc<WindowSystem>) -> Result<WindowContext, WindowError> {
        Ok(WindowContext {
            system: this.clone(),
            event_loop: None,
            triggering_event: SystemEvent::Unknown,
        })
    }

    /// Starts the main even loop for this window system.
    ///
    /// # Panics
    ///
    /// If run has already been invoked, then this function will panic.
    pub fn run<F>(this: Rc<Self>, main_loop: F) -> !
    where
        F: FnMut(WindowContext) + 'static,
    {
        let el = std::mem::replace(&mut *this.event_loop.borrow_mut(), None).unwrap();
        let mut main_loop = main_loop;
        el.run(move |event, el, cf| {
            match &event {
                winit::event::Event::UserEvent(InternalEvent::QuitRequested) => {
                    *cf = winit::event_loop::ControlFlow::Exit
                }
                _ => *cf = winit::event_loop::ControlFlow::Poll,
            }

            match event::convert_winit_event(&this, event) {
                Some(system_event) => {
                    let ctx = WindowContext {
                        system: this.clone(),
                        event_loop: Some(el),
                        triggering_event: system_event.clone(),
                    };

                    this.event_pub.dispatch(&system_event);
                    this.update_windows();

                    main_loop(ctx);
                }
                None => (),
            }
        })
    }

    pub fn new() -> WindowSystem {
        let event_loop = winit::event_loop::EventLoop::with_user_event();
        let event_proxy = event_loop.create_proxy();
        WindowSystem {
            event_loop: RefCell::new(event_loop.into()),
            event_proxy,
            windows: RefCell::new(HashMap::new()),
            event_pub: EventPub::new(),
        }
    }

    pub fn event_pub(&self) -> &EventPub<SystemEvent<Rc<Window>>> {
        &self.event_pub
    }

    pub(crate) fn register_window(&self, window: &Rc<Window>) {
        self.windows
            .borrow_mut()
            .insert(window.window_id(), Rc::downgrade(window));
    }

    pub(crate) fn unregister_window(&self, window_id: &WindowId) {
        self.windows.borrow_mut().remove(window_id);
    }

    pub fn lookup_window(&self, window_id: &WindowId) -> Option<Rc<Window>> {
        self.windows
            .borrow()
            .get(window_id)
            .and_then(|weak| weak.upgrade().map(|rc| rc.into()))
    }

    fn update_windows(&self) {
        for (_, window) in self.windows.borrow().iter() {
            match window.upgrade() {
                Some(window) => window.update(),
                _ => (),
            }
        }
    }
}

impl riddle_window_common::traits::WindowSystem for WindowSystem {
    type WindowHandle = Rc<Window>;

    fn event_pub(&self) -> &EventPub<riddle_window_common::SystemEvent<Self::WindowHandle>> {
        &self.event_pub
    }
}

impl<'a> WindowContext<'a> {
    pub(crate) fn with_event_loop<T, F>(&self, f: F) -> Result<T, WindowError>
    where
        F: FnOnce(
            &winit::event_loop::EventLoopWindowTarget<InternalEvent>,
        ) -> Result<T, WindowError>,
    {
        match self.event_loop {
            Some(el) => f(el),
            None => {
                let el_ref = self.system.event_loop.borrow();
                let el = el_ref.deref();
                match el {
                    Some(el) => f(el),
                    None => Err(WindowError::Unknown),
                }
            }
        }
    }

    pub fn quit(&self) -> Result<(), WindowError> {
        self.system
            .event_proxy
            .send_event(InternalEvent::QuitRequested)
            .map_err(|_| WindowError::Unknown)
    }

    pub fn event(&self) -> &SystemEvent<Rc<Window>> {
        &self.triggering_event
    }
}
/*
impl RiddleModule for WindowSystem {
    fn handle_event(
        &self,
        event: &winit::event::Event<InternalEvent>,
        _: &RiddleState,
    ) -> RdlResult<Option<RdlEvent>> {
        match event {
            winit::event::Event::WindowEvent { window_id, event } => {
                let window = self.lookup_window(&window_id.clone().into());
                match window {
                    Some(window) => Window::handle_event(&window, event)
                        .map(|we| we.map(|we| RdlEvent::Window(we))),
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }
}
*/
