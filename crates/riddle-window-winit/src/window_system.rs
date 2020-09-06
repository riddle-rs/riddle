use crate::{event::InternalEvent, *};

use riddle_common::eventpub::EventPub;

use std::{
    cell::{Ref, RefCell, RefMut},
    ops::Deref,
    rc::Rc,
};

pub struct WindowSystem {
    event_loop: RefCell<Option<winit::event_loop::EventLoop<InternalEvent>>>,
    event_proxy: winit::event_loop::EventLoopProxy<InternalEvent>,

    window_map: RefCell<WindowMap>,

    event_pub: EventPub<SystemEvent>,
}

pub struct WindowContext<'a> {
    pub(crate) system: Rc<WindowSystem>,
    event_loop: Option<&'a winit::event_loop::EventLoopWindowTarget<InternalEvent>>,
    triggering_event: SystemEvent,
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

            window_map: WindowMap::new().into(),

            event_pub: EventPub::new(),
        }
    }

    pub fn event_pub(&self) -> &EventPub<SystemEvent> {
        &self.event_pub
    }

    pub(crate) fn borrow_window_map(&self) -> Ref<WindowMap> {
        self.window_map.borrow()
    }

    pub(crate) fn borrow_window_map_mut(&self) -> RefMut<WindowMap> {
        self.window_map.borrow_mut()
    }

    pub fn lookup_window(&self, window_id: WindowId) -> Option<Rc<Window>> {
        self.window_map.borrow().lookup_window(window_id)
    }

    fn update_windows(&self) {
        let windows = self.window_map.borrow().windows();
        for window in windows {
            window.update()
        }
    }
}

impl riddle_window_common::traits::WindowSystem for WindowSystem {
    fn event_pub(&self) -> &EventPub<riddle_window_common::SystemEvent> {
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

    pub fn event(&self) -> &SystemEvent {
        &self.triggering_event
    }
}
