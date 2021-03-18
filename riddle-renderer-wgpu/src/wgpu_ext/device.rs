use crate::{math::*, *};

/// A [`WGPURenderer`] compatible WGPU device.
///
/// A default implementation exists for `riddle_platform_winit::Window`
/// in [`WindowWGPUDevice`].
///
/// The application may implement this trait to layer the renderer on
/// top of custom WGPU renderer.
///
/// # Example
///
/// ```no_run
/// use std::sync::Arc;
/// use riddle::{common::Color, platform::*, renderer::{wgpu_ext::*, *}, *};
///
/// #[derive(Clone)]
/// struct ACustomRendererHandle {
///     // [..]
/// }
///
/// impl ACustomRendererHandle {
///     // [..]
/// #   fn new() -> Self { todo!() }
/// #   fn start_render(&self) { todo!() }
/// #   fn end_render(&self) { todo!() }
/// #   fn render_3d_scene(&self) { todo!() }
/// }
///
/// impl WGPUDevice for ACustomRendererHandle {
///     // [..]
/// #   fn begin_frame(&self) -> Result<(), RendererError> { todo!() }
/// #   fn end_frame(&self) { todo!() }
/// #   fn viewport_dimensions(&self) -> math::Vector2<f32>  { todo!() }
/// #   fn with_device_info<R, F: FnOnce(&WGPUDeviceInfo) -> Result<R, RendererError>>(&self, f: F) -> Result<R, RendererError> { todo!() }
/// #   fn with_frame<R, F: FnOnce(&wgpu::SwapChainFrame) -> Result<R, RendererError>>(&self, f: F) -> Result<R, RendererError> { todo!() }
/// }
///
/// fn main() -> Result<(), RiddleError> {
///     let rdl =  RiddleLib::new()?;
///     let window = WindowBuilder::new().build(rdl.context())?;
///
///     let custom_renderer = ACustomRendererHandle::new();
///
///     let renderer = WGPURenderer::new_from_device(custom_renderer.clone())?;
///
///     rdl.run(move |rdl| match rdl.event() {
///         Event::Platform(PlatformEvent::WindowClose(_)) => rdl.quit(),
///         Event::ProcessFrame => {
///             custom_renderer.start_render();
///             custom_renderer.render_3d_scene();
///
///             let mut render_ctx = renderer.begin_render().unwrap();
///             render_ctx.clear(Color::RED);
///             render_ctx.present();
///
///             custom_renderer.end_render();
///         }
///         _ => (),
///     })
/// }
/// ```
pub trait WGPUDevice {
	/// Called when the [`WGPURenderer`] begins rendering to the swap chain frame.
	///
	/// Invoked through [`WGPURenderer::begin_render`]
	fn begin_frame(&self) -> Result<()>;

	/// When the renderer is done renderering to the swap chain frame.
	///
	/// Invoked by a [`RenderContext::present`] call on the context returned from
	/// [`WGPURenderer::begin_render`].
	fn end_frame(&self);

	/// The viewport dimensions of the swapchain frame.
	///
	/// This controls the projection matrix used by the sprite renderer.
	fn viewport_dimensions(&self) -> Vector2<f32>;

	/// Provides a reference to the set of wgpu device state for use by the renderer.
	fn with_device_info<R, F: FnOnce(&WGPUDeviceInfo) -> Result<R>>(&self, f: F) -> Result<R>;

	/// Provide a reference to the current swap chain frame for use by the
	/// renderer.
	fn with_frame<R, F: FnOnce(&wgpu::SwapChainFrame) -> Result<R>>(&self, f: F) -> Result<R>;
}

pub struct WGPUDeviceInfo<'a> {
	pub device: &'a wgpu::Device,
	pub queue: &'a wgpu::Queue,
}
