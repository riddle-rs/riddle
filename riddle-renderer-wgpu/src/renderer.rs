use math::Vector2;
use platform::Window;

use crate::*;

/// A simple 2D sprite based renderer.
///
/// A renderer can be created for a Window and holds a reference to the window, which will
/// keep the window alive as long as the renderer is alive.
///
/// # Example
///
/// ```no_run
/// use riddle::{common::Color, platform::*, renderer::*, math::*, *};
/// fn main() -> Result<(), RiddleError> {
///     let rdl =  RiddleLib::new()?;
///     let window = WindowBuilder::new().build(rdl.context())?;
///
///     let renderer = Renderer::new_from_window(&window)?;
///
///     renderer.render(|render_ctx| {
///         render_ctx.clear(Color::RED)?;
///
///         // Change the current transform matrix, and draw a rect
///         render_ctx.set_transform(glam::Mat4::from_scale(glam::vec3(2.0, 2.0, 1.0)).into())?;
///         render_ctx.fill_rect(&Rect::new(vec2(0.0, 0.0), vec2(10.0, 10.0)), Color::GREEN)
///     })?;
///     Ok(())
/// }
/// ```
pub struct Renderer<Device: WgpuDevice> {
	pub(crate) internal: std::sync::Arc<RendererInternal<Device>>,
}

impl Renderer<WindowWgpuDevice> {
	/// Initialize a new Renderer, creating a WGPU device for the window.
	///
	/// # Example
	///
	/// ```no_run
	/// # use riddle::{common::Color, platform::*, renderer::*, math::*, *};
	/// # fn main() -> Result<(), RiddleError> {
	/// let rdl =  RiddleLib::new()?;
	/// let window = WindowBuilder::new().build(rdl.context())?;
	///
	/// let renderer = Renderer::new_from_window(&window)?;
	/// # Ok(()) }
	/// ```
	pub fn new_from_window(window: &Window) -> Result<Self> {
		let wgpu_device = WindowWgpuDevice::new(window)?;
		Self::new_from_device(wgpu_device)
	}
}

impl<Device: WgpuDevice> CommonRenderer for Renderer<Device> {
	type RenderContext = BufferedRenderer<Device, SwapChainFrameTarget<Device>>;
	type Sprite = Sprite<Device>;
	type Texture = Texture;
	type Shader = Shader;
	type SpriteFont = SpriteFont<Self>;

	fn dimensions(&self) -> Vector2<f32> {
		self.internal.wgpu_device.viewport_dimensions()
	}

	fn render<R, F>(&self, f: F) -> std::result::Result<R, RendererError>
	where
		F: FnOnce(&mut Self::RenderContext) -> std::result::Result<R, RendererError>,
	{
		let encoder = self.internal.wgpu_device.with_device_info(|info| {
			Ok(info
				.device
				.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None }))
		})?;

		let target = SwapChainFrameTarget::new(self, self.dimensions());
		let mut ctx = BufferedRenderer::new(target, encoder)?;

		let result = f(&mut ctx)?;

		ctx.present()?;

		Ok(result)
	}
}

impl<Device: WgpuDevice> Renderer<Device> {
	/// Get the frame dimensions as reported by the [`WgpuDevice`].
	///
	/// In the case of a default Window renderer, this will be the internal size of
	/// the window in logical units.
	///
	/// # Example
	///
	/// ```no_run
	/// # use riddle::{common::Color, platform::*, renderer::*, math::*, *};
	/// # fn main() -> Result<(), RiddleError> {
	/// let rdl =  RiddleLib::new()?;
	/// let window = WindowBuilder::new().dimensions(300, 400).build(rdl.context())?;
	///
	/// let renderer = Renderer::new_from_window(&window)?;
	///
	/// assert_eq!(vec2(300.0, 400.0), renderer.dimensions());
	/// # Ok(()) }
	/// ```
	pub fn dimensions(&self) -> Vector2<f32> {
		self.internal.wgpu_device.viewport_dimensions()
	}

	pub(crate) fn standard_res(&self) -> &StandardResources {
		&self.internal.standard_res
	}

	pub fn wgpu_device(&self) -> &Device {
		&self.internal.wgpu_device
	}

	/// Or the renderer can be built on top of existing WGPU contexts, to allow the simple
	/// renderer to be used on top of custom renderers.
	pub fn new_from_device(wgpu_device: Device) -> Result<Self> {
		let internal = RendererInternal::new(wgpu_device)?;
		Ok(Self {
			internal: internal.into(),
		})
	}
}

impl<D: WgpuDevice> Clone for Renderer<D> {
	fn clone(&self) -> Self {
		Self {
			internal: self.internal.clone(),
		}
	}
}

#[doc(hidden)]
#[derive(Clone)]
pub struct StandardResources {
	pub(super) default_shader: Shader,
	pub(super) white_tex: Texture,
}

pub(crate) struct RendererInternal<D: WgpuDevice> {
	wgpu_device: D,
	standard_res: StandardResources,
}

impl<D: WgpuDevice> RendererInternal<D> {
	/// Or the renderer can be built on top of existing WGPU contexts, to allow the simple
	/// renderer to be used on top of custom renderers.
	fn new(wgpu_device: D) -> Result<Self> {
		let (default_shader, white_tex) = wgpu_device.with_device_info(|info| {
			let wgsl = include_bytes!("shaders/default.wgsl");
			let sprite_shader = Shader::from_readers(
				info.device,
				std::io::Cursor::new(&wgsl[..]),
				wgpu::PrimitiveTopology::TriangleList,
			)?;

			let mut white_img = image::Image::new(1, 1);
			white_img.set_pixel([0, 0], Color::from([0xFF; 4]));
			let white_tex = Texture::from_image(
				info.device,
				info.queue,
				&white_img,
				FilterMode::Nearest,
				FilterMode::Nearest,
				TextureType::Plain,
			);

			Ok((sprite_shader, white_tex))
		})?;

		let standard_res = StandardResources {
			default_shader,
			white_tex,
		};

		Ok(Self {
			wgpu_device,
			standard_res,
		})
	}
}
