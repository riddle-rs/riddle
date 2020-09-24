use crate::{event::InternalEvent, *};

use riddle_common::eventpub::EventPub;

use std::{cell::RefCell, sync::Mutex};

pub struct PlatformSystem {
    weak_self: PlatformSystemWeak,
    pub(crate) event_proxy: Mutex<winit::event_loop::EventLoopProxy<InternalEvent>>,

    window_map: Mutex<WindowMap>,

    event_pub: EventPub<PlatformEvent>,
}

define_handles!(<PlatformSystem>::weak_self, pub PlatformSystemHandle, pub PlatformSystemWeak);

impl PlatformSystem {
    pub fn new() -> (PlatformSystemHandle, PlatformMainThreadState) {
        let event_loop = winit::event_loop::EventLoop::with_user_event();
        let event_proxy = event_loop.create_proxy();
        let system = PlatformSystemHandle::new(|weak_self| PlatformSystem {
            weak_self,
            event_proxy: Mutex::new(event_proxy),

            window_map: WindowMap::new().into(),

            event_pub: EventPub::new(),
        });
        let main_thread_state = PlatformMainThreadState {
            system: system.clone(),
            event_loop: RefCell::new(Some(event_loop)),
        };
        (system, main_thread_state)
    }

    pub fn event_pub(&self) -> &EventPub<PlatformEvent> {
        &self.event_pub
    }

    #[inline]
    pub(crate) fn with_window_map<R, F: FnOnce(&WindowMap) -> R>(&self, f: F) -> R {
        f(&self.window_map.lock().unwrap())
    }

    #[inline]
    pub(crate) fn with_window_map_mut<R, F: FnOnce(&mut WindowMap) -> R>(&self, f: F) -> R {
        f(&mut self.window_map.lock().unwrap())
    }

    pub fn lookup_window(&self, window_id: WindowId) -> Option<WindowHandle> {
        self.window_map.lock().unwrap().lookup_window(window_id)
    }

    fn update_windows(&self) {
        let windows = self.window_map.lock().unwrap().windows();
        for window in windows {
            window.update()
        }
    }
}

impl riddle_platform_common::traits::WindowSystem for PlatformSystem {
    fn event_pub(&self) -> &EventPub<riddle_platform_common::PlatformEvent> {
        &self.event_pub
    }
}

pub struct PlatformMainThreadState {
    pub(crate) system: PlatformSystemHandle,
    pub(crate) event_loop: RefCell<Option<winit::event_loop::EventLoop<InternalEvent>>>,
}

impl PlatformMainThreadState {
    /// Starts the main even loop for this window system.
    ///
    /// # Panics
    ///
    /// If run has already been invoked, then this function will panic.
    pub fn run<F>(self, main_loop: F) -> !
    where
        F: FnMut(PlatformContext) + 'static,
    {
        let el = std::mem::replace(&mut *self.event_loop.borrow_mut(), None).unwrap();
        let mut main_loop = main_loop;
        let this = self.system.clone_handle();
        el.run(move |event, el, cf| {
            match &event {
                winit::event::Event::UserEvent(InternalEvent::QuitRequested) => {
                    *cf = winit::event_loop::ControlFlow::Exit
                }
                _ => *cf = winit::event_loop::ControlFlow::Poll,
            }

            match event::convert_winit_event(&this, event) {
                Some(system_event) => {
                    let ctx = PlatformContext {
                        main_thread_state: &self,
                        event_loop: Some(el),
                        triggering_event: system_event.clone(),
                    };

                    this.event_pub.dispatch(system_event);
                    this.update_windows();

                    main_loop(ctx);
                }
                None => (),
            }
        })
    }

    pub fn borrow_context(&self) -> PlatformContext {
        PlatformContext {
            main_thread_state: &self,
            event_loop: None,
            triggering_event: PlatformEvent::Unknown,
        }
    }
}
