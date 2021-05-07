use math::Vector2;

use crate::*;

/// A target which can be both rendered to and referenced as a [`Sprite`] for rendering.
///
/// # Example
///
/// ```no_run
/// # use riddle::{common::Color, image::*, platform::*, renderer::*, math::*, *};
/// # fn main() -> Result<(), RiddleError> {
/// # let rdl =  RiddleLib::new()?;
/// # let window = WindowBuilder::new().build(rdl.context())?;
/// let renderer = Renderer::new_from_window(&window)?;
///
/// let target = SpriteRenderTarget::new(&renderer, vec2(100, 100))?;
///
/// target.render(|target_ctx| {
///     target_ctx.clear(Color::BLUE)
/// })?;
///
/// renderer.render(|render_ctx| {
///     render_ctx.clear(Color::GREEN)?;
///     target.sprite().render_at(render_ctx, vec2(0.0, 0.0))
/// })?;
/// # Ok(()) }
/// ```
pub struct SpriteRenderTarget<Device: WgpuDevice> {
	renderer: Renderer<Device>,

	texture: Texture,
	sprite: Sprite<Device>,
}

impl<Device> SpriteRenderTarget<Device>
where
	Device: WgpuDevice,
{
	/// Create a new render target with the specified dimensions
	pub fn new(renderer: &Renderer<Device>, dimensions: Vector2<u32>) -> Result<Self> {
		let texture = renderer.wgpu_device().with_device_info(|info| {
			Ok(Texture::new(
				info.device,
				FilterMode::Linear,
				FilterMode::Linear,
				TextureType::RenderTarget,
				dimensions,
			))
		})?;

		let sprite = Sprite::from_texture(renderer, &texture)?;

		Ok(Self {
			renderer: renderer.clone(),

			texture,
			sprite,
		})
	}

	/// Get a render context for the current swap chain frame.
	///
	/// # Example
	///
	/// ```no_run
	/// # use riddle::{common::Color, platform::*, renderer::*, math::*, *};
	/// # fn main() -> Result<(), RiddleError> {
	/// # let rdl =  RiddleLib::new()?;
	/// # let window = WindowBuilder::new().build(rdl.context())?;
	/// # let renderer = Renderer::new_from_window(&window)?;
	/// let target = SpriteRenderTarget::new(&renderer, vec2(100, 100))?;
	///
	/// target.render(|target_ctx| {
	///     target_ctx.clear(Color::RED)
	/// })?;
	/// # Ok(()) }
	/// ```
	pub fn render<R, F>(&self, f: F) -> Result<R>
	where
		F: FnOnce(&mut BufferedRenderer<Device, &SpriteRenderTarget<Device>>) -> Result<R>,
	{
		let encoder = self.renderer.wgpu_device().with_device_info(|info| {
			Ok(info
				.device
				.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None }))
		})?;

		let mut ctx = BufferedRenderer::new(self, encoder)?;
		let result = f(&mut ctx)?;
		ctx.present()?;

		Ok(result)
	}

	/// Get the sprite which can be used to render the contents of the render target.
	pub fn sprite(&self) -> &Sprite<Device> {
		&self.sprite
	}
}

impl<Device> WgpuRenderTargetDesc<Device> for &SpriteRenderTarget<Device>
where
	Device: WgpuDevice,
{
	fn dimensions(&self) -> Vector2<f32> {
		self.sprite.dimensions()
	}

	#[inline]
	fn with_view<F: FnOnce(&wgpu::TextureView) -> Result<()>>(&self, f: F) -> Result<()> {
		let view = self
			.texture
			.internal
			.texture
			.create_view(&wgpu::TextureViewDescriptor {
				format: Some(wgpu::TextureFormat::Bgra8Unorm),
				dimension: Some(wgpu::TextureViewDimension::D2),
				aspect: wgpu::TextureAspect::All,
				..Default::default()
			});
		f(&view)
	}

	fn renderer(&self) -> &Renderer<Device> {
		&self.renderer
	}

	fn standard_resources(&self) -> &StandardResources {
		self.renderer.standard_res()
	}

	fn begin_render(&self) -> Result<()> {
		Ok(())
	}

	fn end_render(&self) {}
}
