//! Demonstrates how to use riddle-renderer-wgpu on top of a custom
//! wgpu based renderer.
//!
//! Main things of note:
//!
//! - Use `riddle::renderer::wgpu_ext::*` to get access to the underlying
//!   WGPU types which are generic over WGPUDevice.
//! - Implement WGPUDevice for CustomRenderer is the main piece of work
//!   needed to adapt the riddle renderer to a custom device.

use riddle::{
    common::Color,
    math::*,
    platform::*,
    renderer::{wgpu_ext::*, *},
    *,
};

use anyhow::Result;

mod custom_renderer;
use custom_renderer::*;

fn main() -> Result<()> {
    let rdl = RiddleLib::new()?;
    let mut demo = WGPURendererDemo::new(&rdl)?;

    rdl.run(move |rdl| match rdl.event() {
        Event::Platform(PlatformEvent::WindowClose(_)) => {
            rdl.quit();
        }
        Event::ProcessFrame => {
            demo.update(rdl).unwrap();
            demo.render().unwrap();
        }
        _ => (),
    });
}

struct WGPURendererDemo {
    _window: WindowHandle,

    custom_renderer: CustomRendererHandle,
    rdl_renderer: WGPURendererHandle<CustomRendererHandle>,

    label_sprite: WGPUSprite<CustomRendererHandle>,

    point_cloud: Vec<Vertex>,
    rotation: f32,
}

impl WGPURendererDemo {
    fn new(rdl: &RiddleLib) -> Result<Self> {
        let window = WindowBuilder::new()
            .title("Riddle WGPU Custom Renderer Overlay")
            .dimensions(800, 600)
            .resizeable(false)
            .build(&rdl.context())?;

        let custom_renderer = CustomRendererHandle {
            renderer: std::rc::Rc::new(std::cell::RefCell::new(CustomRenderer::new(&window)?)),
        };
        let rdl_renderer = wgpu_ext::WGPURenderer::new_from_device(custom_renderer.clone())?;

        let mut point_cloud = vec![];
        for _ in 0..300 {
            let p = glam::vec3(
                rand::random::<f32>() - 0.5,
                rand::random::<f32>() - 0.5,
                rand::random::<f32>() - 0.5,
            )
            .normalize()
                * 1.0;
            point_cloud.push(Vertex { pos: p.into() });
        }

        let font = {
            let font_bytes = include_bytes!("../../../example_assets/Roboto-Regular.ttf");
            font::TTFont::load(&font_bytes[..])?
        };
        let label = font.render_simple("Riddle Label", 24)?;
        let label_sprite = SpriteBuilder::new(label).build(&rdl_renderer)?;

        Ok(Self {
            _window: window,
            custom_renderer: custom_renderer,
            rdl_renderer,
            label_sprite,
            point_cloud,
            rotation: 0.0,
        })
    }

    fn render(&mut self) -> Result<()> {
        // Begin frame
        self.custom_renderer.renderer.borrow_mut().begin()?;

        // Draw point cloud with the custom renderer
        self.custom_renderer
            .renderer
            .borrow()
            .draw_points(&self.point_cloud, -1.5, self.rotation);

        // Use the riddle renderer to draw a label on top
        let mut render_ctx = self.rdl_renderer.begin_render()?;

        SpriteRenderArgs::new(vec2(10.0, 10.0))
            .with_color(Color::RED)
            .render(&mut render_ctx, &self.label_sprite)?;

        render_ctx.present()?;

        // End frame
        self.custom_renderer.renderer.borrow_mut().end();

        Ok(())
    }

    fn update(&mut self, rdl: &RiddleContext) -> Result<()> {
        self.rotation += 0.1 * rdl.time().delta_secs();
        Ok(())
    }
}

#[derive(Clone)]
struct CustomRendererHandle {
    renderer: std::rc::Rc<std::cell::RefCell<CustomRenderer>>,
}

impl wgpu_ext::WGPUDevice for CustomRendererHandle {
    fn begin_frame(&self) -> Result<(), RendererError> {
        self.renderer.borrow_mut().commit();
        Ok(())
    }

    fn end_frame(&self) {
        self.renderer.borrow_mut().commit();
    }

    fn viewport_dimensions(&self) -> Vector2<f32> {
        vec2(800.0, 600.0)
    }

    fn with_device_info<R, F>(&self, f: F) -> Result<R, RendererError>
    where
        F: FnOnce(&WGPUDeviceInfo) -> Result<R, RendererError>,
    {
        let renderer = self.renderer.borrow();
        let info = WGPUDeviceInfo {
            device: &renderer.device,
            queue: &renderer.queue,
        };
        f(&info)
    }

    fn with_frame<R, F>(&self, f: F) -> Result<R, RendererError>
    where
        F: FnOnce(&wgpu::SwapChainFrame) -> Result<R, RendererError>,
    {
        let renderer = self.renderer.borrow();
        f(renderer.current_frame.as_ref().unwrap())
    }
}
