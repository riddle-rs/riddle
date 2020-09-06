use crate::*;

use riddle_window_common::SystemEvent;

use std::rc::Rc;
use traits::WindowExt;

#[derive(Debug)]
pub(crate) enum InternalEvent {
    QuitRequested,
}

pub(crate) fn convert_winit_event(
    system: &WindowSystem,
    event: winit::event::Event<InternalEvent>,
) -> Option<SystemEvent> {
    match event {
        winit::event::Event::WindowEvent { window_id, event } => {
            let window = system.borrow_window_map().lookup_winit_window(window_id);
            window.and_then(|window| convert_winit_window_event(window, event))
        }
        winit::event::Event::MainEventsCleared => SystemEvent::ProcessFrame.into(),
        _ => None,
    }
}

fn convert_winit_window_event(
    window: Rc<Window>,
    event: winit::event::WindowEvent,
) -> Option<SystemEvent> {
    match event {
        winit::event::WindowEvent::CloseRequested => {
            Some(WindowEvent::WindowClose(window.window_id()).into())
        }
        winit::event::WindowEvent::Resized(_) => {
            Some(WindowEvent::WindowResize(window.window_id()).into())
        }
        winit::event::WindowEvent::CursorMoved { position, .. } => Some(
            InputEvent::CursorMove {
                window: window.clone().window_id(),
                position: dimensions::logical_pos_from_winit(
                    position.to_logical(window.scale_factor()),
                ),
            }
            .into(),
        ),
        winit::event::WindowEvent::MouseInput { state, .. } => match state {
            winit::event::ElementState::Pressed => Some(
                InputEvent::MouseButtonDown {
                    window: window.window_id(),
                }
                .into(),
            ),
            winit::event::ElementState::Released => Some(
                InputEvent::MouseButtonUp {
                    window: window.window_id(),
                }
                .into(),
            ),
        },
        winit::event::WindowEvent::KeyboardInput {
            input: winit::event::KeyboardInput {
                scancode, state, ..
            },
            ..
        } => match state {
            winit::event::ElementState::Pressed => Some(
                InputEvent::KeyDown {
                    window: window.window_id(),
                    scancode: scancode::from_u32(scancode),
                }
                .into(),
            ),
            winit::event::ElementState::Released => Some(
                InputEvent::KeyUp {
                    window: window.window_id(),
                    scancode: scancode::from_u32(scancode),
                }
                .into(),
            ),
        },
        _ => None,
    }
}
