use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    _pos: [f32; 2],
    _uv: [f32; 2],
    _color: [f32; 4],
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

impl Vertex {
    pub fn ptc<P, T>(pos: P, uv: T, color: &[f32; 4]) -> Self
    where
        P: Into<mint::Point2<f32>>,
        T: Into<mint::Point2<f32>>,
    {
        let pos = pos.into();
        let uv = uv.into();

        Vertex {
            _pos: [pos.x, pos.y],
            _uv: uv.into(),
            _color: *color,
        }
    }
}
