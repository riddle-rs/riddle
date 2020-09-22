use crate::{ext::*, math::*, platform::*, *};

use riddle_common::define_handles;

/// A simple 2D sprite based renderer.
///
/// A renderer is created for a Window and holds a reference to the window, which will keep
/// the window alive as long as the renderer is alive
pub struct Renderer {
    weak_self: RendererWeak,

    wgpu_device: Box<dyn RendererWGPUDevice>,
    standard_res: StandardResources,
}

#[derive(Clone)]
pub struct StandardResources {
    pub(super) default_shader: ShaderHandle,
    pub(super) white_tex: TextureHandle,
}

define_handles!(<Renderer>::weak_self, pub RendererHandle, pub RendererWeak);

impl Renderer {
    pub fn new_from_window(window: &Window) -> Result<RendererHandle> {
        let wgpu_device = WindowWGPUDevice::new(window)?;
        Self::new_from_device(Box::new(wgpu_device))
    }

    fn new_from_device(wgpu_device: Box<dyn RendererWGPUDevice>) -> Result<RendererHandle> {
        let vs = include_bytes!("shaders/default.vert.spv");
        let fs = include_bytes!("shaders/default.frag.spv");
        let sprite_shader = Shader::from_readers(
            wgpu_device.device(),
            std::io::Cursor::new(&vs[..]),
            std::io::Cursor::new(&fs[..]),
            wgpu::PrimitiveTopology::TriangleList,
        )?;

        let mut white_img = image::Image::new(1, 1).map_err(|_| RendererError::Unknown)?;
        white_img.set_pixel(0, 0, Color::from([0xFF; 4]));
        let white_tex = Texture::from_image(
            wgpu_device.device(),
            wgpu_device.queue(),
            white_img,
            FilterMode::Nearest,
            FilterMode::Nearest,
            TextureType::Plain,
        )?
        .into();

        let standard_res = StandardResources {
            default_shader: sprite_shader,
            white_tex,
        };

        Ok(RendererHandle::new(|weak_self| Self {
            weak_self,
            wgpu_device,
            standard_res,
        }))
    }

    pub(crate) fn standard_res(&self) -> &StandardResources {
        &self.standard_res
    }

    pub fn dimensions(&self) -> Vector2<f32> {
        self.wgpu_device.viewport_dimensions()
    }

    pub fn begin_render<'a>(&'a self) -> Result<impl RenderContext + 'a> {
        let encoder = self
            .wgpu_device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let target = SwapChainFrameTarget::new(self, self.dimensions());
        BufferedRenderer::new(target, encoder)
    }
}

impl RendererWGPU for Renderer {
    fn wgpu_device(&self) -> &dyn RendererWGPUDevice {
        Box::as_ref(&self.wgpu_device)
    }
}
