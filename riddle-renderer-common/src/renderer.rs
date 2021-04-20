use crate::{vertex::Vertex, CommonSprite, *};
use riddle_common::Color;
use riddle_math::{Rect, Vector2};

/// The root object of a renderer implementation, associated with a single display.
pub trait CommonRenderer: Sized {
	type RenderContext: RenderContext<Self>;
	type Sprite: CommonSprite<Self>;
	type Texture;
	type Shader;
	type SpriteFont;

	fn dimensions(&self) -> Vector2<f32>;

	fn render<R, F>(&self, f: F) -> Result<R>
	where
		F: FnOnce(&mut Self::RenderContext) -> Result<R>;
}

/// The context provided to render callbacks
pub trait RenderContext<R: CommonRenderer> {
	/// Replace the current world transform.
	fn set_transform(&mut self, transform: mint::ColumnMatrix4<f32>) -> Result<()>;

	/// Fill the target with a flat color.
	fn clear(&mut self, color: Color<f32>) -> Result<()>;

	/// Draw a `Renderable` to the target with the current world transform.
	fn draw(&mut self, renderable: &Renderable<'_, R>) -> Result<()>;

	/// Draw a solid rect with the given color.
	fn fill_rect(&mut self, rect: &Rect<f32>, color: Color<f32>) -> Result<()>;

	/// Consume the context and present any outstanding draw calls.
	fn present(self) -> Result<()>;
}

pub struct Renderable<'a, R: CommonRenderer> {
	pub texture: R::Texture,
	pub shader: R::Shader,
	pub verts: &'a [Vertex],
	pub indices: &'a [u16],
}
