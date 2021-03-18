use crate::wgpu_ext::*;

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
///     let mut render_ctx /*: impl RenderContext*/ = renderer.begin_render()?;
///     render_ctx.clear(Color::RED)?;
///
///     // Change the current transform matrix, and draw a rect
///     render_ctx.set_transform(glam::Mat4::from_scale(glam::vec3(2.0, 2.0, 1.0)).into())?;
///     render_ctx.fill_rect(&Rect::new(vec2(0.0, 0.0), vec2(10.0, 10.0)), Color::GREEN)?;
///
///     render_ctx.present()?;
///     Ok(())
/// }
/// ```
pub struct WGPURenderer<Device: WGPUDevice> {
	weak_self: WGPURendererWeak<Device>,
	wgpu_device: Device,
	standard_res: StandardResources,
}

define_handles!(<WGPURenderer<T> where T: WGPUDevice>::weak_self, 
pub WGPURendererHandle<T>, pub WGPURendererWeak<T>);

impl WGPURenderer<WindowWGPUDevice> {
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
	pub fn new_from_window(window: &Window) -> Result<WGPURendererHandle<WindowWGPUDevice>> {
		let wgpu_device = WindowWGPUDevice::new(window)?;
		Self::new_from_device(wgpu_device)
	}
}

impl<Device: WGPUDevice> WGPURenderer<Device> {
	/// Get the frame dimensions as reported by the [`WGPUDevice`].
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
		self.wgpu_device.viewport_dimensions()
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
	/// let renderer = Renderer::new_from_window(&window)?;
	///
	/// let mut render_ctx = renderer.begin_render()?;
	/// render_ctx.clear(Color::RED);
	/// render_ctx.present();
	/// # Ok(()) }
	/// ```
	pub fn begin_render(&self) -> Result<impl RenderContext + '_> {
		let encoder = self.wgpu_device.with_device_info(|info| {
			Ok(info
				.device
				.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None }))
		})?;

		let target = SwapChainFrameTarget::new(self, self.dimensions());
		BufferedRenderer::new(target, encoder)
	}

	pub(crate) fn standard_res(&self) -> &StandardResources {
		&self.standard_res
	}

	pub fn wgpu_device(&self) -> &Device {
		&self.wgpu_device
	}

	/// Or the renderer can be built on top of existing WGPU contexts, to allow the simple
	/// renderer to be used on top of custom renderers.
	pub fn new_from_device(wgpu_device: Device) -> Result<WGPURendererHandle<Device>> {
		let (default_shader, white_tex) = wgpu_device.with_device_info(|info| {
			let wgsl = include_bytes!("../shaders/default.wgsl");
			let sprite_shader = WGPUShader::from_readers(
				info.device,
				std::io::Cursor::new(&wgsl[..]),
				wgpu::PrimitiveTopology::TriangleList,
			)?;

			let mut white_img = image::Image::new(1, 1);
			white_img.set_pixel([0, 0], Color::from([0xFF; 4]));
			let white_tex = WGPUTexture::from_image(
				info.device,
				info.queue,
				&white_img,
				FilterMode::Nearest,
				FilterMode::Nearest,
				TextureType::Plain,
			)?;

			Ok((sprite_shader, white_tex))
		})?;

		let standard_res = StandardResources {
			default_shader,
			white_tex,
		};

		Ok(WGPURendererHandle::new(|weak_self| Self {
			weak_self,
			wgpu_device,
			standard_res,
		}))
	}
}

#[doc(hidden)]
#[derive(Clone)]
pub struct StandardResources {
	pub(super) default_shader: WGPUShaderHandle,
	pub(super) white_tex: WGPUTextureHandle,
}
