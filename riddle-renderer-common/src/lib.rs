#![feature(generic_associated_types)]

use riddle_math::Vector2;

trait Renderer {
	type RenderContextT<'a>: RenderContext + 'a;

	fn dimensions(&self) -> Vector2<f32>;
	fn begin_render(&self) -> Result<Self::RenderContextT<'_>, ()>;
}

trait RenderContext {}
