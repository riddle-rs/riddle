use crate::*;

use riddle_window_common::SystemEvent;

use std::rc::Rc;

#[derive(Debug)]
pub(crate) enum InternalEvent {
    QuitRequested,
}

pub(crate) fn convert_winit_event(
    system: &WindowSystem,
    event: winit::event::Event<InternalEvent>,
) -> Option<SystemEvent<Rc<Window>>> {
    match event {
        winit::event::Event::WindowEvent { window_id, event } => {
            let window = system.lookup_window(&window_id.into());
            window.and_then(|window| convert_winit_window_event(window, event))
        }
        winit::event::Event::MainEventsCleared => SystemEvent::ProcessFrame.into(),
        _ => None,
    }
}

fn convert_winit_window_event(
    window: Rc<Window>,
    event: winit::event::WindowEvent,
) -> Option<SystemEvent<Rc<Window>>> {
    match event {
        winit::event::WindowEvent::CloseRequested => Some(WindowEvent::WindowClose(window).into()),
        winit::event::WindowEvent::Resized(_) => Some(WindowEvent::WindowResize(window).into()),
        winit::event::WindowEvent::CursorMoved { position, .. } => Some(
            InputEvent::CursorMove {
                window: window.clone(),
                position: dimensions::logical_pos_from_winit(
                    position.to_logical(window.scale_factor()),
                ),
            }
            .into(),
        ),
        winit::event::WindowEvent::MouseInput { state, .. } => match state {
            winit::event::ElementState::Pressed => {
                Some(InputEvent::MouseButtonDown { window }.into())
            }
            winit::event::ElementState::Released => {
                Some(InputEvent::MouseButtonUp { window }.into())
            }
        },
        winit::event::WindowEvent::KeyboardInput {
            input: winit::event::KeyboardInput {
                scancode, state, ..
            },
            ..
        } => match state {
            winit::event::ElementState::Pressed => Some(
                InputEvent::KeyDown {
                    window,
                    scancode: scancode::from_u32(scancode),
                }
                .into(),
            ),
            winit::event::ElementState::Released => Some(
                InputEvent::KeyUp {
                    window,
                    scancode: scancode::from_u32(scancode),
                }
                .into(),
            ),
        },
        _ => None,
    }
}
