use riddle_window_common::*;

pub(crate) fn logical_size_from_winit(size: winit::dpi::LogicalSize<u32>) -> LogicalSize {
    LogicalSize {
        width: size.width,
        height: size.height,
    }
}

pub(crate) fn logical_pos_from_winit(pos: winit::dpi::LogicalPosition<u32>) -> LogicalPosition {
    LogicalPosition { x: pos.x, y: pos.y }
}

pub(crate) fn logical_vec_to_winit(vec2: LogicalVec2) -> winit::dpi::LogicalPosition<u32> {
    winit::dpi::LogicalPosition {
        x: vec2.x,
        y: vec2.y,
    }
}
