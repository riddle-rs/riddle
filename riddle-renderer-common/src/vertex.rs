use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
	pub pos: [f32; 2],
	pub uv: [f32; 2],
	pub color: [f32; 4],
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
			pos: [pos.x, pos.y],
			uv: uv.into(),
			color: *color,
		}
	}
}
